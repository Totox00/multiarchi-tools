mod comments;
mod game;
mod name;
mod read;
mod special;
mod util;
mod valid_games;
mod write;

use std::{
    fmt::Write as FmtWrite,
    fs::{read_to_string, rename, File},
    io::Write,
    path::PathBuf,
};

use yaml_rust2::{YamlEmitter, YamlLoader};

use crate::{
    comments::{get_comments, insert_comments},
    game::choose_game,
    name::set_name,
    read::read_process_list,
    special::handle_special,
    write::write_to_output_list,
};

pub const BUCKET_PATH: &str = "./bucket";
pub const USED_PATH: &str = "./used";
pub const DIST_PATH: &str = "./dist";
pub const PROCESS_LIST_PATH: &str = "./process.tsv";
pub const OUTPUT_LIST_PATH: &str = "./output.tsv";

fn main() {
    let process_list = read_process_list();
    let mut output_writer = match File::create(PathBuf::from(OUTPUT_LIST_PATH)) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Error when creating output file: {err}");
        }
    };

    for (name, id) in process_list {
        let (games, notes) = process_file(&name, &id);

        if let Some((_, count)) = games.iter().find(|(game, _)| game == "Keymaster's Keep") {
            if *count > 1 {
                println!("'{name}.yaml' contains {count} Keymaster's Keeps");
            } else {
                println!("'{name}.yaml' contains {count} Keymaster's Keep");
            }
        }

        write_to_output_list(&mut output_writer, &name, &games, &notes);
        if let Err(err) = rename(
            PathBuf::from(BUCKET_PATH).join(format!("bucket ({id}).yaml")),
            PathBuf::from(USED_PATH).join(format!("bucket ({id}).yaml")),
        ) {
            println!("Failed to move 'bucket ({id}).yaml' to used directory: {err}");
        }
    }
}

fn process_file(name: &str, id: &str) -> (Vec<(String, u32)>, Vec<String>) {
    let mut games_in_file = vec![];
    let mut notes = vec![];
    let content = match read_to_string(PathBuf::from(BUCKET_PATH).join(format!("bucket ({id}).yaml"))) {
        Ok(content) => content.trim_matches(|char: char| char == '\n' || char == '\r' || char == '\u{feff}').to_owned(),
        Err(err) => {
            println!("Error when reading 'bucket ({id}).yaml': {err}");
            return (games_in_file, notes);
        }
    };

    let comments = get_comments(&content);

    let mut documents = match YamlLoader::load_from_str(&content) {
        Ok(documents) => documents,
        Err(err) => {
            println!("Error when loading 'bucket ({id}).yaml': {err}");
            return (games_in_file, notes);
        }
    };

    for doc in &mut documents {
        let game = if let Some(game) = choose_game(doc) {
            let game_str = game.as_str().expect("Game should be a string");
            if let Some((_, count)) = games_in_file.last_mut().filter(|(existing_game, _)| existing_game == game_str) {
                *count += 1;
            } else {
                games_in_file.push((game_str.to_string(), 1));
            }

            game
        } else {
            return (games_in_file, notes);
        };

        if game.as_str().is_some_and(|game| game == "Chrono Trigger Jets of Time") {
            println!("'{name}.yaml' contains a Chrono Trigger Jets of Time");
        } else {
            set_name(doc, &format!("{name}{{NUMBER}}"), &game);
        }

        notes.extend(handle_special(doc, &game, name));
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

    let lines = insert_comments(output_buf, &comments, id);

    match File::create(PathBuf::from(DIST_PATH).join(format!("{name}.yaml"))) {
        Ok(mut writer) => writer.write_all(lines.join("\n").as_bytes()).unwrap_or_else(|_| println!("Error when writing to '{name}.yaml'")),
        Err(err) => {
            println!("Error when creating '{name}.yaml': {err}")
        }
    };

    (games_in_file, notes)
}
