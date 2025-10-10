use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub ui: UIConfig,
    pub hotkey: HotkeyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub auto_start: bool,
    pub show_notifications: bool,
    pub prefer_wayland: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: String,
    pub show_window_names: bool,
    pub window_width: i32,
    pub window_height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub enabled: bool,
    pub modifiers: Vec<String>,
    pub key: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                auto_start: false,
                show_notifications: true,
                prefer_wayland: false,
            },
            ui: UIConfig {
                theme: "default".to_string(),
                show_window_names: true,
                window_width: 350,
                window_height: 200,
            },
            hotkey: HotkeyConfig {
                enabled: true,
                modifiers: vec!["Ctrl".to_string(), "Alt".to_string()],
                key: "K".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content =
                fs::read_to_string(&config_path).context("Falha ao ler arquivo de configuração")?;
            let config: Config =
                toml::from_str(&content).context("Falha ao parsear configuração")?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Falha ao criar diretório de configuração")?;
        }

        let content = toml::to_string_pretty(self).context("Falha ao serializar configuração")?;
        fs::write(&config_path, content).context("Falha ao salvar configuração")?;

        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Falha ao obter diretório de configuração")?;
        Ok(config_dir.join("extra-cosmic-xkill").join("config.toml"))
    }

    #[allow(dead_code)]
    pub fn get_config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Falha ao obter diretório de configuração")?;
        Ok(config_dir.join("extra-cosmic-xkill"))
    }
}

#[derive(Default)]
#[allow(dead_code)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn auto_start(mut self, value: bool) -> Self {
        self.config.general.auto_start = value;
        self
    }

    pub fn show_notifications(mut self, value: bool) -> Self {
        self.config.general.show_notifications = value;
        self
    }

    pub fn prefer_wayland(mut self, value: bool) -> Self {
        self.config.general.prefer_wayland = value;
        self
    }

    pub fn theme(mut self, theme: impl Into<String>) -> Self {
        self.config.ui.theme = theme.into();
        self
    }

    pub fn window_size(mut self, width: i32, height: i32) -> Self {
        self.config.ui.window_width = width;
        self.config.ui.window_height = height;
        self
    }

    pub fn hotkey(mut self, enabled: bool, modifiers: Vec<String>, key: impl Into<String>) -> Self {
        self.config.hotkey.enabled = enabled;
        self.config.hotkey.modifiers = modifiers;
        self.config.hotkey.key = key.into();
        self
    }

    pub fn validate(&self) -> Result<()> {
        if self.config.ui.window_width < 100 || self.config.ui.window_width > 3840 {
            anyhow::bail!("Largura da janela deve estar entre 100 e 3840");
        }
        if self.config.ui.window_height < 100 || self.config.ui.window_height > 2160 {
            anyhow::bail!("Altura da janela deve estar entre 100 e 2160");
        }
        if self.config.hotkey.enabled && self.config.hotkey.key.is_empty() {
            anyhow::bail!("Tecla de atalho nao pode ser vazia quando habilitada");
        }
        let valid_themes = ["default", "dark", "light", "dracula"];
        if !valid_themes.contains(&self.config.ui.theme.as_str()) {
            anyhow::bail!(
                "Tema invalido: {}. Opcoes: {:?}",
                self.config.ui.theme,
                valid_themes
            );
        }
        Ok(())
    }

    pub fn build(self) -> Result<Config> {
        self.validate()?;
        Ok(self.config)
    }
}

// "Nao e porque as coisas sao dificeis que nao ousamos; e porque nao ousamos que sao dificeis." - Seneca
