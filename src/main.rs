use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
use rand::Rng;

// Structure to hold radio station information
#[derive(Debug)]
struct RadioStation {
    name: String,
    url: String,
}

fn load_stations(file_path: &str) -> Vec<RadioStation> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut stations = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.starts_with('#') || line.is_empty() || line.starts_with("//") {
            // println!("{}",line);
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            //println!("{}", parts[1]);
            continue;
        }
        let name = parts[1].replace('~', "").replace('-', " ");
        let url = parts[0].to_string();
        stations.push(RadioStation { name, url });
    }

    stations
}

fn random_station(stations: &Vec<RadioStation>) -> &RadioStation {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..stations.len());
    &stations[index]
}

fn play_station(station: &RadioStation) {
    println!("Playing station: {}", station.name);
    println!("URL: {}", station.url);
    let _ = Command::new("mpv")
        .arg(station.url.clone())
        .spawn()
        .expect("Failed to spawn mpv process");
}

fn main() {
    let stations_file = "/Users/charlie/git/iradio-rust/src/stations.txt"; // Replace with your actual file path
    let stations = load_stations(stations_file);

    loop {
        println!("Select an option:");
        println!("1. Play a random station");
        println!("2. Select a station");
        println!("3. Quit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        match input {
            "1" => {
                let station = random_station(&stations);
                play_station(station);
            }
            "2" => {
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
            "3" => break,
            _ => println!("Invalid option"),
        }
    }
}
