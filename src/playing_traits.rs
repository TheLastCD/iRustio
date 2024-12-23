use radiobrowser::ApiStation;

use crate::structs::ApiStationShort;
use std::process::Command;

pub trait Play {
    fn play_station(&self);
}

impl Play for ApiStationShort {
    fn play_station(&self) {
        //mpv player function, accepts
        println!("Playing station: {}", self.station_name);
        println!("URL: {}", self.station_url);
        let _ = Command::new("mpv")
            .arg(self.station_url.clone())
            .spawn()
            .expect("Failed to spawn mpv process");
    }
}

impl Play for ApiStation {
    fn play_station(&self) {
        //mpv player function, accepts
        println!("Playing station: {}", self.name);
        println!("URL: {}", self.url);

        let _ = Command::new("mpv")
            .arg(self.url.clone())
            .spawn()
            .expect("Failed to spawn mpv process");
    }
}

pub trait Selecting {
    fn station_select(&self);
}

impl Selecting for Vec<ApiStationShort> {
    fn station_select(&self) {
        //segement for selecting stations from a given list

        for (index, station) in self.iter().enumerate() {
            println!("{}: {}", index + 1, station.station_name);
        }
        println!("Enter the number of the station to play:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= self.len() => {
                self[num - 1].play_station();
            }
            _ => {
                print!("Invalid Input")
            }
        }
    }
}

impl Selecting for Vec<ApiStation> {
    fn station_select(&self) {
        //segement for selecting stations from a given list

        for (index, station) in self.iter().enumerate() {
            println!("{}: {}", index + 1, station.name);
        }
        println!("Enter the number of the station to play:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= self.len() => {
                self[num - 1].play_station();
            }
            _ => {
                print!("Invalid Input")
            }
        }
    }
}
