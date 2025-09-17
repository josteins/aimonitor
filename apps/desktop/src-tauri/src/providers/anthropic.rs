use async_trait::async_trait;
use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::{Metric, MetricType, Provider, ProviderType, ProviderUsage};
use super::ProviderAdapter;

pub struct AnthropicAdapter {
    client: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct UsageReport {
    usage: Vec<UsageItem>,
}

#[derive(Debug, Deserialize)]
struct UsageItem {
    input_tokens: u64,
    output_tokens: u64,
    input_cached_tokens: Option<u64>,
    model: String,
    timestamp: String,
}

#[derive(Debug, Deserialize)]
struct CostReport {
    costs: Vec<CostItem>,
}

#[derive(Debug, Deserialize)]
struct CostItem {
    amount: f64,
    currency: String,
    timestamp: String,
}

impl AnthropicAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }
}

#[async_trait]
impl ProviderAdapter for AnthropicAdapter {
    async fn fetch_usage(&self, api_key: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<Metric>> {
        let mut metrics = Vec::new();

        let usage_url = format!("{}/organizations/usage_report/messages", self.base_url);
        let params = HashMap::from([
            ("start_date", from.format("%Y-%m-%d").to_string()),
            ("end_date", to.format("%Y-%m-%d").to_string()),
        ]);

        let usage_response = self.client
            .get(&usage_url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .query(&params)
            .send()
            .await?
            .json::<UsageReport>()
            .await?;

        for item in usage_response.usage {
            let timestamp = DateTime::parse_from_rfc3339(&item.timestamp)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            metrics.push(Metric {
                id: uuid::Uuid::new_v4().to_string(),
                provider_id: "anthropic".to_string(),
                metric_type: MetricType::TokensIn,
                value: item.input_tokens as f64,
                unit: "tokens".to_string(),
                timestamp,
                dimensions: HashMap::from([
                    ("model".to_string(), item.model.clone()),
                ]),
            });

            metrics.push(Metric {
                id: uuid::Uuid::new_v4().to_string(),
                provider_id: "anthropic".to_string(),
                metric_type: MetricType::TokensOut,
                value: item.output_tokens as f64,
                unit: "tokens".to_string(),
                timestamp,
                dimensions: HashMap::from([
                    ("model".to_string(), item.model.clone()),
                ]),
            });

            if let Some(cached) = item.input_cached_tokens {
                metrics.push(Metric {
                    id: uuid::Uuid::new_v4().to_string(),
                    provider_id: "anthropic".to_string(),
                    metric_type: MetricType::TokensCached,
                    value: cached as f64,
                    unit: "tokens".to_string(),
                    timestamp,
                    dimensions: HashMap::from([
                        ("model".to_string(), item.model.clone()),
                    ]),
                });
            }
        }

        let cost_url = format!("{}/organizations/cost_report", self.base_url);
        let cost_response = self.client
            .get(&cost_url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .query(&params)
            .send()
            .await?
            .json::<CostReport>()
            .await?;

        for item in cost_response.costs {
            let timestamp = DateTime::parse_from_rfc3339(&item.timestamp)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            metrics.push(Metric {
                id: uuid::Uuid::new_v4().to_string(),
                provider_id: "anthropic".to_string(),
                metric_type: MetricType::CostUsd,
                value: item.amount,
                unit: item.currency.to_lowercase(),
                timestamp,
                dimensions: HashMap::new(),
            });
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
                id: "anthropic".to_string(),
                name: "Anthropic".to_string(),
                provider_type: ProviderType::Anthropic,
                api_key_ref: Some("anthropic_key".to_string()),
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