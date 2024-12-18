

use std::process::Command;


use radiobrowser::blocking::RadioBrowserAPI;
use radiobrowser::{ApiStation, ApiTag };
use radiobrowser::{StationOrder,TagOrder};
use std::error::Error;

fn create_api_instance() -> Result<RadioBrowserAPI, Box<dyn Error>>{
    //create api client instance here and pass through to relevent functions
    //really just to prevent creating too many clients and the server responding with a 429 error
    Ok(RadioBrowserAPI::new()?)
}
 
fn get_top_stations(api: &RadioBrowserAPI) -> Result<Vec<ApiStation>, Box<dyn Error>>{
    

    let stations = api.get_stations()
        .limit("100")
        .order(StationOrder::Clickcount)
        .send()?;

    Ok(stations)
}


fn get_stations_by_name(api: &RadioBrowserAPI, query: &String) -> Result<Vec<ApiStation>, Box<dyn Error>>{


    let stations = api.get_stations()
        .name(query)
        .limit("100") // arbitary just here incase query is too generic
        .order(StationOrder::Votes)
        .send()?;

    Ok(stations)
}



fn get_presets(api: &RadioBrowserAPI, presets: &[&str]) -> Result<Vec<ApiStation>, Box<dyn std::error::Error>> {

    let mut stations: Vec<ApiStation> = Vec::with_capacity(presets.len());
    for &preset in presets{
        let station = api.get_stations()
        .name(preset)
        .name_exact(true)
        .order(StationOrder::Clickcount)
        .send()?;
        stations.push(station[0].clone());
    }


    Ok(stations)
}


fn get_stations_by_tag(api: &RadioBrowserAPI, query: &String) -> Result<Vec<ApiStation>, Box<dyn Error>>{


    let stations = api.get_stations()
        .tag(query)
        .limit("100") // arbitary just here incase query is too generic
        .order(StationOrder::Votes)
        .send()?;

    Ok(stations)
}

fn get_tags(api: &RadioBrowserAPI, limit: &str) -> Result<Vec<ApiTag>, Box<dyn Error>>{
    let tags = api.get_tags()
        .order(TagOrder::StationCount)
        .reverse(true)
        .limit("30") // replace with dynamic 
        .send()?;

    Ok(tags)
    
}


fn station_select(stations: Vec<ApiStation>){
    //segement for selecting stations from a given list

    for (index, station) in stations.iter().enumerate() {
        println!("{}: {}", index + 1, station.name);
    }
    println!("Enter the number of the station to play:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().parse::<usize>().unwrap_or(0);
    if input > 0 && input <= stations.len() {
        play_station(&stations[input - 1]);
    } else {
        println!("Invalid input");
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

fn generic_query()-> String{
    // not elegant, needs better handling of return within the confines of rust
    let mut station_query = String::new();
    println!("Enter the station name:");
    std::io::stdin().read_line(&mut station_query)
        .expect("Failed to read input");

    let station_query = station_query.trim();
    station_query.to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let preset_list = vec![
        "NTS Radio 1",
        "BBC Radio 1",
        "Naim Radio [lossless flac]",
        "Capital FM London",
        "Radio X"
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
        
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read input");
                let input = input.trim();
        
                match input {
                    "1" => {
                        
                        station_select(preset_stations.clone());
                    }
                    "2" => {

                        let stations = get_stations_by_name(api_ref, &generic_query())?;
                        station_select(stations);
                    
                    }
                    "3" =>{
                        let tags = get_tags(api_ref);
                        
                        
                    } 
                    "4"=> break,
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

