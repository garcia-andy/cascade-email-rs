use std::collections::HashMap;

use calamine::{open_workbook, Data, Error, Range, Reader, Xlsx};

use crate::excel2json::get_keys;

pub fn get_range(path: &str) -> Result<Range<Data>, Error> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let sheet_name = workbook.sheet_names()[0].clone();
    let range = workbook.worksheet_range(&sheet_name)?;
    Ok(range)
}

pub fn parse(file: &str) -> Vec<serde_json::Value> {
    let range = get_range(file).expect("Error getting the first sheet");

    let keys = get_keys(0, &range);

    let key_row: u32 = 0;

    let mut json_map: Vec<HashMap<String, String>> = vec![];
    let mut json_result: Vec<serde_json::Value> = vec![];
    crate::excel2json::xlsx2map(key_row, &keys, &range, &mut json_map);

    for map in json_map {
        let v = serde_json::to_value(map).expect("Invalid value given");
        json_result.push(v);
    }

    json_result
}
