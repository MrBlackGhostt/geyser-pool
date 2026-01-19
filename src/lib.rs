use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, Result,
};
use serde::Deserialize;
use std::fs::{File, OpenOptions, read_to_string};
use std::io::Write;
use std::sync::Mutex;

#[derive(Deserialize)]
struct PluginConfig {
    log_path: String,
}

#[derive(Default, Debug)]
pub struct LearningPlugin {
    pub file_path: String,
    pub file: Option<Mutex<File>>, //Option tell it may or may not have a file
}

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

                let log_entry = format!(
                    "Slot: {} | Pubkey: {:?} | Data Len: {}\n",
                    slot,
                    account_info.pubkey,
                    account_info.data.len()
                );

                if let Some(mutex) = &self.file {
                    //here check the self.file has something some init if it it borrow call it mutex and if it is empty (None) skip this block
                    let mut file = mutex.lock().unwrap();
                    file.write_all(log_entry.as_bytes())
                        .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;
                }
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
