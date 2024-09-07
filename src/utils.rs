use std::fs::File;
use std::io::{BufReader, Read};

pub fn sanitize(s: &str) -> String {
    s.to_lowercase()
        .split(' ')
        .collect::<Vec<&str>>()
        .join("_")
        .as_str()
        .to_string()
}

pub fn read_file(path: &str) -> String {
    let file = File::open(path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("Failed to read file");
    contents
}

pub fn read_json(path: &str) -> serde_json::Value {
    let file = std::fs::File::open(path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader).expect("Error parsing json");

    json
}
