pub mod monitor;
pub mod keychain;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::db::Database;
use crate::providers::{ProviderAdapter, openai::OpenAIAdapter, anthropic::AnthropicAdapter, openrouter::OpenRouterAdapter};
use crate::models::{Provider, ProviderType};

pub struct AppState {
    pub db: Arc<Database>,
    pub providers: Arc<RwLock<Vec<Provider>>>,
}

impl AppState {
    pub async fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
            providers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn get_provider_adapter(provider_type: &ProviderType) -> Box<dyn ProviderAdapter> {
        match provider_type {
            ProviderType::OpenAI => Box::new(OpenAIAdapter::new()),
            ProviderType::Anthropic => Box::new(AnthropicAdapter::new()),
            ProviderType::OpenRouter => Box::new(OpenRouterAdapter::new()),
        }
    }
}