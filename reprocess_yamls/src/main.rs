use std::{
    fmt::Write,
    fs::{File, read_to_string},
    io::Write as IoWrite,
    path::{Path, PathBuf},
};

use common::{
    comments::{get_comments, insert_comments},
    special::handle_special,
    write::write_to_output_list,
};
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

pub const DIST_PATH: &str = "./dist";
pub const OUTPUT_LIST_PATH: &str = "./output.tsv";

fn main() {
    if let Ok(dir) = Path::new(DIST_PATH).read_dir() {
        let mut output_writer = match File::create(PathBuf::from(OUTPUT_LIST_PATH)) {
            Ok(writer) => writer,
            Err(err) => {
                panic!("Error when creating output file: {err}");
            }
        };

        for yaml in dir.flatten() {
            let buf = PathBuf::from(yaml.file_name());
            let name = buf.file_stem().map(|str| str.to_string_lossy()).unwrap_or_else(|| panic!("Failed to get name from {:?}", yaml.path()));
            let games = process_file(&yaml.path(), &name);

            if let Some((_, count, _)) = games.iter().find(|(game, _, _)| game == "Keymaster's Keep") {
                if *count > 1 {
                    println!("'{name}.yaml' contains {count} Keymaster's Keeps");
                } else {
                    println!("'{name}.yaml' contains a Keymaster's Keep");
                }
            }

            write_to_output_list(&mut output_writer, &name, &games);
        }
    }
}

fn process_file(path: &Path, name: &str) -> Vec<(String, u32, Vec<String>)> {
    let mut games_in_file = vec![];
    let content = match read_to_string(path) {
        Ok(content) => content.trim_matches(|char: char| char == '\n' || char == '\r' || char == '\u{feff}').to_owned(),
        Err(err) => {
            println!("Error when reading '{name}.yaml': {err}");
            return games_in_file;
        }
    };

    let comments = get_comments(&content);

    let mut documents = match YamlLoader::load_from_str(&content) {
        Ok(documents) => documents,
        Err(err) => {
            println!("Error when loading '{name}.yaml': {err}");
            return games_in_file;
        }
    };

    for doc in &mut documents {
        if let Some(game) = doc.as_hash().and_then(|hash| hash.get(&Yaml::from_str("game"))).cloned() {
            let game_str = game.as_str().expect("Game should be a string");
            if let Some((_, count, last_notes)) = games_in_file.last_mut().filter(|(existing_game, _, _)| existing_game == game_str) {
                *count += 1;
                last_notes.extend(handle_special(doc, &game, name));
            } else {
                games_in_file.push((game_str.to_string(), 1, handle_special(doc, &game, name)));
            }

            if game.as_str().is_some_and(|game| game == "Chrono Trigger Jets of Time") {
                println!("'{name}.yaml' contains a Chrono Trigger Jets of Time");
            }
        }
    }
    if documents.len() > 8 {
        println!("'{name}.yaml' contains {} games.", documents.len());
    }

    let mut output_buf = String::new();

    for doc in documents {
        let mut emitter = YamlEmitter::new(&mut output_buf);
        let _ = emitter.dump(&doc);
        let _ = output_buf.write_char('\n');
    }

    let lines = insert_comments(output_buf, &comments, &format!("{name}.yaml"));

    match File::create(path) {
        Ok(mut writer) => writer.write_all(lines.join("\n").as_bytes()).unwrap_or_else(|_| println!("Error when writing to '{name}.yaml'")),
        Err(err) => {
            println!("Error when creating '{name}.yaml': {err}")
        }
    };

    games_in_file
}
