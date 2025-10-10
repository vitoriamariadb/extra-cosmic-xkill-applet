use extra_cosmic_xkill::config::{Config, ConfigBuilder};
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

#[test]
fn test_config_builder_default() {
    let config = ConfigBuilder::new().build().unwrap();
    assert!(!config.general.auto_start);
    assert!(config.general.show_notifications);
    assert_eq!(config.ui.theme, "default");
}

#[test]
fn test_config_builder_custom() {
    let config = ConfigBuilder::new()
        .auto_start(true)
        .theme("dark")
        .window_size(800, 600)
        .hotkey(true, vec!["Super".to_string()], "X")
        .build()
        .unwrap();

    assert!(config.general.auto_start);
    assert_eq!(config.ui.theme, "dark");
    assert_eq!(config.ui.window_width, 800);
    assert_eq!(config.hotkey.key, "X");
}

#[test]
fn test_config_builder_invalid_width() {
    let result = ConfigBuilder::new().window_size(50, 200).build();
    assert!(result.is_err());
}

#[test]
fn test_config_builder_invalid_theme() {
    let result = ConfigBuilder::new().theme("neon").build();
    assert!(result.is_err());
}

#[test]
fn test_config_builder_empty_hotkey() {
    let result = ConfigBuilder::new().hotkey(true, vec![], "").build();
    assert!(result.is_err());
}

// "A verdadeira sabedoria esta em reconhecer a propria ignorancia." - Socrates
