use serde::{Deserialize, Serialize};


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







