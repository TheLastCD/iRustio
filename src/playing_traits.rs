use radiobrowser::ApiStation;

use std::io::Write;


use crate::config::{StationConfigCache, ConfigCycle};
use crate::backend::mpv::mpv_play;
use crate::structs::convert_station_2_short;
use crate::structs::ApiStationShort;
use std::process::Command;
use std::error::Error;
use chrono::Utc;

/*
    Play trait used to allow the playing of stations despite being 2 different datatypes
    implements the play_station function
 */
pub trait Play {
    fn play_station(&self)-> Result<(), Box<dyn Error>>;
}



impl Play for ApiStationShort {
    fn play_station(&self) -> Result<(), Box<dyn Error>> {
        play("mpv", &self.station_name, &self.station_url)
    }
}

impl Play for ApiStation {
    fn play_station(&self) -> Result<(), Box<dyn Error>> {
        play("mpv", &self.name, &self.url)
    }
}


// Common play logic
fn play(player: &str, name: &str, url: &str) -> Result<(), Box<dyn Error>> {
    println!("Playing station: {}", name);
    println!("URL: {}", url);

    // let mut instance = Command::new(player)
    //     .arg(url)
    //     .spawn()
    //     .expect("Failed to spawn mpv process");

    // wait_for_child(&mut instance)?;

    let mut instance = mpv_play(url);
    
    Ok(())
}




/*
    Selecting trait used to allow the selection process of stations despite being 2 different datatypes
    implements the station_select function  
*/
pub trait Selecting {
    fn station_select(&self) -> Result<(), Box<dyn Error>>;
}

impl Selecting for Vec<ApiStationShort> {
    fn station_select(&self) -> Result<(), Box<dyn Error>> {
        // Display stations
        for (index, station) in self.iter().enumerate() {
            println!("{}: {}", index + 1, station.station_name);
        }

        // Get user selection
        match get_user_input("Enter the number of the station to play:") {
            Ok(num) if num > 0 && num <= self.len() => {
                let _ = self[num - 1].play_station();
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
    fn station_select(&self) -> Result<(), Box<dyn Error>> {
        //segement for selecting stations from a given list

        for (index, station) in self.iter().enumerate() {
            println!("{}: {}", index + 1, station.name);
        }

        // Get user selection
        match get_user_input("Enter the number of the station to play:") {
            Ok(num) if num > 0 && num <= self.len() => {
                // add_recent(config, vec![self[num - 1]]);
                let _ = self[num - 1].play_station();
                Ok(())
            }
            _ => {
                println!("Invalid Input");
                Ok(())
            }
        }
    }
}

fn wait_for_child(child: &mut std::process::Child) -> Result<std::process::ExitStatus, Box<dyn Error>> {
    let status = child.wait()?;
    
    if status.success() {
        Ok(status)
    } else {
        Err(format!("Child process exited with status: {}", status).into())
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

fn add_recent(config:&mut StationConfigCache, stations: Vec<ApiStation>) -> &mut StationConfigCache
{
    config.update(
        &convert_station_2_short(
            &stations,
            &Utc::now().to_string()
        )
    );
    config.save();
    config

}