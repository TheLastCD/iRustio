use serde::{Deserialize, Serialize};
//TODO Refactor to have all structs here :)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiStationShort {
    pub station_name: String,
    pub station_url: String,
    pub last_checked: String,
}
