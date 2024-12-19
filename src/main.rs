mod getters;
use crate::getters::{
    get_countries, get_presets, get_stations_by_country, get_stations_by_name, get_stations_by_tag,
    get_tags, get_top_stations,
};

use std::process::Command;
use std::string;

use radiobrowser::blocking::RadioBrowserAPI;
use radiobrowser::{ApiCountry, ApiStation, ApiTag};

use std::error::Error;

fn create_api_instance() -> Result<RadioBrowserAPI, Box<dyn Error>> {
    //create api client instance here and pass through to relevent functions
    //really just to prevent creating too many clients and the server responding with a 429 error
    Ok(RadioBrowserAPI::new()?)
}

fn station_select(stations: Vec<ApiStation>) {
    //segement for selecting stations from a given list

    for (index, station) in stations.iter().enumerate() {
        println!("{}: {}", index + 1, station.name);
    }
    println!("Enter the number of the station to play:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= stations.len() => {
            play_station(&stations[num - 1]);
        }
        _ => {
            print!("Invalid Input")
        }
    }
}

fn play_station(station: &ApiStation) {
    //mpv player function, accepts
    println!("Playing station: {}", station.name);
    println!("URL: {}", station.url);
    let _ = Command::new("mpv")
        .arg(station.url.clone())
        .spawn()
        .expect("Failed to spawn mpv process");
}

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

enum QueryEntity {
    Tag(Vec<ApiTag>),
    Country(Vec<ApiCountry>),
}

impl QueryEntity {
    fn args_query(&self) {
        let mut test = false;
        match self {
            QueryEntity::Tag(tag) => {
                let data = tag;
            }
            QueryEntity::Country(country) => {
                test = true;
                let data = country;
            }
            _ => {
                let data: Vec<ApiTag> = Vec::new();
            }
        }
        println!("hello");
        for (index, tag) in data.iter().enumerate() {
            println!("{}: {}", index + 1, tag.name);
        }
    }
}

fn args_query2(data: Vec<ApiTag>) -> Result<(), Box<dyn Error>> {
    // Print tags with their indices

    for (index, tag) in data.iter().enumerate() {
        println!("{}: {}", index + 1, tag.name);
    }

    println!("Enter the number you want to search:");

    // Read user input
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse input and handle potential errors
    match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= data.len() => {
            let stations = get_stations_by_country(api, &data[num - 1].name)?;
            station_select(stations);
        }
        _ => {
            println!("Invalid input");
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let preset_list = vec![
        "NTS Radio 1",
        "BBC Radio 1",
        "Radio Paradise",
        "Capital FM London",
        "Radio X",
    ];

    match create_api_instance() {
        //Pass api instance if we can successfully connect
        Ok(api) => {
            let api_ref = &api; //create pointer reference, probably unecassery
            let preset_stations = get_presets(api_ref, &preset_list)?;
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
                        station_select(preset_stations.clone());
                    }
                    "2" => {
                        let stations = get_stations_by_name(api_ref, &generic_query())?;
                        station_select(stations);
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
                                station_select(stations);
                            }
                            _ => {
                                println!("Invalid input");
                            }
                        }
                    }
                    "4" => {
                        let stations = get_top_stations(api_ref)?;
                        station_select(stations);
                    }
                    "5" => {
                        let countries = get_countries(api_ref, "30")?;
                        let test: Box<dyn Processable> = Box::new(countries);
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
