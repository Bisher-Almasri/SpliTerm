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
    pub background: Option<String>,
    pub foreground: Option<String>,

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
          font: Some(Font { font_family: Some("monospaced".to_string()), font_size: Some(16) }),
          theme: Some(Theme::default())
      }
  }
}

impl AppConfig {
    fn file_path(app: &AppHandle) -> Result<PathBuf, String> {
      let config = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve config dir: {}", e))?;

      fs::create_dir_all(&config)
        .map_err(|e| format!("Failed to create config dir: {}", e))?;

      Ok(config.join("config.toml"))
    }

    pub fn load_or_create(app: &AppHandle) -> Result<Self, String> {
      let path = Self::file_path(app)?;

      if !path.exists() {
        let default_config = Self::default();
        return Ok(default_config);
      }

      let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

      toml::from_str(&content)
        .map_err(|e| format!("Failed to parse config.toml: {}", e))
    }
}