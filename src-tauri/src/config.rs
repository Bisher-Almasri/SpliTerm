use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub font: Option<Font>,
    pub theme: Option<Theme>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Font {
    pub font_family: Option<String>,
    pub font_size: Option<i8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
pub enum CursorStyle {
    #[default]
    Bar,
    Underline,
    Block,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Theme {
    pub theme_name: Option<String>,
    pub theme: Option<ThemeConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ThemeConfig {
    pub background: Option<String>,
    pub foreground: Option<String>,

    pub transparency: Option<bool>,

    pub cursor: Option<String>,
    pub cursor_accent: Option<String>,
    pub cursor_style: Option<CursorStyle>,
    pub cursor_blink: Option<bool>,

    pub selection_background: Option<String>,
    pub selection_foreground: Option<String>,

    pub black: Option<String>,
    pub red: Option<String>,
    pub green: Option<String>,
    pub yellow: Option<String>,
    pub blue: Option<String>,
    pub magenta: Option<String>,
    pub cyan: Option<String>,
    pub white: Option<String>,

    pub bright_black: Option<String>,
    pub bright_red: Option<String>,
    pub bright_green: Option<String>,
    pub bright_yellow: Option<String>,
    pub bright_blue: Option<String>,
    pub bright_magenta: Option<String>,
    pub bright_cyan: Option<String>,
    pub bright_white: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            font: Some(Font {
                font_family: Some("monospaced".to_string()),
                font_size: Some(16),
            }),
            theme: Some(Theme::default()),
        }
    }
}

impl AppConfig {
    fn file_path(app: &AppHandle) -> Result<PathBuf, String> {
        let config = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to resolve config dir: {}", e))?;

        fs::create_dir_all(&config).map_err(|e| format!("Failed to create config dir: {}", e))?;

        Ok(config.join("config.toml"))
    }

    pub fn load_or_create(app: &AppHandle) -> Result<Self, String> {
        let path = Self::file_path(app)?;

        if !path.exists() {
            let default_config = Self::default();
            return Ok(default_config);
        }

        let content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read config file: {}", e))?;

        let mut config: Self =
            toml::from_str(&content).map_err(|e| format!("Failed to parse config.toml: {}", e))?;

        let has_theme_name: bool = config
            .theme
            .as_ref()
            .and_then(|t| t.theme_name.as_ref())
            .is_some();

        if has_theme_name {
            if let Some(theme) = &mut config.theme {
                if let Some(name) = &theme.theme_name {
                    theme.theme = Some(Self::get_theme(app, name.clone())?);
                }
            }
        }

        Ok(config)
    }

    fn get_theme(app: &AppHandle, name: String) -> Result<ThemeConfig, String> {
        let config = app.path().app_config_dir().map_err(|e| e.to_string())?;

        let theme_dir = config.join("theme/");

        let theme_path = theme_dir.join(format!("{}.toml", name));

        if !theme_path.exists() {
            return Err(format!("Theme '{}' not found", name));
        }

        let content = fs::read_to_string(&theme_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&content).map_err(|e| format!("Failed to parse config.toml: {}", e))
    }
}
