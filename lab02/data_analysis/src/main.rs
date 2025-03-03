use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;

const ENROLMENTS_PATH: &str = "enrolments.psv";

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
struct Record {
    course_code: String,
    student_number: String,
    name: String,
    program: String,
    plan: String,
    wam: f32,
    session: String,
    birthdate: String,
    sex: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(ENROLMENTS_PATH)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    let mut students = HashSet::new();
    let mut courses = HashMap::new();
    let mut total_wam = 0.0;
    let mut unique_wam_count = 0;

    for result in rdr.deserialize() {
        let record: Record = result?;
        // If the student_number is not in the HashSet, add them, else ignore
        if students.insert(record.student_number.clone()) {
            total_wam += record.wam;
            unique_wam_count += 1;
        }

        courses
            .entry(record.course_code.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let unique_students = students.len();
    let average_wam = total_wam / unique_students.max(1) as f32;

    let (max_name, max_count) = courses.iter().max_by_key(|(_, count)| *count).unwrap();
    let (min_name, min_count) = courses.iter().min_by_key(|(_, count)| *count).unwrap();

    println!("Number of students: {}", unique_students);

    println!(
        "Most common course: {} with {} {}",
        max_name,
        max_count,
        if *max_count == 1 {
            "student"
        } else {
            "students"
        }
    );
    println!(
        "Least common course: {} with {} {}",
        min_name,
        min_count,
        if *min_count == 1 {
            "student"
        } else {
            "students"
        }
    );

    println!("Average WAM: {:.2}", average_wam);

    Ok(())
}
