use async_trait::async_trait;
use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::{Metric, MetricType, Provider, ProviderType, ProviderUsage};
use super::ProviderAdapter;

pub struct OpenRouterAdapter {
    client: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct KeyInfo {
    data: KeyData,
}

#[derive(Debug, Deserialize)]
struct KeyData {
    limit: Option<f64>,
    usage: f64,
    limit_remaining: Option<f64>,
    is_free_tier: bool,
}

#[derive(Debug, Deserialize)]
struct CreditsInfo {
    total_credits: f64,
    used_credits: f64,
    remaining_credits: f64,
}

impl OpenRouterAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://openrouter.ai".to_string(),
        }
    }
}

#[async_trait]
impl ProviderAdapter for OpenRouterAdapter {
    async fn fetch_usage(&self, api_key: &str, _from: DateTime<Utc>, _to: DateTime<Utc>) -> Result<Vec<Metric>> {
        let mut metrics = Vec::new();

        let credits_url = format!("{}/api/v1/credits", self.base_url);
        let credits_response = self.client
            .get(&credits_url)
            .bearer_auth(api_key)
            .send()
            .await?
            .json::<CreditsInfo>()
            .await?;

        metrics.push(Metric {
            id: uuid::Uuid::new_v4().to_string(),
            provider_id: "openrouter".to_string(),
            metric_type: MetricType::CreditsRemaining,
            value: credits_response.remaining_credits,
            unit: "credits".to_string(),
            timestamp: Utc::now(),
            dimensions: HashMap::new(),
        });

        metrics.push(Metric {
            id: uuid::Uuid::new_v4().to_string(),
            provider_id: "openrouter".to_string(),
            metric_type: MetricType::CostUsd,
            value: credits_response.used_credits,
            unit: "credits".to_string(),
            timestamp: Utc::now(),
            dimensions: HashMap::new(),
        });

        Ok(metrics)
    }

    async fn fetch_balance(&self, api_key: &str) -> Result<Option<f64>> {
        let key_url = format!("{}/api/v1/key", self.base_url);
        let key_response = self.client
            .get(&key_url)
            .bearer_auth(api_key)
            .send()
            .await?
            .json::<KeyInfo>()
            .await?;

        Ok(key_response.data.limit_remaining)
    }

    async fn get_current_usage(&self, api_key: &str) -> Result<ProviderUsage> {
        let key_url = format!("{}/api/v1/key", self.base_url);
        let key_response = self.client
            .get(&key_url)
            .bearer_auth(api_key)
            .send()
            .await?
            .json::<KeyInfo>()
            .await?;

        let credits_url = format!("{}/api/v1/credits", self.base_url);
        let credits_response = self.client
            .get(&credits_url)
            .bearer_auth(api_key)
            .send()
            .await?
            .json::<CreditsInfo>()
            .await?;

        let budget_used_percentage = if let Some(limit) = key_response.data.limit {
            Some((key_response.data.usage / limit) * 100.0)
        } else {
            None
        };

        Ok(ProviderUsage {
            provider: Provider {
                id: "openrouter".to_string(),
                name: "OpenRouter".to_string(),
                provider_type: ProviderType::OpenRouter,
                api_key_ref: Some("openrouter_key".to_string()),
                enabled: true,
                created_at: Utc::now(),
            },
            today_tokens: 0,
            today_cost: 0.0,
            mtd_tokens: 0,
            mtd_cost: credits_response.used_credits,
            balance: key_response.data.limit_remaining,
            credits: Some(credits_response.remaining_credits),
            budget_used_percentage,
        })
    }
}