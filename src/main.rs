mod api_utils;
mod config;
mod getters;
mod playing_traits;
mod structs;
mod query;  
mod preset;
mod backend;

use crate::api_utils::create_api_instance;
use crate::config::{load_or_initialize, Configurable};
use crate::getters::{
    get_countries, get_stations_by_name, get_tags, get_top_stations,
};
// use crate::structs::convert_station_2_short;
use crate::query::{generic_query,Query};

use crate::playing_traits::Selecting;

use std::error::Error;

use structs::ApiStationShort;


fn main() -> Result<(), Box<dyn Error>> {
    match create_api_instance() {
        //Pass api instance if we can successfully connect
        Ok(api) => {
            let api_ref = &api; //create pointer reference, probably unecassery
            
            // load or create our config, this stores certain station data ie presets, this increases speed from user perspective
            let mut config = load_or_initialize()?; 
            
            //grab the station presets from the config file and 
            // let preset_stations = config.station_presets;
            loop {
                println!("Select an option:");
                println!("1. Select preset station");
                println!("2. Show recently played stations");
                println!("3. Search station by name");
                println!("4. Search tags");
                println!("5. Search top 100 stations");
                println!("6. Search country");

                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let input = input.trim();

                match input {
                    "1" => {
                        // let _ = config.station_presets.station_select(&mut config)?;
                        let presets = config.station_presets.clone();
                        presets.station_select(&mut config)?;
                    }
                    "2" => {

                        let limit = 10;
                        let start_index = config.recents.len().saturating_sub(limit);
                        let recent_slice: Vec<ApiStationShort> = config.recents[start_index..].to_vec();
                        let _ = recent_slice.station_select(&mut config);
                    }

                    "3" =>{
                        let stations = get_stations_by_name(api_ref, &generic_query())?;
                        let _ = stations.station_select(&mut config);
                    }
                    "4" => {
                        let tags = get_tags(api_ref, "30")?;
                        let _ = tags.category_query(api_ref,&mut config);

                    }
                    "5" => {
                        let stations = get_top_stations(api_ref)?;
                        let _ = stations.station_select(&mut config);
                    }
                    "6" => {
                        let countries = get_countries(api_ref, "30")?;
                        let _ = countries.category_query(api_ref, &mut config);

                    }

                    "7" => {
                        config.save();
                        break},
                    _ => println!("Invalid option"),
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
