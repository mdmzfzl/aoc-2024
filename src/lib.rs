use std::fs;

/// Read input file and return the contents as a string
pub fn read_input(day: u8) -> String {
    let path = format!("./src/inputs/day{:02}.txt", day);
    fs::read_to_string(path).expect("Failed to read input file")
}
