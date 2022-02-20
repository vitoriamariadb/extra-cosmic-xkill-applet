use extra_cosmic_xkill::config::Config;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_config_default() {
    let config = Config::default();

    assert!(!config.general.auto_start);
    assert!(config.general.show_notifications);
    assert!(!config.general.prefer_wayland);
    assert_eq!(config.ui.theme, "default");
    assert_eq!(config.ui.window_width, 350);
    assert_eq!(config.ui.window_height, 200);
    assert!(config.hotkey.enabled);
}

#[test]
fn test_config_serialization() {
    let config = Config::default();
    let toml_str = toml::to_string(&config).unwrap();

    assert!(toml_str.contains("auto_start"));
    assert!(toml_str.contains("show_notifications"));
    assert!(toml_str.contains("theme"));
}

#[test]
fn test_config_deserialization() {
    let toml_str = r#"
[general]
auto_start = true
show_notifications = false
prefer_wayland = true

[ui]
theme = "dark"
show_window_names = true
window_width = 400
window_height = 250

[hotkey]
enabled = false
modifiers = ["Super"]
key = "X"
"#;

    let config: Config = toml::from_str(toml_str).unwrap();

    assert!(config.general.auto_start);
    assert!(!config.general.show_notifications);
    assert!(config.general.prefer_wayland);
    assert_eq!(config.ui.theme, "dark");
    assert_eq!(config.ui.window_width, 400);
    assert!(!config.hotkey.enabled);
}

#[test]
fn test_config_save_and_load() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let original_config = Config::default();
    let toml_content = toml::to_string_pretty(&original_config).unwrap();
    fs::write(&config_path, toml_content).unwrap();

    let loaded_content = fs::read_to_string(&config_path).unwrap();
    let loaded_config: Config = toml::from_str(&loaded_content).unwrap();

    assert_eq!(
        original_config.general.auto_start,
        loaded_config.general.auto_start
    );
    assert_eq!(original_config.ui.theme, loaded_config.ui.theme);
}
