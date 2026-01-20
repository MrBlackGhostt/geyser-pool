use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoV3, ReplicaAccountInfoVersions,
};
use geyser_basic::LearningPlugin;
use solana_program::pubkey::Pubkey;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

#[test]
fn test_integration() {
    let _ = fs::remove_file("dummy_log.txt"); //what does _ mean
    let _ = fs::remove_file("dummy_config.json");

    let dummy_path = "dummy_config.json";

    let dummy_config = r#"{
    "log_path": "dummy_log.txt",
    "target_owner": "11111111111111111111111111111111"
}"#;
    let mut dummy_config_file = File::create("dummy_config.json").unwrap();

    dummy_config_file
        .write_all(dummy_config.as_bytes())
        .unwrap();

    let mut plugin = LearningPlugin::default();
    println!("Initialize the plugin");

    plugin.on_load(dummy_path, false).unwrap();
    println!("load the plugin and create the dummy_log.txt");

    let fake_pub = Pubkey::new_unique().to_bytes();
    let fake_owner = Pubkey::new_unique().to_bytes();
    let fake_data = vec![1, 2, 2, 3, 4];

    let fake_pub_2 = Pubkey::from_str("11111111111111111111111111111111")
        .unwrap()
        .to_bytes();
    let fake_owner_2 = Pubkey::from_str("11111111111111111111111111111111")
        .unwrap()
        .to_bytes();
    let fake_data_2 = vec![1, 2, 2, 3, 4, 4, 5, 5];

    let fake_acc = ReplicaAccountInfoV3 {
        pubkey: &fake_pub,
        lamports: 1000,
        owner: &fake_owner,
        executable: false,
        rent_epoch: 0,
        data: &fake_data,
        write_version: 1,
        txn: None,
    };

    let fake_acc2 = ReplicaAccountInfoV3 {
        pubkey: &fake_pub_2.as_ref(),
        lamports: 1000,
        owner: &fake_owner_2.as_ref(),
        executable: false,
        rent_epoch: 0,
        data: &fake_data_2,
        write_version: 1,
        txn: None,
    };
    let account_version = ReplicaAccountInfoVersions::V0_0_3(&fake_acc);
    let account_version2 = ReplicaAccountInfoVersions::V0_0_3(&fake_acc2);

    plugin
        .update_account(account_version, 00088888, false)
        .unwrap();
    plugin
        .update_account(account_version2, 000888882222, false)
        .unwrap();

    println!("--- Simulation Complete ---");
    println!("Check 'simulation_log.txt' to see the results!");

    let read_logs = fs::read_to_string("dummy_log.txt").unwrap();
    println!("The LOGS ARE {}", read_logs);

    assert!(read_logs.contains("\"slot\":888882222"));
}
