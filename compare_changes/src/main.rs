use std::{collections::HashMap, fs::File, io::Read, path::Path};

use yaml_rust2::{Yaml, YamlLoader};

pub const COMPARE_OLD_PATH: &str = "./compare_old";
pub const COMPARE_NEW_PATH: &str = "./compare_new";

fn main() {
    let game_key = Yaml::from_str("game");

    let old_yamls = read_yamls(COMPARE_OLD_PATH);
    let new_yamls = read_yamls(COMPARE_NEW_PATH);

    for (name, old_yaml) in old_yamls {
        if let Some(new_yaml) = new_yamls.get(&name) {
            if let (Some(old), Some(new)) = (old_yaml.as_hash(), new_yaml.as_hash()) {
                let old_game = old.get(&game_key).unwrap_or_else(|| panic!("{name}: Old yaml has no game"));
                let new_game = new.get(&game_key).unwrap_or_else(|| panic!("{name}: New yaml has no game"));

                if old_game != new_game {
                    println!("{name}: Game name has been changed");
                }

                let old_options = old.get(old_game).unwrap_or_else(|| panic!("{name}: Old yaml has no game options"));
                let new_options = new.get(new_game).unwrap_or_else(|| panic!("{name}: New yaml has no game options"));

                compare(&name, old_options, new_options);
            }
        } else {
            println!("{name}: No longer exists");
        }
    }
}

fn read_yamls(path: &str) -> HashMap<String, Yaml> {
    let mut yamls = HashMap::new();

    if let Ok(dir) = Path::new(path).read_dir() {
        for yaml in dir.flatten() {
            let mut yaml_str = String::new();
            File::open(yaml.path())
                .expect("Failed to open yaml file")
                .read_to_string(&mut yaml_str)
                .expect("Failed to read yaml file");

            yamls.insert(
                yaml.file_name().into_string().expect("Failed to convert OsString to String"),
                YamlLoader::load_from_str(yaml_str.trim_matches(|char: char| char == '\n' || char == '\r' || char == '\u{feff}'))
                    .unwrap_or_else(|err| panic!("Failed to read yaml file {:?} for reason {:?}", yaml.path(), err))
                    .remove(0),
            );
        }
    }

    yamls
}

macro_rules! value_change {
    ($name:expr, $old:expr, $new:expr) => {
        if $old != $new {
            println!("{}: Value changed from {} to {}", $name, $old, $new);
        }
    };
}

fn compare(name: &str, old: &Yaml, new: &Yaml) {
    match (old, new) {
        (Yaml::Real(old_value), Yaml::Real(new_value)) => value_change!(name, old_value, new_value),
        (Yaml::Integer(old_value), Yaml::Integer(new_value)) => value_change!(name, old_value, new_value),
        (Yaml::String(old_value), Yaml::String(new_value)) => value_change!(name, old_value, new_value),
        (Yaml::Boolean(old_value), Yaml::Boolean(new_value)) => value_change!(name, old_value, new_value),
        (Yaml::Array(old_arr), Yaml::Array(new_arr)) => {
            if old_arr.len() != new_arr.len() {
                println!("{name}: Length changed from {} to {}", old_arr.len(), new_arr.len());
            } else {
                let mut any_changes = false;

                for old_item in old_arr {
                    if !new_arr.contains(old_item) {
                        println!("{name}: `{}` removed", to_string(old_item));
                        any_changes = true;
                    }
                }

                for new_item in new_arr {
                    if !old_arr.contains(new_item) {
                        println!("{name}: `{}` added", to_string(new_item));
                        any_changes = true;
                    }
                }

                if !any_changes && old_arr != new_arr {
                    println!("{name}: Order changed from {} to {}", to_string(old), to_string(new));
                }
            }
        }
        (Yaml::Hash(old_hash), Yaml::Hash(new_hash)) => {
            let mut compared_keys = vec![];
            for (key, old_value) in old_hash {
                compared_keys.push(key);
                if let Some(new_value) = new_hash.get(key) {
                    compare(&format!("{name}: {}", to_string(key)), old_value, new_value);
                } else {
                    println!("{name}: Key `{}` removed", to_string(key));
                }
            }

            for (key, _) in new_hash {
                if !compared_keys.contains(&key) {
                    println!("{name}: Key `{}` added", to_string(key))
                }
            }
        }
        (Yaml::Alias(old_value), Yaml::Alias(new_value)) => value_change!(name, old_value, new_value),
        (Yaml::Null, Yaml::Null) | (Yaml::BadValue, Yaml::BadValue) => (),
        _ => println!("{name}: Type changed"),
    }
}

fn to_string(yaml: &Yaml) -> String {
    match yaml {
        Yaml::Real(str) | Yaml::String(str) => str.to_owned(),
        Yaml::Integer(int) => int.to_string(),
        Yaml::Boolean(bool) => bool.to_string(),
        Yaml::Array(yamls) => format!("[{}]", yamls.iter().map(to_string).collect::<Vec<_>>().join(", ")),
        Yaml::Hash(linked_hash_map) => {
            let relevant_entries: Vec<_> = linked_hash_map
                .iter()
                .filter(|(_, weight)| as_i64(weight).is_some_and(|weight| weight > 0))
                .map(|(yaml, weight)| {
                    let weight_string = to_string(weight);
                    if weight_string == "~" {
                        to_string(yaml)
                    } else {
                        format!("{}: {weight_string}", to_string(yaml))
                    }
                })
                .collect();

            if relevant_entries.is_empty() {
                format!("{{{}}}", linked_hash_map.keys().map(to_string).collect::<Vec<_>>().join(", "))
            } else if relevant_entries.len() == 1 {
                if let Some((value, _)) = relevant_entries[0].split_once(':') {
                    String::from(value)
                } else {
                    String::from("none")
                }
            } else {
                format!("{{{}}}", relevant_entries.join(", "))
            }
        }

        Yaml::Alias(_) => String::from("Unknown"),
        Yaml::Null => String::from("~"),
        Yaml::BadValue => String::from("Invalid"),
    }
}

pub fn as_i64(yaml: &Yaml) -> Option<i64> {
    if let Some(value) = yaml.as_i64() {
        return Some(value);
    }

    if let Some(value) = yaml.as_f64() {
        return Some(value as i64);
    }

    None
}
