use serde::{Deserialize, Serialize};
use radiobrowser::ApiStation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiStationShort {
    /*  a shorter version of APIStation from radiobrowser::ApiStation, 
        used in config files to cache on necassery station elelments
     */
    pub station_name: String,
    pub station_url: String,
    pub station_icon: String,
    pub last_checked: String,
}




pub fn convert_station_2_short(station: &Vec<ApiStation>, date: &str) -> Vec<ApiStationShort>{
    let mut station_list: Vec<ApiStationShort> = Vec::new();
    for station in station {
        station_list.push(ApiStationShort {
            station_name: station.name.clone(),
            station_url: station.url.clone(),
            station_icon: station.favicon.clone(),
            last_checked: date.to_string(),
        });
    }
    station_list

}

