use std::collections::HashMap;

use hashlink::LinkedHashMap;
use yaml_rust2::Yaml;

const MAPPING_DATA: &str = include_str!("name_mapping.tsv");

pub type Mapping = HashMap<String, (HashMap<String, String>, HashMap<String, String>)>;

pub fn load_name_mapping() -> Mapping {
    let mut game = "";
    let mut items = false;
    let mut locations = false;
    let mut mapping = HashMap::new();
    let mut game_mappings = None;

    for (i, line) in MAPPING_DATA.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if let Some(("", rest)) = line.split_once("game ") {
            if let Some(game_mappings) = game_mappings {
                mapping.insert(game.to_string(), game_mappings);
            }

            items = false;
            locations = false;
            game = rest;
            game_mappings = Some((HashMap::new(), HashMap::new()));
        } else if line == "items" {
            items = true;
        } else if line == "locations" {
            locations = true;
        } else if let Some(("", rest)) = line.split_once("addexact ") {
            if let Some(game_mappings) = &mut game_mappings {
                if let Some((old, new)) = rest.split_once('\t') {
                    if items {
                        game_mappings.0.insert(old.to_string(), new.to_string());
                    }
                    if locations {
                        game_mappings.1.insert(old.to_string(), new.to_string());
                    }
                }
            } else {
                panic!("Cannot add item without set game at line {i}");
            }
        }
        if let Some(game_mappings) = &mut game_mappings {
            if let Some((old, new)) = line.split_once('\t') {
                if items {
                    game_mappings.0.insert(old.to_string(), new.to_string());
                }
                if locations {
                    game_mappings.1.insert(old.to_string(), new.to_string());
                }
            }
        } else {
            panic!("Cannot add item without set game at line {i}");
        }
    }

    if let Some(game_mappings) = game_mappings {
        mapping.insert(game.to_string(), game_mappings);
    }

    mapping
}

pub fn remap_common_options(mapping: &Mapping, doc: &mut Yaml, game: &str) {
    let Some(game_hash) = doc.as_mut_hash().and_then(|hash| hash.get_mut(&Yaml::from_str(game))).and_then(|yaml| yaml.as_mut_hash()) else {
        return;
    };

    let Some((item_mapping, location_mapping)) = mapping.get(game) else {
        return;
    };

    remap_list(item_mapping, game_hash, "local_items");
    remap_list(item_mapping, game_hash, "non_local_items");
    remap_hash(item_mapping, game_hash, "start_inventory");
    remap_list(item_mapping, game_hash, "start_hints");
    remap_list(location_mapping, game_hash, "start_location_hints");
    remap_list(location_mapping, game_hash, "exclude_locations");
    remap_list(location_mapping, game_hash, "priority_locations");

    if let Some(plando_entries) = game_hash.get_mut(&Yaml::from_str("plando_items")).and_then(|yaml| yaml.as_mut_vec()) {
        let item_key = Yaml::from_str("item");
        let location_key = Yaml::from_str("location");

        for entry in plando_entries {
            if let Some(plando_hash) = entry.as_mut_hash() {
                remap_list(location_mapping, plando_hash, "locations");
                remap_hash(item_mapping, plando_hash, "items");

                if let Some(item) = plando_hash.get_mut(&item_key)
                    && let Some(old) = item.as_str()
                    && let Some(new) = item_mapping.get(old)
                {
                    *item = Yaml::from_str(new);
                }

                if let Some(location) = plando_hash.get_mut(&location_key)
                    && let Some(old) = location.as_str()
                    && let Some(new) = location_mapping.get(old)
                {
                    *location = Yaml::from_str(new);
                }
            }
        }
    }
}

fn remap_list(mapping: &HashMap<String, String>, hash: &mut LinkedHashMap<Yaml, Yaml>, option: &str) {
    if let Some(option_vec) = hash.get_mut(&Yaml::from_str(option)).and_then(|yaml| yaml.as_mut_vec()) {
        for entry in option_vec {
            if let Some(old) = entry.as_str()
                && let Some(new) = mapping.get(old)
            {
                *entry = Yaml::from_str(new);
            }
        }
    }
}

fn remap_hash(mapping: &HashMap<String, String>, hash: &mut LinkedHashMap<Yaml, Yaml>, option: &str) {
    if let Some(option_hash) = hash.get_mut(&Yaml::from_str(option)).and_then(|yaml| yaml.as_mut_hash()) {
        let mut to_map = vec![];

        for old in option_hash.keys().filter_map(|yaml| yaml.as_str()) {
            if let Some(new) = mapping.get(old) {
                to_map.push((old.to_string(), new));
            }
        }

        for (old, new) in to_map {
            if let Some(val) = option_hash.remove(&Yaml::String(old)) {
                option_hash.insert(Yaml::from_str(new), val);
            }
        }
    }
}
