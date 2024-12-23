mod api_utils;
mod config;
mod getters;
mod playing_traits;
mod structs;
mod query;  

use crate::api_utils::create_api_instance;
use crate::config::load_or_initialize;
use crate::getters::{
    get_countries, get_stations_by_name, get_tags, get_top_stations,
};
use crate::query::{generic_query,Query};

use crate::playing_traits::Selecting;

use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    match create_api_instance() {
        //Pass api instance if we can successfully connect
        Ok(api) => {
            let api_ref = &api; //create pointer reference, probably unecassery
                                //let preset_stations = get_presets(api_ref, &preset_list)?;
            let config = load_or_initialize();
            let preset_stations = config.unwrap().station_presets;
            loop {
                println!("Select an option:");
                println!("1. Select preset station");
                println!("2. Search station by name");
                println!("3. Search tags");
                println!("4. Search top 100 stations");
                println!("5. Search country");

                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let input = input.trim();

                match input {
                    "1" => {
                        let _ = preset_stations.station_select();
                    }
                    "2" => {
                        let stations = get_stations_by_name(api_ref, &generic_query())?;
                        let _ = stations.station_select();
                    }
                    "3" => {
                        let tags = get_tags(api_ref, "30")?;
                        let _ = tags.category_query(api_ref);

                    }
                    "4" => {
                        let stations = get_top_stations(api_ref)?;
                        let _ = stations.station_select();
                    }
                    "5" => {
                        let countries = get_countries(api_ref, "30")?;
                        let _ = countries.category_query(api_ref);

                    }

                    "6" => break,
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
