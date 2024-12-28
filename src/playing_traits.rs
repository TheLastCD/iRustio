use radiobrowser::ApiStation;

use std::io::Write;


use crate::backend::{mpv::*,gstreamer::*};
use crate::config::{StationConfigCache,StationManager};


use crate::structs::ApiStationShort;

use std::error::Error;


/*
    Play trait used to allow the playing of stations despite being 2 different datatypes
    implements the play_station function
 */
pub trait Play {
    fn play_station(&self, config: &str)-> Result<(), Box<dyn Error>>;
}



impl Play for ApiStationShort {
    fn play_station(&self, config: &str) -> Result<(), Box<dyn Error>> {
        play(&self.station_name, &self.station_url, config)
    }
}

impl Play for ApiStation {
    fn play_station(&self, config: &str) -> Result<(), Box<dyn Error>> {
        play(&self.name, &self.url, config)
    }
}


// Common play logic
fn play(name: &str, url: &str, backend: &str) -> Result<(), Box<dyn Error>> {
    println!("Playing station: {}", name);
    println!("URL: {}", url);

    
    if backend == "MPV"{
        mpv_play(url);
    }
    else {
        gst_play(url);
    }
    
    
    
    Ok(())
}




/*
    Selecting trait used to allow the selection process of stations despite being 2 different datatypes
    implements the station_select function  
*/
pub trait Selecting {
    fn station_select(&self,config:&mut StationConfigCache) -> Result<(), Box<dyn Error>>;
}

impl Selecting for Vec<ApiStationShort> {
    fn station_select(&self, config: &mut StationConfigCache) -> Result<(), Box<dyn Error>> {
        // Display stations
        for (index, station) in self.iter().enumerate() {
            println!("{}: {}", index + 1, station.station_name);
        }

        // Get user selection
        match get_user_input("Enter the number of the station to play:") {
            Ok(num) if num > 0 && num <= self.len() => {
                self[num - 1].add_recent(config);
                let _ = self[num - 1].play_station(&config.backend);
                Ok(())
            }
            _ => {
                println!("Invalid Input");
                Ok(())
            }
        }
    }
}

impl Selecting for Vec<ApiStation> {
    fn station_select(&self, config:&mut StationConfigCache) -> Result<(), Box<dyn Error>> {
        //segement for selecting stations from a given list

        for (index, station) in self.iter().enumerate() {
            println!("{}: {}", index + 1, station.name);
        }

        // Get user selection
        match get_user_input("Enter the number of the station to play:") {
            Ok(num) if num > 0 && num <= self.len() => {
                self[num - 1].add_recent(config);
                let _ = self[num - 1].play_station(&config.backend);
                Ok(())
            }
            _ => {
                println!("Invalid Input");
                Ok(())
            }
        }
    }
}


fn get_user_input(prompt: &str) -> Result<usize, Box<dyn Error>>
{
    print!("{} ", prompt);
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    input.trim().parse::<usize>().map_err(|e| e.into())
}
