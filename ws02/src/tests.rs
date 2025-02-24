#[cfg(test)]
use crate::*;

#[test]
pub fn test_most_least_used_stations() {
    let solution = new_solution().unwrap();
    // There are two least used stations, both with 170 total uses. You can return either of them.
    assert!(matches!(
        most_least_used_stations(&solution, TimeOfDay::Morning),
        Some((min, max)) if
            (min == "Bowral" && max == "Central")
            || (min == "Lithgow" && max == "Central")
    ));
}

#[test]
pub fn test_search_station_busiest_times_of_day() {
    let solution = new_solution().unwrap();
    assert_eq!(
        sorted(vec![
            (TimeOfDay::Morning, 76500),
            (TimeOfDay::Midday, 124060),
            (TimeOfDay::Evening, 304890),
            (TimeOfDay::Midnight, 117840),
        ]),
        sorted(search_station_busiest_times_of_day(&solution, "Central").unwrap())
    );
}

#[test]
pub fn test_search_station_busiest_year() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some("2020".to_string()),
        search_station_busiest_year(&solution, "Central")
    );
}

#[test]
pub fn test_find_largest_yearly_utilisation_increase() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some("Town Hall".to_string()),
        find_largest_yearly_utilisation_increase(&solution)
    );
}

#[test]
pub fn test_find_biggest_percentage_change() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some("Olympic Park".to_string()),
        find_biggest_percentage_change(&solution)
    );
}

#[test]
pub fn test_find_north_most_station() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some("Singleton".to_string()),
        find_north_most_station(&solution)
    );
}

#[test]
pub fn test_find_south_most_station() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some("Bomaderry".to_string()),
        find_south_most_station(&solution)
    );
}

#[test]
pub fn test_find_east_most_station() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some("Newcastle Interchange".to_string()),
        find_east_most_station(&solution)
    );
}

#[test]
pub fn test_find_west_most_station() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some("Bathurst".to_string()),
        find_west_most_station(&solution)
    );
}

#[cfg(test)]
fn sorted<T: Ord>(mut vec: Vec<T>) -> Vec<T> {
    vec.sort();
    vec
}

#[test]
pub fn test_find_closest_stations() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some(sorted(vec![
            "Martin Place".to_string(),
            "St James".to_string()
        ])),
        find_two_closest_stations(&solution).map(|(a, b)| sorted(vec![a, b]))
    );
}

#[test]
pub fn test_find_furthest_stations() {
    let solution = new_solution().unwrap();
    assert_eq!(
        Some(sorted(vec![
            "East Maitland".to_string(),
            "Goulburn".to_string()
        ])),
        find_two_furthest_stations(&solution).map(|(a, b)| sorted(vec![a, b]))
    );
}
