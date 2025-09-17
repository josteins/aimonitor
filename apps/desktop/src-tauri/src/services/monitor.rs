use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use tokio::time::{Duration, interval};
use tracing::{info, error};

use crate::db::Database;
use crate::models::{Provider, ProviderUsage};
use crate::services::{AppState, keychain::KeychainService};

pub struct MonitorService {
    state: Arc<AppState>,
}

impl MonitorService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn start_polling(&self) {
        let state = Arc::clone(&self.state);

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(60));

            loop {
                ticker.tick().await;

                let providers = state.providers.read().await;

                for provider in providers.iter() {
                    if !provider.enabled {
                        continue;
                    }

                    if let Err(e) = Self::poll_provider(&state, provider).await {
                        error!("Failed to poll provider {}: {}", provider.name, e);
                    }
                }
            }
        });
    }

    async fn poll_provider(state: &AppState, provider: &Provider) -> Result<()> {
        let api_key = KeychainService::get_api_key(&provider.id)?;

        let adapter = AppState::get_provider_adapter(&provider.provider_type);
        let usage = adapter.get_current_usage(&api_key).await?;

        let now = Utc::now();
        let start_of_month = now.date_naive()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        let metrics = adapter.fetch_usage(&api_key, start_of_month, now).await?;

        for metric in metrics {
            state.db.insert_metric(&metric).await?;
        }

        info!(
            "Polled {}: {} tokens today, ${:.2} MTD",
            provider.name, usage.today_tokens, usage.mtd_cost
        );

        Ok(())
    }

    pub async fn get_all_usage(&self) -> Result<Vec<ProviderUsage>> {
        let mut all_usage = Vec::new();
        let providers = self.state.providers.read().await;

        for provider in providers.iter() {
            if !provider.enabled {
                continue;
            }

            match self.get_provider_usage(provider).await {
                Ok(usage) => all_usage.push(usage),
                Err(e) => error!("Failed to get usage for {}: {}", provider.name, e),
            }
        }

        Ok(all_usage)
    }

    async fn get_provider_usage(&self, provider: &Provider) -> Result<ProviderUsage> {
        let api_key = KeychainService::get_api_key(&provider.id)?;
        let adapter = AppState::get_provider_adapter(&provider.provider_type);
        adapter.get_current_usage(&api_key).await
    }
}