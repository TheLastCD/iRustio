use crate::api_utils::create_api_instance;
use crate::getters::get_presets;
use crate::structs::{ApiStationShort,convert_station_2_short};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StationConfigCache {
    date: String,
    pub station_presets: Vec<ApiStationShort>,
    pub recents: Option<Vec<ApiStationShort>>,
}


// default list of presets that iRustio comes with:
static DEFAULT_PRESETS: [&str; 5] = [
    "NTS Radio 1",
    "BBC Radio 1",
    "Radio Paradise",
    "Capital FM London",
    "Radio X",
];

/* 

*/
fn default_preset_return() -> Result<Vec<ApiStationShort>, Box<dyn std::error::Error>> {
    match create_api_instance() {
        Ok(tmp_api) => {
            let api_ref = &tmp_api;
            let defaults = &get_presets(&api_ref, &DEFAULT_PRESETS)?;
            let station_list = convert_station_2_short
                (
                    defaults, 
                    &Utc::now().to_string(),
                );

            return Ok(station_list); //force the return
        }

        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let null_return = vec![ApiStationShort {
        station_name: "null".to_string(),
        station_url: "null".to_string(),
        station_icon: "null".to_string(),
        last_checked: "null".to_string(),
    }]; // if no response return null TODO: handle this properly

    Ok(null_return)
}

/* 
    Establish default for StationConfigCache if no config.toml exists
*/
impl Default for StationConfigCache {
    fn default() -> Self {
        StationConfigCache {
            date: Utc::now().to_string(),
            station_presets: default_preset_return().expect("Failed to retrieve streams"),
            recents: None, //no recents by default
        }
    }
}

use std::fs;
use std::path::Path;

static CONFIG_NAME: &str = "Config.toml";

fn config_write(config_path: &Path, config: &StationConfigCache) -> Result<(), Box<dyn std::error::Error>> {
    
    let toml_string = toml::to_string(&config)?;
    fs::write(config_path, toml_string)?;
    Ok(())
    
}

pub fn load_or_initialize() -> Result<StationConfigCache, Box<dyn std::error::Error>> {
    let config_path = Path::new(CONFIG_NAME);

    if config_path.exists() {
        let content = fs::read_to_string(config_path);
        let config: StationConfigCache = toml::from_str(&content.unwrap())?;
        Ok(config)
    } else {
        let config = StationConfigCache::default();
        let _ = config_write(&config_path, &config);
        Ok(config)
    }
}



// pub fn update() -> Result<StationConfigCache, Box<dyn std::error::Error>> {

// }
