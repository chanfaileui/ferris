#![allow(unused)]
// #![allow(dead_code)]
mod tests;
mod useful_code;

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
    records: Vec<CSVRecord>,
}

pub fn new_solution() -> Result<Solution, Box<dyn Error>> {
    // TODO: Initialise the common state you will require here.

    let path = Path::new("trains.csv");

    let records: Vec<CSVRecord> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    Ok(Solution { records }) // if you name the struct something else: Ok(Solution { STRUCT_NAME: records })
}

/// What is the north-most station?
pub fn find_north_most_station(solution: &Solution) -> Option<String> {
    let Some(mut north_most) = &solution.records.get(0) else {
        return None;
    };

    for record in solution.records.iter().skip(1) {
        if record.latitude > north_most.latitude {
            north_most = record;
        }
    }

    Some(north_most.station.clone())
}

/// What is the south-most station?
pub fn find_south_most_station(solution: &Solution) -> Option<String> {
    solution
        .records
        .iter()
        .reduce(|acc, record| {
            if record.latitude < acc.latitude {
                record
            } else {
                acc
            }
        })
        .map(|record| record.station.clone())
}

/// What is the east-most station?
pub fn find_east_most_station(solution: &Solution) -> Option<String> {
    solution
        .records
        .iter()
        .reduce(|acc, record| {
            if record.longitude > acc.longitude {
                record
            } else {
                acc
            }
        })
        .map(|record| record.station.clone())
}

/// What is the west-most station?
pub fn find_west_most_station(solution: &Solution) -> Option<String> {
    solution
        .records
        .iter()
        .reduce(|acc, record| {
            if record.longitude < acc.longitude {
                record
            } else {
                acc
            }
        })
        .map(|record| record.station.clone())
}

/// Return the names of the most and least used (total entries + exits) stations on the NSW network at each time of day, in total over all of the years.
pub fn most_least_used_stations(
    solution: &Solution,
    time_of_day: TimeOfDay,
) -> Option<(String, String)> {
    // look at which columns based on time of day (eg col2 + 3)
    // add total entries + exits

    // null check
    if solution.records.is_empty() {
        return None;
    }

    //
    let mut stations = std::collections::HashMap::new();

    for record in &solution.records {
        // Get entries and exits based on time of day
        let (entries, exits) = match time_of_day {
            TimeOfDay::Morning => (record.entries_morning, record.exits_morning),
            TimeOfDay::Midday => (record.entries_midday, record.exits_midday),
            TimeOfDay::Evening => (record.entries_evening, record.exits_evening),
            TimeOfDay::Midnight => (record.entries_midnight, record.exits_midnight),
            TimeOfDay::Total => (record.entries_total, record.exits_total),
        };

        if entries == None || exits == None {
            continue;
        }

        let total = entries.unwrap() + exits.unwrap();

        //
        stations
            .entry(record.station.clone())
            .and_modify(|count| *count += total)
            .or_insert(total);
    }

    if stations.is_empty() {
        return None;
    }

    // Find station with maximum usage
    let most_used = stations
        .iter()
        .max_by_key(|&(_, total)| total)
        .map(|(station, _)| station.clone())
        .unwrap();

    // Find station with minimum usage
    let least_used = stations
        .iter()
        .min_by_key(|&(_, total)| total)
        .map(|(station, _)| station.clone())
        .unwrap();

    Some((least_used, most_used))
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
