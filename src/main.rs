use std::{
    collections::HashMap,
    fmt::Write as FmtWrite,
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use rand::thread_rng;
use rand_distr::{Distribution, WeightedIndex};
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

const BUCKET_PATH: &str = "./bucket";
const DIST_PATH: &str = "./dist";
const PROCESS_LIST_PATH: &str = "./process.tsv";
const OUTPUT_LIST_PATH: &str = "./output.tsv";

fn main() {
    let process_list = read_process_list();
    let mut output_writer = match File::create(PathBuf::from(OUTPUT_LIST_PATH)) {
        Ok(writer) => writer,
        Err(err) => {
            panic!("Error when creating output file: {err}");
        }
    };

    for (name, id) in process_list {
        let games: Vec<_> = process_file(&name, &id).into_iter().collect();
        if let Err(err) = write!(&mut output_writer, "{name}\t") {
            println!("Failed to write to output file: {err}");
        }

        if games.len() == 1 {
            if games[0].1 > 1 {
                if let Err(err) = writeln!(&mut output_writer, "{} *{}", games[0].0, games[0].1) {
                    println!("Failed to write to output file: {err}");
                }
            } else if let Err(err) = writeln!(&mut output_writer, "{}", games[0].0) {
                println!("Failed to write to output file: {err}");
            }
        } else {
            if let Err(err) = write!(&mut output_writer, "\"") {
                println!("Failed to write to output file: {err}");
            }

            for (game, count) in &games[0..games.len() - 1] {
                if *count > 1 {
                    if let Err(err) = write!(&mut output_writer, "{} *{} AND\n ", game, count) {
                        println!("Failed to write to output file: {err}");
                    }
                } else if let Err(err) = write!(&mut output_writer, "{} AND\n ", game) {
                    println!("Failed to write to output file: {err}");
                }
            }

            let (game, count) = games.last().expect("Last game does not exist");
            if *count > 1 {
                if let Err(err) = writeln!(&mut output_writer, "{} *{}\"", game, count) {
                    println!("Failed to write to output file: {err}");
                }
            } else if let Err(err) = writeln!(&mut output_writer, "{}\"", game) {
                println!("Failed to write to output file: {err}");
            }
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

fn process_file(name: &str, id: &str) -> HashMap<String, u32> {
    let mut games_in_file = HashMap::new();
    let content =
        match read_to_string(PathBuf::from(BUCKET_PATH).join(format!("bucket ({id}).yaml"))) {
            Ok(content) => content
                .trim_matches(|char: char| char == '\n' || char == '\r' || char == '\u{feff}')
                .to_owned(),
            Err(err) => {
                println!("Error when reading 'bucket ({id}).yaml': {err}");
                return games_in_file;
            }
        };

    let mut last_key = None;
    let mut comments: Vec<(Option<&str>, String, bool)> = vec![];
    for line in content.lines() {
        let mut new_key = false;
        if let Some(key) = find_key(line) {
            last_key = Some(key);
            new_key = true;
        }
        if let Some((_, comment)) = line.split_once('#') {
            if let Some((last_last_key, last_comment, _)) = comments.last_mut() {
                if last_key == *last_last_key {
                    last_comment.push('\n');
                    last_comment.push('#');
                    last_comment.push_str(comment);
                } else {
                    comments.push((last_key, comment.to_string(), new_key));
                }
            } else {
                comments.push((last_key, comment.to_string(), new_key));
            }
        }
    }

    let mut documents = match YamlLoader::load_from_str(&content) {
        Ok(documents) => documents,
        Err(err) => {
            println!("Error when loading 'bucket ({id}).yaml': {err}");
            return games_in_file;
        }
    };

    for doc in &mut documents {
        let game = if let Some(game) = choose_game(doc) {
            let game_str = game.as_str().expect("Game should be a string");
            if let Some(count) = games_in_file.get_mut(game_str) {
                *count += 1;
            } else {
                games_in_file.insert(game_str.to_string(), 1);
            }

            game
        } else {
            return games_in_file;
        };
        set_name(doc, &format!("{name}{{NUMBER}}"), &game);
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

    let mut lines: Vec<_> = output_buf.lines().map(String::from).collect();
    let mut line_i = 0;
    'outer: for (last_key, comment, inline) in comments {
        if last_key.is_none() {
            lines.insert(line_i, format!("#{comment}"));
            line_i += 1;
            continue;
        }

        while find_key(&lines[line_i]) != last_key {
            line_i += 1;
            if line_i == lines.len() {
                println!("Failed to preserve all comments from 'bucket ({id}).yaml'");
                break 'outer;
            }
        }

        if inline {
            lines[line_i].push_str(&format!(" #{comment}"));
        } else {
            line_i += 1;
            lines.insert(line_i, format!("#{comment}"));
        }
        line_i += 1;
    }

    match File::create(PathBuf::from(DIST_PATH).join(format!("{name}.yaml"))) {
        Ok(mut writer) => writer
            .write_all(lines.join("\n").as_bytes())
            .unwrap_or_else(|_| println!("Error when writing to '{name}.yaml'")),
        Err(err) => {
            println!("Error when creating '{name}.yaml': {err}")
        }
    };

    games_in_file
}

fn choose_game(doc: &mut Yaml) -> Option<Yaml> {
    let game_key = Yaml::from_str("game");

    if let Some(games) = doc.as_mut_hash()?.get_mut(&game_key) {
        let game = match games {
            Yaml::Hash(games) => {
                let mut rng = thread_rng();

                let games: Vec<_> = games
                    .iter()
                    .filter_map(|(k, v)| match (k.as_str(), v.as_f64(), v.as_i64()) {
                        (Some(game), Some(weight), None) => Some((game, weight)),
                        (Some(game), None, Some(weight)) => Some((game, weight as f64)),
                        _ => None,
                    })
                    .collect();
                let dist = WeightedIndex::new(games.iter().map(|(_, weight)| weight))
                    .expect("Failed to create index");
                Yaml::from_str(games[dist.sample(&mut rng)].0)
            }
            Yaml::String(game) => Yaml::from_str(game),
            _ => return None,
        };

        *games = game.clone();
        Some(game)
    } else {
        None
    }
}

fn set_name(doc: &mut Yaml, name: &str, game: &Yaml) {
    let name_key = Yaml::from_str("name");
    let triggers_key = Yaml::from_str("triggers");
    let name_value = Yaml::from_str(name);

    let hash = if let Some(hash) = doc.as_mut_hash() {
        hash
    } else {
        return;
    };

    if let Some(game_options) = hash.get_mut(game) {
        if let Some(game_options_hash) = game_options.as_mut_hash() {
            if let Some(triggers) = game_options_hash.get_mut(&triggers_key) {
                strip_name_changes_from_triggers(triggers);
            }
        }
    }

    if let Some(triggers) = hash.get_mut(&triggers_key) {
        strip_name_changes_from_triggers(triggers);
    }

    if let Some(name_entry) = hash.get_mut(&name_key) {
        *name_entry = name_value;
    } else {
        hash.insert(name_key, name_value);
    }
}

fn strip_name_changes_from_triggers(doc: &mut Yaml) {
    let name_key = Yaml::from_str("name");
    let options_key = Yaml::from_str("options");
    let null_key = Yaml::from_str("null");
    let root_key = Yaml::from_str("~");
    let remove_key = Yaml::from_str("_remove_");

    if let Some(vec) = doc.as_mut_vec() {
        for trigger in vec.iter_mut() {
            if let Some(trigger_hash) = trigger.as_mut_hash() {
                if let Some(options) = trigger_hash.get_mut(&options_key) {
                    if let Some(options_hash) = options.as_mut_hash() {
                        let mut remove_null_hash = false;
                        if let Some(null) = options_hash.get_mut(&null_key) {
                            if let Some(null_hash) = null.as_mut_hash() {
                                if null_hash.remove(&name_key).is_some() && null_hash.is_empty() {
                                    remove_null_hash = true;
                                }
                            }
                        }
                        let mut remove_root_hash = false;
                        if let Some(root) = options_hash.get_mut(&root_key) {
                            if let Some(root_hash) = root.as_mut_hash() {
                                if root_hash.remove(&name_key).is_some() && root_hash.is_empty() {
                                    remove_root_hash = true;
                                }
                            }
                        }
                        if remove_null_hash {
                            options_hash.remove(&null_key);
                        }
                        if remove_root_hash {
                            options_hash.remove(&root_key);
                        }
                        if options_hash.is_empty() {
                            trigger_hash.insert(remove_key.to_owned(), Yaml::Boolean(true));
                        }
                    }
                }
            }
        }

        vec.retain(|trigger| {
            if let Some(trigger_hash) = trigger.as_hash() {
                !trigger_hash.contains_key(&remove_key)
            } else {
                true
            }
        });
    }
}

fn find_key(line: &str) -> Option<&str> {
    let not_comment = if let Some((content, _)) = line.split_once('#') {
        content
    } else {
        line
    };

    if let Some((key, _)) = not_comment.split_once(':') {
        Some(key.trim_start().trim_matches('\'').trim_matches('\"'))
    } else {
        None
    }
}
