use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::PROCESS_LIST_PATH;

pub fn read_process_list() -> Vec<(String, String)> {
    BufReader::new(
        File::open(Path::new(PROCESS_LIST_PATH))
            .unwrap_or_else(|_| panic!("Failed to open '{PROCESS_LIST_PATH}'")),
    )
    .lines()
    .zip(1..)
    .map(|(line, i)| {
        (
            line.unwrap_or_else(|_| panic!("Failed to read line {i}.")),
            i,
        )
    })
    .filter(|(line, _)| !line.is_empty())
    .map(|(line, i)| {
        if let Some((name, id)) = line.split_once('\t') {
            (name.to_string(), id.to_string())
        } else {
            panic!("Line {i} does not contain a pair of name and id.");
        }
    })
    .collect()
}
