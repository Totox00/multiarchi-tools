mod game;
mod read;

use std::{
    collections::HashMap,
    env::args,
    fmt::Write as FmtWrite,
    fs::{read_to_string, rename, File},
    io::Write,
    path::PathBuf,
};

use common::{
    comments::{get_comments, insert_comments},
    name::{rename_plando_worlds, set_name},
    special::handle_special,
    write::{write_to_bot_output, write_to_output_list},
};
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

use crate::{game::choose_game, read::read_process_list};

pub const BUCKET_PATH: &str = "./bucket";
pub const USED_PATH: &str = "./used";
pub const DIST_PATH: &str = "./dist";
pub const PROCESS_LIST_PATH: &str = "./process.tsv";
pub const OUTPUT_LIST_PATH: &str = "./output.tsv";
pub const OUTPUT_BOT_PATH: &str = "./bot_output.txt";

fn main() {
    let process_list = read_process_list();
    let mut output_writer = match File::create(PathBuf::from(OUTPUT_LIST_PATH)) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Error when creating output file: {err}");
        }
    };

    let mut bot_output_writer = match File::create(PathBuf::from(OUTPUT_BOT_PATH)) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Error when creating bot output file: {err}");
        }
    };

    for (name, id) in process_list {
        let games = process_file(&name, &id);

        if let Some((_, count, _)) = games.iter().find(|(game, _, _)| game == "Keymaster's Keep") {
            if *count > 1 {
                println!("'{name}.yaml' contains {count} Keymaster's Keeps");
            } else {
                println!("'{name}.yaml' contains a Keymaster's Keep");
            }
        }

        write_to_output_list(&mut output_writer, &name, &games);
        write_to_bot_output(&mut bot_output_writer, &name, &games);

        if args().any(|arg| arg == "--move-files") {
            if let Err(err) = rename(
                PathBuf::from(BUCKET_PATH).join(format!("bucket ({id}).yaml")),
                PathBuf::from(USED_PATH).join(format!("bucket ({id}).yaml")),
            ) {
                println!("Failed to move 'bucket ({id}).yaml' to used directory: {err}");
            }
        }
    }
}

fn process_file(name: &str, id: &str) -> Vec<(String, u32, Vec<String>)> {
    let mut games_in_file = vec![];
    let content = match read_to_string(PathBuf::from(BUCKET_PATH).join(format!("bucket ({id}).yaml"))) {
        Ok(content) => content.trim_matches(|char: char| char == '\n' || char == '\r' || char == '\u{feff}').to_owned(),
        Err(err) => {
            println!("Error when reading 'bucket ({id}).yaml': {err}");
            return games_in_file;
        }
    };

    let comments = get_comments(&content);

    let mut documents = match YamlLoader::load_from_str(&content) {
        Ok(documents) => documents,
        Err(err) => {
            println!("Error when loading 'bucket ({id}).yaml': {err}");
            return games_in_file;
        }
    };

    let single_game = documents.len() == 1;

    let mut name_mapping = HashMap::new();

    for (i, doc) in documents.iter_mut().enumerate() {
        let new_name = if single_game { name.to_string() } else { format!("{name}{}", i + 1) };
        let mut old_name = None;

        if let Some(game) = choose_game(doc) {
            let game_str = game.as_str().expect("Game should be a string");
            if let Some((_, count, last_notes)) = games_in_file.last_mut().filter(|(existing_game, _, _)| existing_game == game_str) {
                *count += 1;
                last_notes.extend(handle_special(doc, &game, name));
            } else {
                games_in_file.push((game_str.to_string(), 1, handle_special(doc, &game, name)));
            }

            if game.as_str().is_some_and(|game| game == "Chrono Trigger Jets of Time") {
                println!("'{name}.yaml' contains a Chrono Trigger Jets of Time");
            } else if game.as_str().is_some_and(|game| game == "Final Fantasy") {
                println!("'{name}.yaml' contains a Final Fantasy");
            } else {
                old_name = set_name(doc, &new_name, Some(&game));
            }
        } else {
            old_name = set_name(doc, &new_name, None);
        }

        if let Some(old_name) = old_name {
            name_mapping.insert(old_name, Yaml::String(new_name));
        }
    }

    rename_plando_worlds(&name_mapping, &mut documents, name);

    if documents.len() > 8 {
        println!("'{name}.yaml' contains {} games.", documents.len());
    }

    let mut output_buf = String::new();

    for doc in documents {
        let mut emitter = YamlEmitter::new(&mut output_buf);
        let _ = emitter.dump(&doc);
        let _ = output_buf.write_char('\n');
    }

    let lines = insert_comments(output_buf, &comments, &format!("bucket ({id}).yaml"));

    match File::create(PathBuf::from(DIST_PATH).join(format!("{name}.yaml"))) {
        Ok(mut writer) => writer.write_all(lines.join("\n").as_bytes()).unwrap_or_else(|_| println!("Error when writing to '{name}.yaml'")),
        Err(err) => {
            println!("Error when creating '{name}.yaml': {err}")
        }
    };

    games_in_file
}
