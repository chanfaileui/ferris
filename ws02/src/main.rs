#![allow(dead_code)]
mod tests;

use std::error::Error;
use std::path::Path;

use geoutils::Location;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CSVRecord {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>,

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimeOfDay {
    Morning,
    Midday,
    Evening,
    Midnight,
    Total,
}

/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: You can test your `Solution` methods here manually, or call `cargo test` to execute unit tests.
    let solution = new_solution()?;

    Ok(())
}

pub struct Solution {
    // TODO: You can put whatever state you require for each query here.
}

pub fn new_solution() -> Result<Solution, Box<dyn Error>> {
    // TODO: Initialise the common state you will require here.

    let path = Path::new("trains.csv");

    let records: Vec<CSVRecord> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    todo!()
}

/// What is the north-most station?
pub fn find_north_most_station(solution: &Solution) -> Option<String> {
    todo!()
}

/// What is the south-most station?
pub fn find_south_most_station(solution: &Solution) -> Option<String> {
    todo!()
}

/// What is the east-most station?
pub fn find_east_most_station(solution: &Solution) -> Option<String> {
    todo!()
}

/// What is the west-most station?
pub fn find_west_most_station(solution: &Solution) -> Option<String> {
    todo!()
}

/// Return the names of the most and least used (total entries + exits) stations on the NSW network at each time of day, in total over all of the years.
pub fn most_least_used_stations(
    solution: &Solution,
    time_of_day: TimeOfDay,
) -> Option<(String, String)> {
    todo!()
}

// TODO: if you think the Vec return type is inefficient/unsuitable, ask your tutor about more flexible alternatives (hint: iterators).
/// Allow a user to search for a station, and show it's busiest times of day.
pub fn search_station_busiest_times_of_day(
    solution: &Solution,
    station_name: &str,
) -> Option<Vec<(TimeOfDay, i32)>> {
    todo!()
}

/// Allow a user to search for a station, if it exists, and show it's busiest year.
pub fn search_station_busiest_year(solution: &Solution, station_name: &str) -> Option<String> {
    todo!()
}

/// Which station had its yearly utilisation (total entries + exits) increase the most from 2016 (inclusive) to 2020 (inclusive)?
pub fn find_largest_yearly_utilisation_increase(solution: &Solution) -> Option<String> {
    todo!()
}

/// Which station had the biggest percentage change in utilisation (total entries + exits) from 2019 to 2020?
pub fn find_biggest_percentage_change(solution: &Solution) -> Option<String> {
    todo!()
}

/// Find the names of the two closest from each other.
pub fn find_two_closest_stations(solution: &Solution) -> Option<(String, String)> {
    todo!()
}

/// Find the names of the two furthest away from each other.
pub fn find_two_furthest_stations(solution: &Solution) -> Option<(String, String)> {
    todo!()
}
