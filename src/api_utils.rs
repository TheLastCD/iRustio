use radiobrowser::blocking::RadioBrowserAPI;
use std::error::Error;
pub fn create_api_instance() -> Result<RadioBrowserAPI, Box<dyn Error>> {
    //create api client instance here and pass through to relevent functions
    //really just to prevent creating too many clients and the server responding with a 429 error
    Ok(RadioBrowserAPI::new()?)
}
