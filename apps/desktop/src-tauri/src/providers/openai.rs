use async_trait::async_trait;
use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::{Metric, MetricType, Provider, ProviderType, ProviderUsage};
use super::ProviderAdapter;

pub struct OpenAIAdapter {
    client: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct UsageResponse {
    data: Vec<UsageData>,
    daily_costs: Option<Vec<DailyCost>>,
}

#[derive(Debug, Deserialize)]
struct UsageData {
    aggregation_timestamp: i64,
    n_requests: u32,
    operation: String,
    snapshot_id: String,
    n_context_tokens_total: u64,
    n_generated_tokens_total: u64,
    n_cached_context_tokens_total: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct DailyCost {
    timestamp: i64,
    line_items: Vec<LineItem>,
}

#[derive(Debug, Deserialize)]
struct LineItem {
    name: String,
    cost: f64,
}

impl OpenAIAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }
}

#[async_trait]
impl ProviderAdapter for OpenAIAdapter {
    async fn fetch_usage(&self, api_key: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<Metric>> {
        let url = format!("{}/usage", self.base_url);

        let params = HashMap::from([
            ("date", from.format("%Y-%m-%d").to_string()),
        ]);

        let response = self.client
            .get(&url)
            .bearer_auth(api_key)
            .query(&params)
            .send()
            .await?
            .json::<UsageResponse>()
            .await?;

        let mut metrics = Vec::new();

        for usage in response.data {
            let timestamp = DateTime::from_timestamp(usage.aggregation_timestamp, 0)
                .unwrap_or_else(|| Utc::now());

            metrics.push(Metric {
                id: uuid::Uuid::new_v4().to_string(),
                provider_id: "openai".to_string(),
                metric_type: MetricType::TokensIn,
                value: usage.n_context_tokens_total as f64,
                unit: "tokens".to_string(),
                timestamp,
                dimensions: HashMap::from([
                    ("operation".to_string(), usage.operation.clone()),
                ]),
            });

            metrics.push(Metric {
                id: uuid::Uuid::new_v4().to_string(),
                provider_id: "openai".to_string(),
                metric_type: MetricType::TokensOut,
                value: usage.n_generated_tokens_total as f64,
                unit: "tokens".to_string(),
                timestamp,
                dimensions: HashMap::from([
                    ("operation".to_string(), usage.operation.clone()),
                ]),
            });

            if let Some(cached) = usage.n_cached_context_tokens_total {
                metrics.push(Metric {
                    id: uuid::Uuid::new_v4().to_string(),
                    provider_id: "openai".to_string(),
                    metric_type: MetricType::TokensCached,
                    value: cached as f64,
                    unit: "tokens".to_string(),
                    timestamp,
                    dimensions: HashMap::from([
                        ("operation".to_string(), usage.operation.clone()),
                    ]),
                });
            }
        }

        if let Some(daily_costs) = response.daily_costs {
            for cost in daily_costs {
                let timestamp = DateTime::from_timestamp(cost.timestamp, 0)
                    .unwrap_or_else(|| Utc::now());

                let total_cost: f64 = cost.line_items.iter().map(|item| item.cost).sum();

                metrics.push(Metric {
                    id: uuid::Uuid::new_v4().to_string(),
                    provider_id: "openai".to_string(),
                    metric_type: MetricType::CostUsd,
                    value: total_cost,
                    unit: "usd".to_string(),
                    timestamp,
                    dimensions: HashMap::new(),
                });
            }
        }

        Ok(metrics)
    }

    async fn fetch_balance(&self, _api_key: &str) -> Result<Option<f64>> {
        Ok(None)
    }

    async fn get_current_usage(&self, api_key: &str) -> Result<ProviderUsage> {
        let now = Utc::now();
        let start_of_day = now.date_naive().and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();
        let start_of_month = now.date_naive()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        let metrics = self.fetch_usage(api_key, start_of_month, now).await?;

        let today_tokens: u64 = metrics.iter()
            .filter(|m| m.timestamp >= start_of_day)
            .filter(|m| matches!(m.metric_type, MetricType::TokensIn | MetricType::TokensOut))
            .map(|m| m.value as u64)
            .sum();

        let today_cost: f64 = metrics.iter()
            .filter(|m| m.timestamp >= start_of_day)
            .filter(|m| matches!(m.metric_type, MetricType::CostUsd))
            .map(|m| m.value)
            .sum();

        let mtd_tokens: u64 = metrics.iter()
            .filter(|m| matches!(m.metric_type, MetricType::TokensIn | MetricType::TokensOut))
            .map(|m| m.value as u64)
            .sum();

        let mtd_cost: f64 = metrics.iter()
            .filter(|m| matches!(m.metric_type, MetricType::CostUsd))
            .map(|m| m.value)
            .sum();

        Ok(ProviderUsage {
            provider: Provider {
                id: "openai".to_string(),
                name: "OpenAI".to_string(),
                provider_type: ProviderType::OpenAI,
                api_key_ref: Some("openai_key".to_string()),
                enabled: true,
                created_at: Utc::now(),
            },
            today_tokens,
            today_cost,
            mtd_tokens,
            mtd_cost,
            balance: None,
            credits: None,
            budget_used_percentage: None,
        })
    }
}