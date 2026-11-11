use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    font_family: String,
    font_size: i8,
}

impl Default for AppConfig {
  fn default() -> Self {
      Self {
          font_family: "monospaced".to_string(),
          font_size: 16,
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