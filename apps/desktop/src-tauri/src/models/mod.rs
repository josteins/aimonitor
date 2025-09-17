use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub api_key_ref: Option<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    OpenRouter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub id: String,
    pub provider_id: String,
    pub metric_type: MetricType,
    pub value: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub dimensions: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricType {
    TokensIn,
    TokensOut,
    TokensCached,
    CostUsd,
    CreditsRemaining,
    Balance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub provider_id: String,
    pub period: BudgetPeriod,
    pub soft_limit: Option<f64>,
    pub hard_limit: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetPeriod {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub provider_id: String,
    pub rule: AlertRule,
    pub last_fired_at: Option<DateTime<Utc>>,
    pub status: AlertStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertRule {
    SpendThreshold { amount: f64, is_soft: bool },
    CreditThreshold { amount: f64, is_soft: bool },
    ProjectedRunOut { days_before: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    Active,
    Triggered,
    Snoozed,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderUsage {
    pub provider: Provider,
    pub today_tokens: u64,
    pub today_cost: f64,
    pub mtd_tokens: u64,
    pub mtd_cost: f64,
    pub balance: Option<f64>,
    pub credits: Option<f64>,
    pub budget_used_percentage: Option<f64>,
}