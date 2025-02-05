

pub mod api_utils;
pub mod config;
pub mod getters;
pub mod playing_traits;
pub mod structs;
pub mod query;
pub mod preset;
pub mod backend;

// Re-export commonly used items for easier access
pub use api_utils::create_api_instance;
pub use config::{load_or_initialize, Configurable};
pub use getters::{get_countries, get_stations_by_name, get_tags, get_top_stations};
pub use query::{generic_query, Query};
pub use playing_traits::Selecting;
pub use structs::ApiStationShort;

// You can add any library-wide functions or constants here
// For example:

pub fn initialize() {
    // Any initialization code for your library
    println!("Initializing library...");
}

// If you have any common error types or results, you can define them here
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
