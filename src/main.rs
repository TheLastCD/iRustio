mod api_utils;
mod config;
mod getters;
mod playing_traits;
mod structs;

use crate::api_utils::create_api_instance;
use crate::config::load_or_initialize;
use crate::getters::{
    get_countries, get_stations_by_name, get_stations_by_tag, get_tags, get_top_stations,
};

use crate::playing_traits::Selecting;

use std::error::Error;

fn generic_query() -> String {
    // not elegant, needs better handling of return within the confines of rust
    let mut station_query = String::new();
    println!("Enter the station name:");
    std::io::stdin()
        .read_line(&mut station_query)
        .expect("Failed to read input");

    let station_query = station_query.trim();
    station_query.to_string()
}

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
                        preset_stations.station_select();
                    }
                    "2" => {
                        let stations = get_stations_by_name(api_ref, &generic_query())?;
                        stations.station_select();
                    }
                    "3" => {
                        let tags = get_tags(api_ref, "30")?;

                        // Print tags with their indices
                        for (index, tag) in tags.iter().enumerate() {
                            println!("{}: {}", index + 1, tag.name);
                        }

                        println!("Enter the number of the tag you want to search:");

                        // Read user input
                        let mut input = String::new();
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read input");

                        // Parse input and handle potential errors
                        match input.trim().parse::<usize>() {
                            Ok(num) if num > 0 && num <= tags.len() => {
                                let stations = get_stations_by_tag(api_ref, &tags[num - 1].name)?;
                                stations.station_select();
                            }
                            _ => {
                                println!("Invalid input");
                            }
                        }
                    }
                    "4" => {
                        let stations = get_top_stations(api_ref)?;
                        stations.station_select();
                    }
                    "5" => {
                        let countries = get_countries(api_ref, "30")?;
                        //let test: Box<dyn Processable> = Box::new(countries);
                        // Print tags with their indices
                        //for (index, tag) in countries.iter().enumerate() {
                        //    println!("{}: {}", index + 1, tag.name);
                        //}

                        //println!("Enter the number of the tag you want to search:");

                        // Read user input
                        //let mut input = String::new();
                        //std::io::stdin()
                        //    .read_line(&mut input)
                        //    .expect("Failed to read input");

                        // Parse input and handle potential errors
                        //match input.trim().parse::<usize>() {
                        //    Ok(num) if num > 0 && num <= countries.len() => {
                        //        let stations =
                        //            get_stations_by_country(api_ref, &countries[num - 1].name)?;
                        //        station_select(stations);
                        //    }
                        //    _ => {
                        //        println!("Invalid input");
                        //    }
                        //}
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
