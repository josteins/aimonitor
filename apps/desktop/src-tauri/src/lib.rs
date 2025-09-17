mod models;

use serde::{Deserialize, Serialize};

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
    // Mock implementation for initial setup
    println!("Adding provider: {} - {:?}", request.name, request.provider_type);
    Ok(())
}

#[tauri::command]
async fn get_usage() -> Result<ProviderUsageResponse, String> {
    // Mock implementation for initial setup
    Ok(ProviderUsageResponse { providers: vec![] })
}

#[tauri::command]
async fn remove_provider(provider_id: String) -> Result<(), String> {
    // Mock implementation for initial setup
    println!("Removing provider: {}", provider_id);
    Ok(())
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