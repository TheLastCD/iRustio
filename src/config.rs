use radiobrowser::ApiStation;

mod statics;

use crate::config::statics::*;
use crate::api_utils::create_api_instance;
use crate::getters::get_presets;
use crate::structs::ApiStationShort;
use chrono::Utc;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};





// /*
// Structs and implementations of said structs
// */


#[derive(Serialize, Deserialize, Debug)]
pub struct StationConfigCache {
    date: String,
    pub backend: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub station_presets: Vec<ApiStationShort>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recents: Vec<ApiStationShort>,   
}

 
//Establish default for StationConfigCache if no config.toml exists

impl Default for StationConfigCache {
    fn default() -> Self {
        StationConfigCache {
            date: Utc::now().to_string(),
            station_presets: default_preset_return().expect("Failed to retrieve streams"),
            recents: default_preset_return().expect("Failed to retrieve streams") , // set default favourites as recents. this avoid some toml stuff with empty recents
            backend: "MPV".to_string(),
        }
    }
}




pub trait StationManager {
    fn add_recent(&self, config: &mut StationConfigCache);
    fn convert_to_short(&self, date: &str) -> ApiStationShort;
}

pub trait Configurable {
    fn update(&mut self, incoming: &ApiStationShort);
    fn save(&self);
}



impl StationManager for ApiStation {
    fn convert_to_short(&self, date: &str) -> ApiStationShort {
        ApiStationShort {
            station_name: self.name.clone(),
            station_url: self.url.clone(),
            station_icon: self.favicon.clone(),
            last_checked: date.to_string(),
        }
    }

    fn add_recent(&self, config: &mut StationConfigCache) {
        let date = Utc::now().to_string();
        config.update(&self.convert_to_short(&date));
        config.save();
    }
}

impl StationManager for ApiStationShort {
    fn convert_to_short(&self, date: &str) -> ApiStationShort {
        ApiStationShort {
            station_name: self.station_name.clone(),
            station_url: self.station_url.clone(),
            station_icon: self.station_icon.clone(),
            last_checked: date.to_string(),
        }
    }

    fn add_recent(&self, config: &mut StationConfigCache) {
        let date = Utc::now().to_string();
        config.update(&self.convert_to_short(&date));
        config.save();
    }
}


impl Configurable for StationConfigCache {
    fn update(&mut self, incoming: &ApiStationShort) {
        self.recents = update_recents(500, &mut self.recents, incoming);
    }

    fn save(&self) {
        let config_path = Path::new(CONFIG_NAME);
        if let Err(e) = config_write(config_path, self) {
            println!("Failed to write file: {}", e);
        }
    }
}








// /* 
// Private Functions
// */


fn convert_stations_2_short(stations: &Vec<ApiStation>, date: &str) -> Vec<ApiStationShort>{
    let mut station_list: Vec<ApiStationShort> = Vec::new();
    for station in stations {
        station_list.push(station.convert_to_short(date)); 
    }
    station_list

}
fn default_preset_return() -> Result<Vec<ApiStationShort>, Box<dyn std::error::Error>> {
    match create_api_instance() {
        Ok(tmp_api) => {

            let defaults = &get_presets(&tmp_api, &DEFAULT_PRESETS)?;
            let station_list = convert_stations_2_short
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






fn config_write(config_path: &Path, config: &StationConfigCache) -> Result<(), Box<dyn std::error::Error>> {
    
    let toml_string = toml::to_string(&config)?;
    fs::write(config_path, toml_string)?;
    Ok(())
    
}


fn update_recents(limit: usize, cache: &mut Vec<ApiStationShort>, addition: &ApiStationShort)-> Vec<ApiStationShort>{
    let combined_len = cache.len() + 1;
    if combined_len > limit{
        print!("removing first recents");
        let to_pop = combined_len - limit;
        cache.drain(..to_pop);
        
    }
    
    cache.push(addition.clone());
    cache.to_vec()
}




// /*
// Public functions
// */

pub fn load_or_initialize() -> Result<StationConfigCache, Box<dyn std::error::Error>> {
    let station_path = Path::new(CONFIG_NAME);
    
    if station_path.exists() {

        let content = fs::read_to_string(station_path);
        let config: StationConfigCache = toml::from_str(&content.unwrap())?;
        Ok(config)
    } else {

        let config = StationConfigCache::default();
        
        let write_result = config_write(station_path, &config);
        if let Err(e) = write_result {
            println!("Failed to write file: {}", e);
        }

        Ok(config)
    }

}


