pub mod openai;
pub mod anthropic;
pub mod openrouter;

use async_trait::async_trait;
use anyhow::Result;
use crate::models::{Metric, ProviderUsage};
use chrono::{DateTime, Utc};

#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    async fn fetch_usage(&self, api_key: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<Metric>>;
    async fn fetch_balance(&self, api_key: &str) -> Result<Option<f64>>;
    async fn get_current_usage(&self, api_key: &str) -> Result<ProviderUsage>;
}