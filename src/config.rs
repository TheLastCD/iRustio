use crate::api_utils::create_api_instance;
use crate::getters::get_presets;
use crate::structs::{ApiStationShort,convert_station_2_short};
use chrono::Utc;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};


/* 
Static variables for config and cache file names and Default preset list.
*/
static CONFIG_NAME: &str = "Config.toml";
// static STATION_CACHE: &str = "Station_cache.toml";

// default list of presets that iRustio comes with:
static DEFAULT_PRESETS: [&str; 5] = [
    "NTS Radio 1",
    "BBC Radio 1",
    "Radio Paradise",
    "Capital FM London",
    "Radio X",
];


/*
Structs and implementations of said structs
*/
#[derive(Serialize,Deserialize,Debug)]
pub struct RadioConfig{
    backend: String,
}

impl Default for RadioConfig{
    fn default() -> Self {
        RadioConfig{
            backend: "MPV".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StationConfigCache {
    date: String,
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
        }
    }
}
pub trait ConfigCycle{
    fn update(&mut self, incoming: &Vec<ApiStationShort>);
    fn save(&self);
}

// -> Result<StationConfigCache, Box<dyn std::error::Error>> 
impl ConfigCycle for StationConfigCache{
    fn update(&mut self,incoming: &Vec<ApiStationShort>){
        self.recents = update_recents(500, &mut self.recents, incoming);
    }
    fn save(&self){
        let config_path = Path::new(CONFIG_NAME);
        let write_result = config_write(&config_path, &self);
        match write_result {
            Err(e) => println!("Failed to write file: {}", e),
            Ok(_) => (),
        }
    }

}




/* 
Private Functions
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






fn config_write(config_path: &Path, config: &StationConfigCache) -> Result<(), Box<dyn std::error::Error>> {
    
    let toml_string = toml::to_string(&config)?;
    fs::write(config_path, toml_string)?;
    Ok(())
    
}

fn update_recents(limit: usize, cache: &mut Vec<ApiStationShort>, addition: &Vec<ApiStationShort>)-> Vec<ApiStationShort>{
    let combined_len = cache.len() + addition.len();
    if combined_len > limit{
        print!("removing first recents");
        let to_pop = combined_len - limit;
        cache.drain(..to_pop);
        
    }
    
    cache.extend_from_slice(addition);
    cache.to_vec()
}




/*
Public functions
*/

pub fn load_or_initialize() -> Result<StationConfigCache, Box<dyn std::error::Error>> {
    let config_path = Path::new(CONFIG_NAME);
    

    if config_path.exists() {

        let content = fs::read_to_string(config_path);
        let config: StationConfigCache = toml::from_str(&content.unwrap())?;
        Ok(config)
    } else {

        let config = StationConfigCache::default();
        
        let write_result = config_write(&config_path, &config);
        match write_result {
            Err(e) => println!("Failed to write file: {}", e),
            Ok(_) => (),
        }

        Ok(config)
    }
}






