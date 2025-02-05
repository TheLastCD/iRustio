use radiobrowser::blocking::RadioBrowserAPI;
use radiobrowser::{ApiCountry, ApiTag};
use crate::config::StationConfigCache;
use crate::getters::{get_stations_by_country,get_stations_by_tag};
use crate::playing_traits::Selecting;
use std::error::Error;

pub fn generic_query() -> String {
    // not elegant, needs better handling of return within the confines of rust
    let mut station_query = String::new();
    println!("Enter the station name:");
    std::io::stdin()
        .read_line(&mut station_query)
        .expect("Failed to read input");

    let station_query = station_query.trim();
    station_query.to_string()
}

pub trait Query {
    fn category_query(&self, api_ref: &RadioBrowserAPI, config: &mut StationConfigCache) -> Result<(), Box<dyn Error>>;
}

impl Query for Vec<ApiCountry>{
    fn category_query(&self, api_ref: &RadioBrowserAPI, config: &mut StationConfigCache) -> Result<(), Box<dyn Error>>{
        // Print tags with their indices
        for (index, tag) in self.iter().enumerate() {
           println!("{}: {}", index + 1, tag.name);
        }

        println!("Enter the number of the tag you want to search:");

        //Read user input
        let mut input = String::new();
        std::io::stdin()
           .read_line(&mut input)
           .expect("Failed to read input");

        //Parse input and handle potential errors
        match input.trim().parse::<usize>() {
           Ok(num) if num > 0 && num <= self.len() => {
               let stations =
                   get_stations_by_country(api_ref, &self[num - 1].name)?;
                   let _ = stations.station_select(config);
               Ok(())
        }
           _ => {
               println!("Invalid input");
               Ok(())
        }
        }
    }
}

impl Query for Vec<ApiTag>{
    fn category_query(&self, api_ref: &RadioBrowserAPI, config: &mut StationConfigCache) -> Result<(), Box<dyn Error>>{
        // Print tags with their indices
        for (index, tag) in self.iter().enumerate() {
           println!("{}: {}", index + 1, tag.name);
        }

        println!("Enter the number of the tag you want to search:");

        //Read user input
        let mut input = String::new();
        std::io::stdin()
           .read_line(&mut input)
           .expect("Failed to read input");

        //Parse input and handle potential errors
        match input.trim().parse::<usize>() {
           Ok(num) if num > 0 && num <= self.len() => {
               let stations =
                   get_stations_by_tag(api_ref, &self[num - 1].name)?;
                   let _ = stations.station_select(config);
               Ok(())
        }
           _ => {
               println!("Invalid input");
               Ok(())
        }
        }
    }
}