//! This library provides a simple implementation of the Caesar cipher for strings.
//!
//! When given a list of strings and a number, it shifts each line of the string by the specified number
//! and returns the shifted strings in a vector.

// Default number to shift if no shift number is provided.
const DEFAULT_SHIFT: i32 = 5;
// ASCII value of 'A'
const UPPERCASE_A: i32 = 65;
// ASCII value of 'a'
const LOWERCASE_A: i32 = 97;
// Number of letters in English alphabet
const ALPHABET_SIZE: i32 = 26;

/// Perform caesar_shift for each letter in each line
/// # Arguments
/// * `shift_by` - The number to shift each letter by
/// * `lines` - Vector of lines to caesar shift
///
/// ```
/// use doctor_who::caesar_shift;
/// let lines = vec!["a".to_string(), "ab".to_string(), "abc".to_string()];
/// assert_eq!(vec!["b".to_string(), "bc".to_string(), "bcd".to_string()], caesar_shift(Some(1), lines));
/// ```
pub fn caesar_shift(shift_by: Option<i32>, lines: Vec<String>) -> Vec<String> {
    let shift_number = shift_by.unwrap_or(DEFAULT_SHIFT);

    // no idea what this is doing? Ask the forums and/or
    // look back at the functional programming lectures!
    lines
        .iter()
        .map(|line| shift(shift_number, line.to_string()))
        .collect()
}

/// Takes in a line and shift the line by a number
fn shift(shift_by: i32, line: String) -> String {
    let mut result: Vec<char> = Vec::new();

    // turn shift_by into a positive number between 0 and 25
    let shift_by = shift_by % ALPHABET_SIZE + ALPHABET_SIZE;

    line.chars().for_each(|c| {
        let ascii = c as i32;

        if ('A'..='Z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - UPPERCASE_A) + shift_by, ALPHABET_SIZE) + UPPERCASE_A,
            ));
        } else if ('a'..='z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - LOWERCASE_A) + shift_by, ALPHABET_SIZE) + LOWERCASE_A,
            ));
        } else {
            result.push(c)
        }
    });

    result.iter().collect()
}

/// Returns the aboslute value of the modulo of a % b
fn abs_modulo(a: i32, b: i32) -> i32 {
    (a % b).abs()
}

/// returns ASCII from u32
fn to_ascii(i: i32) -> char {
    char::from_u32(i as u32).unwrap()
}
