use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, Result,
};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize)]
pub struct TokenAccountInfo {
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct PluginConfig {
    pub log_path: String,
    pub target_owner: String,
}

#[derive(Serialize)]
pub struct LogEntry {
    pub slot: u64,
    pub pubkey: String,
    pub owner: String,
    pub data_len: usize,
    pub decode_data: Option<TokenAccountInfo>,
}
#[derive(Default, Debug)]
pub struct LearningPlugin {
    pub target_owner: Pubkey,
    pub file_path: String,
    pub file: Option<Mutex<File>>, //Option tell it may or may not have a file
}
impl LearningPlugin {
    pub fn should_log(&self, owner: &[u8]) -> bool {
        self.target_owner.as_ref() == owner
    }
    pub fn write_entry(&self, entry: &LogEntry) -> Result<()> {
        // Move the JSON serialization and file writing logic here
        // Note: You'll need to handle the errors and return Result<()>
        // ...LogEntry
        let json_bytes =
            serde_json::to_vec(&entry).map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;
        if let Some(mutex) = &self.file {
            //here check the self.file has something some init if it it borrow call it mutex and if it is empty (None) skip this block
            let mut file = mutex.lock().unwrap();
            file.write_all(&json_bytes)
                .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;
            file.write_all(b",\n")
                .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;
        };
        Ok(())
    }
}
