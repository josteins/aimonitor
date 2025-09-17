use anyhow::{Result, anyhow};
use keyring::Entry;

const SERVICE_NAME: &str = "AIMonitor";

pub struct KeychainService;

impl KeychainService {
    pub fn store_api_key(provider: &str, key: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, provider)?;
        entry.set_password(key)?;
        Ok(())
    }

    pub fn get_api_key(provider: &str) -> Result<String> {
        let entry = Entry::new(SERVICE_NAME, provider)?;
        entry.get_password()
            .map_err(|e| anyhow!("Failed to retrieve API key: {}", e))
    }

    pub fn delete_api_key(provider: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, provider)?;
        entry.delete_credential()?;
        Ok(())
    }

    pub fn has_api_key(provider: &str) -> bool {
        Entry::new(SERVICE_NAME, provider)
            .ok()
            .and_then(|entry| entry.get_password().ok())
            .is_some()
    }
}