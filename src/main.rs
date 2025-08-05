use std::{
    fs::{copy, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

const BUCKET_PATH: &str = "./bucket";
const DIST_PATH: &str = "./dist";
const PROCESS_LIST_PATH: &str = "./process.tsv";

fn main() {
    let process_list = read_process_list();
    for (name, id) in process_list {
        if let Err(err) = copy(
            PathBuf::from(BUCKET_PATH).join(&format!("bucket ({id}).yaml")),
            PathBuf::from(DIST_PATH).join(&format!("{name}.yaml")),
        ) {
            println!("Error when copying 'bucket ({id}).yaml' to '{name}.yaml': {err}");
        }
    }
}

fn read_process_list() -> Vec<(String, String)> {
    BufReader::new(
        File::open(Path::new(PROCESS_LIST_PATH))
            .unwrap_or_else(|_| panic!("Failed to open '{PROCESS_LIST_PATH}'")),
    )
    .lines()
    .zip(1..)
    .map(|(line, i)| {
        if let Some((name, id)) = line
            .unwrap_or_else(|_| panic!("Failed to read line {i}."))
            .split_once('\t')
        {
            (name.to_string(), id.to_string())
        } else {
            panic!("Line {i} does not contain a pair of name and id.");
        }
    })
    .collect()
}
