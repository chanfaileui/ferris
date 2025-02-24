use std::collections::HashMap;

#[derive(Debug)]
struct Record {
    time_period: String,
    station: String,
    entries: HashMap<TimeOfDay, i32>,
    exits: HashMap<TimeOfDay, i32>,
    latitude: f64,
    longitude: f64,
}

fn convert_csvrecord_to_record(csv_record: &CSVRecord) -> Record {
    let mut record = Record {
        time_period: csv_record.time_period.clone(),
        station: csv_record.station.clone(),
        entries: HashMap::new(),
        exits: HashMap::new(),
        latitude: csv_record.latitude,
        longitude: csv_record.longitude,
    };

    if let Some(e) = csv_record.entries_morning {
        record.entries.insert(TimeOfDay::Morning, e);
    }
    if let Some(e) = csv_record.entries_midday {
        record.entries.insert(TimeOfDay::Midday, e);
    }
    if let Some(e) = csv_record.entries_evening {
        record.entries.insert(TimeOfDay::Evening, e);
    }
    if let Some(e) = csv_record.entries_midnight {
        record.entries.insert(TimeOfDay::Midnight, e);
    }
    if let Some(e) = csv_record.entries_total {
        record.entries.insert(TimeOfDay::Total, e);
    }

    if let Some(e) = csv_record.exits_morning {
        record.exits.insert(TimeOfDay::Morning, e);
    }
    if let Some(e) = csv_record.exits_midday {
        record.exits.insert(TimeOfDay::Midday, e);
    }
    if let Some(e) = csv_record.exits_evening {
        record.exits.insert(TimeOfDay::Evening, e);
    }
    if let Some(e) = csv_record.exits_midnight {
        record.exits.insert(TimeOfDay::Midnight, e);
    }
    if let Some(e) = csv_record.exits_total {
        record.exits.insert(TimeOfDay::Total, e);
    }

    record
}
