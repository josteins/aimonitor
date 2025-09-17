mod models;

use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

// In-memory storage for providers (temporary solution)
static PROVIDERS: OnceLock<Mutex<HashMap<String, models::ProviderUsage>>> = OnceLock::new();

#[derive(Serialize, Deserialize)]
struct AddProviderRequest {
    provider_type: models::ProviderType,
    api_key: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct ProviderUsageResponse {
    providers: Vec<models::ProviderUsage>,
}

#[tauri::command]
async fn add_provider(request: AddProviderRequest) -> Result<(), String> {
    println!("Adding provider: {} - {:?}", request.name, request.provider_type);

    let provider_id = format!("{:?}_{}", request.provider_type, uuid::Uuid::new_v4());

    let provider = models::Provider {
        id: provider_id.clone(),
        name: request.name,
        provider_type: request.provider_type,
        api_key_ref: Some(provider_id.clone()),
        enabled: true,
        created_at: chrono::Utc::now(),
    };

    let usage = models::ProviderUsage {
        provider,
        today_tokens: 0,
        today_cost: 0.0,
        mtd_tokens: 0,
        mtd_cost: 0.0,
        balance: None,
        credits: None,
        budget_used_percentage: None,
    };

    let providers_map = PROVIDERS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut providers = providers_map.lock().unwrap();
    providers.insert(provider_id, usage);

    println!("Provider stored successfully. Total providers: {}", providers.len());
    Ok(())
}

#[tauri::command]
async fn get_usage() -> Result<ProviderUsageResponse, String> {
    let providers_map = PROVIDERS.get_or_init(|| Mutex::new(HashMap::new()));
    let providers = providers_map.lock().unwrap();
    let provider_list: Vec<models::ProviderUsage> = providers.values().cloned().collect();

    println!("get_usage called, returning {} providers", provider_list.len());
    Ok(ProviderUsageResponse { providers: provider_list })
}

#[tauri::command]
async fn remove_provider(provider_id: String) -> Result<(), String> {
    let providers_map = PROVIDERS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut providers = providers_map.lock().unwrap();

    if providers.remove(&provider_id).is_some() {
        println!("Removed provider: {}", provider_id);
        Ok(())
    } else {
        Err(format!("Provider {} not found", provider_id))
    }
}

#[tauri::command]
async fn toggle_provider(provider_id: String, enabled: bool) -> Result<(), String> {
    // Mock implementation for initial setup
    println!("Toggling provider {} to {}", provider_id, enabled);
    Ok(())
}

// Tray functionality temporarily disabled for initial setup

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // Initialize app state here
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_provider,
            get_usage,
            remove_provider,
            toggle_provider
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}