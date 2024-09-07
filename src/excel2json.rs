use calamine::{Data, Range};
use std::collections::HashMap;

use crate::utils::sanitize;

pub fn get_keys(start: u32, range: &Range<Data>) -> Vec<String> {
    let col = range.get_size().1 as u32;
    let mut keys: Vec<String> = vec![];
    for n in 0..col {
        let k = range.get_value((start, n)).unwrap().to_string();
        keys.push(sanitize(k.as_str()));
    }
    keys
}

pub fn parse_row(keys: &[String], range: &Range<Data>) -> HashMap<String, String> {
    let mut line: HashMap<String, String> = HashMap::new();
    for n in 0..range.get_size().1 {
        let k = range.get((0, n)).unwrap().to_string();
        line.insert(keys[n].to_string(), k);
    }
    line
}

pub fn xlsx2map(
    key_row: u32,
    keys: &[String],
    range: &Range<Data>,
    vec: &mut Vec<HashMap<String, String>>,
) {
    let (row, col): (usize, usize) = range.get_size();
    let data_row = key_row + 1;
    for n in data_row..row as u32 {
        let rec = range.range((n, 0), (n, col as u32 - 1));

        vec.push(parse_row(&keys, &rec));
    }
}

pub fn xlsx2string(
    key_row: u32,
    keys: &[String],
    range: &Range<Data>,
    vec: &mut Vec<HashMap<String, String>>,
) -> String {
    xlsx2map(key_row, keys, range, vec);
    serde_json::to_string_pretty(&vec).unwrap()
}
