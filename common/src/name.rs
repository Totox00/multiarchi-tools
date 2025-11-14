use std::collections::HashMap;

use yaml_rust2::Yaml;

pub fn set_name(doc: &mut Yaml, name: &str, game: Option<&Yaml>) -> Option<Yaml> {
    let name_key = Yaml::from_str("name");
    let triggers_key = Yaml::from_str("triggers");
    let name_value = Yaml::from_str(name);

    let hash = doc.as_mut_hash()?;

    if let Some(game) = game
        && let Some(game_options) = hash.get_mut(game)
        && let Some(game_options_hash) = game_options.as_mut_hash()
        && let Some(triggers) = game_options_hash.get_mut(&triggers_key)
    {
        strip_name_changes_from_triggers(triggers);
    }

    if let Some(triggers) = hash.get_mut(&triggers_key) {
        strip_name_changes_from_triggers(triggers);
    }

    if let Some(name_entry) = hash.get_mut(&name_key) {
        let old = name_entry.clone();
        *name_entry = name_value;
        Some(old)
    } else {
        hash.insert(name_key, name_value)
    }
}

pub fn strip_name_changes_from_triggers(doc: &mut Yaml) {
    let name_key = Yaml::from_str("name");
    let options_key = Yaml::from_str("options");
    let null_key = Yaml::from_str("null");
    let root_key = Yaml::from_str("~");
    let remove_key = Yaml::from_str("_remove_");

    if let Some(vec) = doc.as_mut_vec() {
        for trigger in vec.iter_mut() {
            if let Some(trigger_hash) = trigger.as_mut_hash()
                && let Some(options) = trigger_hash.get_mut(&options_key)
                && let Some(options_hash) = options.as_mut_hash()
            {
                let mut remove_null_hash = false;
                if let Some(null) = options_hash.get_mut(&null_key)
                    && let Some(null_hash) = null.as_mut_hash()
                    && null_hash.remove(&name_key).is_some()
                    && null_hash.is_empty()
                {
                    remove_null_hash = true;
                }
                let mut remove_root_hash = false;
                if let Some(root) = options_hash.get_mut(&root_key)
                    && let Some(root_hash) = root.as_mut_hash()
                    && root_hash.remove(&name_key).is_some()
                    && root_hash.is_empty()
                {
                    remove_root_hash = true;
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

        vec.retain(|trigger| {
            if let Some(trigger_hash) = trigger.as_hash() {
                !trigger_hash.contains_key(&remove_key)
            } else {
                true
            }
        });
    }
}

pub fn rename_plando_worlds(mapping: &HashMap<Yaml, Yaml>, docs: &mut [Yaml]) {
    let game_key = Yaml::from_str("game");
    let plando_key = Yaml::from_str("plando_items");
    let world_key = Yaml::from_str("world");

    for doc in docs {
        if let Some(hash) = doc.as_mut_hash()
            && let Some(options_key) = hash.get_mut(&game_key)
            && let Some(plando_items) = options_key.as_mut_hash().and_then(|hash| hash.get_mut(&plando_key)).and_then(|plando_items| plando_items.as_mut_vec())
        {
            for plando_block in plando_items {
                if let Some(world) = plando_block.as_mut_hash().and_then(|hash| hash.get_mut(&world_key)) {
                    if let Some(new_name) = mapping.get(world) {
                        *world = new_name.clone();
                    } else if let Some(worlds) = world.as_mut_vec() {
                        for world in worlds {
                            if let Some(new_name) = mapping.get(world) {
                                *world = new_name.clone();
                            }
                        }
                    }
                }
            }
        }
    }
}
