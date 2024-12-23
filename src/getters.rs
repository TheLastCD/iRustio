use radiobrowser::blocking::RadioBrowserAPI;
use radiobrowser::{ApiCountry, ApiStation, ApiTag};
use radiobrowser::{CountryOrder, StationOrder, TagOrder};
use std::error::Error;

pub fn get_top_stations(api: &RadioBrowserAPI) -> Result<Vec<ApiStation>, Box<dyn Error>> {
    let stations = api
        .get_stations()
        .limit("100")
        .order(StationOrder::Clickcount)
        .reverse(true)
        .send()?;

    Ok(stations)
}

pub fn get_stations_by_name(
    api: &RadioBrowserAPI,
    query: &String,
) -> Result<Vec<ApiStation>, Box<dyn Error>> {
    let stations = api
        .get_stations()
        .name(query)
        .limit("100") // arbitary just here incase query is too generic
        .order(StationOrder::Votes)
        .reverse(true)
        .send()?;

    Ok(stations)
}

pub fn get_presets(
    api: &RadioBrowserAPI,
    presets: &[&str],
) -> Result<Vec<ApiStation>, Box<dyn std::error::Error>> {
    let mut stations: Vec<ApiStation> = Vec::with_capacity(presets.len());
    for &preset in presets {
        let station = api
            .get_stations()
            .name(preset)
            .name_exact(true)
            .order(StationOrder::Clickcount)
            .reverse(true)
            .limit("1") // don't want to do this but may speed things up
            .send()?;
        stations.push(station[0].clone());
    }
    Ok(stations)
}

pub fn get_stations_by_tag(
    api: &RadioBrowserAPI,
    query: &String,
) -> Result<Vec<ApiStation>, Box<dyn Error>> {
    let stations = api
        .get_stations()
        .tag(query)
        .limit("100") // arbitary just here incase query is too generic
        .order(StationOrder::Votes)
        .reverse(true)
        .send()?;

    Ok(stations)
}

pub fn get_tags(api: &RadioBrowserAPI, limit: &str) -> Result<Vec<ApiTag>, Box<dyn Error>> {
    let tags = api
        .get_tags()
        .order(TagOrder::StationCount)
        .reverse(true)
        .limit(limit) // replace with dynamic
        .send()?;

    Ok(tags)
}

pub fn get_countries(
    api: &RadioBrowserAPI,
    limit: &str,
) -> Result<Vec<ApiCountry>, Box<dyn Error>> {
    let countries = api
        .get_countries()
        .order(CountryOrder::StationCount)
        .reverse(true)
        .limit(limit) // replace with dynamic
        .send()?;

    Ok(countries)
}

pub fn get_stations_by_country(
    api: &RadioBrowserAPI,
    query: &String,
) -> Result<Vec<ApiStation>, Box<dyn Error>> {
    let stations = api
        .get_stations()
        .country(query)
        .limit("100") // arbitary just here incase query is too generic
        .order(StationOrder::Votes)
        .reverse(true)
        .send()?;

    Ok(stations)
}
