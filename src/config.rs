use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub app_id: String,
    pub update_interval: u64,
    pub large_image: String,
    pub small_image: String
}

impl Default for Config {
	fn default() -> Self {
		Self {
			app_id: "1513925936150479078".to_string(),
			update_interval: 60,
			large_image: "big_image".to_string(),
			small_image: "small_image".to_string(),
		}
	}
}

// Get the config path
fn config_path() -> PathBuf {
	dirs::config_dir()
    	.expect("[!] Could not find config directory")
    	.join("sys2dis.toml")
}

// Load the config
pub fn load_or_create_config() -> Config {
    let path = config_path();

    if path.exists() {
    	let contents = fs::read_to_string(&path)
        	.expect("[!] Could not read config file");

    	toml::from_str(&contents).unwrap_or_else(|e| {
     		eprintln!("[!] Invalid config. Will use default: {}", e);
       		Config::default()
     	})
    } else {
    	let default_config = Config::default();

     	if let Some(parent) = path.parent() {
      		fs::create_dir_all(parent)
      			.expect("[!] Could not create config directory");
      	}

    	fs::write(&path, toml::to_string_pretty(&default_config).unwrap())
    		.expect("[!] Could not write config file");

    	default_config
    }
}