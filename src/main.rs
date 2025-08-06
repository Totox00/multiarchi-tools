use std::{
    fmt::Write as FmtWrite,
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

const BUCKET_PATH: &str = "./bucket";
const DIST_PATH: &str = "./dist";
const PROCESS_LIST_PATH: &str = "./process.tsv";

fn main() {
    let process_list = read_process_list();
    for (name, id) in process_list {
        process_file(&name, &id);
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

fn process_file(name: &str, id: &str) {
    let content =
        match read_to_string(PathBuf::from(BUCKET_PATH).join(format!("bucket ({id}).yaml"))) {
            Ok(content) => content
                .trim_matches(|char: char| char == '\n' || char == '\r' || char == '\u{feff}')
                .to_owned(),
            Err(err) => {
                println!("Error when reading 'bucket ({id}).yaml': {err}");
                return;
            }
        };

    let mut documents = match YamlLoader::load_from_str(&content) {
        Ok(documents) => documents,
        Err(err) => {
            println!("Error when loading 'bucket ({id}).yaml': {err}");
            return;
        }
    };

    if documents.len() == 1 {
        set_name(documents.first_mut().unwrap(), name);
    } else {
        for (doc, i) in documents.iter_mut().zip(1..) {
            set_name(doc, &format!("{name}{i}"));
        }
        if documents.len() > 8 {
            println!("'{name}.yaml' contains {} games.", documents.len());
        }
    }

    let mut output_buf = String::new();

    for doc in documents {
        let mut emitter = YamlEmitter::new(&mut output_buf);
        emitter.dump(&doc);
        output_buf.write_char('\n');
    }

    match File::create(PathBuf::from(DIST_PATH).join(format!("{name}.yaml"))) {
        Ok(mut writer) => writer
            .write_all(output_buf.as_bytes())
            .unwrap_or_else(|_| println!("Error when writing to '{name}.yaml'")),
        Err(err) => {
            println!("Error when creating '{name}.yaml': {err}")
        }
    };
}

fn set_name(doc: &mut Yaml, name: &str) {
    let name_key = Yaml::from_str("name");
    let game_key = Yaml::from_str("game");
    let triggers_key = Yaml::from_str("triggers");
    let name_value = Yaml::from_str(name);

    let hash = if let Some(hash) = doc.as_mut_hash() {
        hash
    } else {
        return;
    };

    let games = if let Some(games) = hash.get(&game_key) {
        match games {
            Yaml::Hash(games) => games.keys().cloned().collect(),
            Yaml::String(game) => {
                vec![Yaml::from_str(game)]
            }
            _ => return,
        }
    } else {
        return;
    };

    for game in games {
        if let Some(game_options) = hash.get_mut(&game) {
            if let Some(game_options_hash) = game_options.as_mut_hash() {
                if let Some(triggers) = game_options_hash.get_mut(&triggers_key) {
                    strip_name_changes_from_triggers(triggers);
                }
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
