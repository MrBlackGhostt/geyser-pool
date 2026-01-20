use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, Result,
};

mod state;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use std::convert::TryFrom;
use std::fs::{File, OpenOptions, read_to_string};
use std::io::Write;
use std::str::FromStr;
use std::sync::Mutex;

use crate::state::{LearningPlugin, LogEntry, PluginConfig};

impl GeyserPlugin for LearningPlugin {
    fn name(&self) -> &'static str {
        "LearningGeyserPlugin"
    }

    fn on_load(&mut self, config_file: &str, _is_reload: bool) -> Result<()> {
        // GO to the file (config_file) & read the content and make the json into the string (config_path)
        let config_path =
            read_to_string(config_file).map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;

        // Deserialize the string (config_path) into the struct (PluginConfig)
        let config: PluginConfig = serde_json::from_str(&config_path)
            .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;

        // Set the file path to the log path from the config
        self.file_path = config.log_path;
        self.target_owner = Pubkey::from_str(&config.target_owner)
            .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;

        self.file = Some(Mutex::new(file)); // the file wrapping in the mutex for the safe lock and putting that lock in the option box(that why use the Some);
        Ok(())
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: solana_program::clock::Slot,
        _is_startup: bool,
    ) -> Result<()> {
        match account {
            ReplicaAccountInfoVersions::V0_0_3(account_info) => {
                //Serialize the data  to a simple string
                if self.should_log(account_info.owner) {
                    let entry = LogEntry {
                        slot,
                        pubkey: Pubkey::try_from(account_info.pubkey).unwrap().to_string(),
                        owner: Pubkey::try_from(account_info.owner).unwrap().to_string(),
                        data_len: account_info.data.len(),
                    };
                    // let json_bytes = serde_json::to_vec(&entry)
                    //     .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;

                    self.write_entry(&entry)?;
                };
            }
            _ => {}
        }
        Ok(())
    }
}
#[unsafe(no_mangle)] // this will not allow to change the name of the fun
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = LearningPlugin::default();
    let boxed: Box<dyn GeyserPlugin> = Box::new(plugin); //This puts your plugin on the "Heap" so it lives longer than the function call.
    Box::into_raw(boxed) //This tells Rust: "Do not delete this memory when the function ends. Hand the pointer to the validator." The validator then "owns" that memory and will call your methods through that pointer
}
