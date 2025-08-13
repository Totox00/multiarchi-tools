use hashlink::LinkedHashMap;
use yaml_rust2::Yaml;

use crate::util::{as_i64, resolve_weighted_option};

pub fn handle_special(doc: &mut Yaml, game: &Yaml, name: &str) -> Vec<String> {
    let mut notes = vec![];

    let hash = if let Some(hash) = doc.as_mut_hash() {
        hash
    } else {
        return notes;
    };

    let game_hash = if let Some(game_options) = hash.get_mut(game) {
        if let Some(game_hash) = game_options.as_mut_hash() {
            game_hash
        } else {
            return notes;
        }
    } else {
        return notes;
    };

    match game.as_str() {
        Some("OpenRCT2") => {
            println!("'{name}.yaml' contains an OpenRCT2");
            game_hash.insert(Yaml::from_str("scenario"), Yaml::from_str("To review"));
        }
        Some("Stardew Valley") => {
            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("goal")) {
                move_option_weight(goal, "Perfection", "random");
            }

            if option_can_be(game_hash, "goal", &Yaml::from_str("random"), &Yaml::from_str("allsanity")) {
                println!("'{name}.yaml' has goal 'allsanity'");
            }

            if let Some(entrance_randomization) = game_hash.get_mut(&Yaml::from_str("entrance_randomization")) {
                move_option_weight(entrance_randomization, "chaos", "disabled");
            }

            change_option_name(game_hash, "trap_items", "trap_difficulty");
            push_value_or_default(&mut notes, game_hash, "mods", "[]");
        }
        Some("osu!") => {
            game_hash.insert(Yaml::from_str("explicit_lyrics"), Yaml::Boolean(false));

            push_value_or_default(&mut notes, game_hash, "minimum_grade", "off");
            push_value_or_default(&mut notes, game_hash, "disable_difficulty_reduction", "false");
            if option_can_be(game_hash, "exclude_standard", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                push_value_or_default(&mut notes, game_hash, "minimum_difficulty_standard", "0");
                push_value_or_default(&mut notes, game_hash, "maximum_difficulty_standard", "1000");
            }
            if option_can_be(game_hash, "exclude_catch", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                push_value_or_default(&mut notes, game_hash, "minimum_difficulty_catch", "0");
                push_value_or_default(&mut notes, game_hash, "maximum_difficulty_catch", "1000");
            }
            if option_can_be(game_hash, "exclude_taiko", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                push_value_or_default(&mut notes, game_hash, "minimum_difficulty_taiko", "0");
                push_value_or_default(&mut notes, game_hash, "maximum_difficulty_taiko", "1000");
            }
            if option_can_be(game_hash, "exclude_4k", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                push_value_or_default(&mut notes, game_hash, "minimum_difficulty_4k", "0");
                push_value_or_default(&mut notes, game_hash, "maximum_difficulty_4k", "1000");
            }
            if option_can_be(game_hash, "exclude_7k", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                push_value_or_default(&mut notes, game_hash, "minimum_difficulty_7k", "0");
                push_value_or_default(&mut notes, game_hash, "maximum_difficulty_7k", "1000");
            }
            if option_can_be(game_hash, "exclude_other_keys", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                push_value_or_default(&mut notes, game_hash, "minimum_difficulty_other_keys", "0");
                push_value_or_default(&mut notes, game_hash, "maximum_difficulty_other_keys", "1000");
            }
        }
        Some("Keymaster's Keep") => {
            game_hash.insert(Yaml::from_str("include_adult_only_or_unrated_games"), Yaml::Boolean(false));
        }
        Some("Overcooked! 2") => {
            if let Some(star_threshold_scale) = game_hash.get_mut(&Yaml::from_str("star_threshold_scale")) {
                move_option_weight_matches(star_threshold_scale, |yaml| as_i64(yaml).is_some_and(|v| v >= 90), "random-range-50-90");
            }

            push_value_or_default(&mut notes, game_hash, "include_dlcs", "['Story', 'Seasonal']");
            push_value_or_default(&mut notes, game_hash, "ramp_tricks", "false");
        }
        Some("Blasphemous") => push_value_or_default(&mut notes, game_hash, "Difficulty", "normal"),
        Some("Bomb Rush Cyberfunk") => push_value_or_default(&mut notes, game_hash, "logic", "glitchless"),
        Some("Celeste 64") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "Standard"),
        Some("Dark Souls III") => push_value_or_default(&mut notes, game_hash, "enable_dlc", "false"),
        Some("DLCQuest") => push_value_or_default(&mut notes, game_hash, "double_jump_glitch", "none"),
        Some("DOOM 1993") => push_value_or_default(&mut notes, game_hash, "pro", "false"),
        Some("DOOM II") => push_value_or_default(&mut notes, game_hash, "pro", "false"),
        Some("Final Fantasy") => push_value_or_default(&mut notes, game_hash, "permalink", "N/A"),
        Some("Final Fantasy Mystic Quest") => push_value_or_default(&mut notes, game_hash, "logic", "standard"),
        Some("A Hat in Time") => {
            push_value_or_default(&mut notes, game_hash, "LogicDifficulty", "normal");
            push_value_or_default(&mut notes, game_hash, "EnableDLC1", "false");
            push_value_or_default(&mut notes, game_hash, "EnableDLC2", "false");
            push_value_or_default(&mut notes, game_hash, "DWEnableBonus", "false");
            push_value_or_default(&mut notes, game_hash, "DWExcludeAnnoyingContracts", "true");
            if option_can_be(game_hash, "DWShuffle", &Yaml::Boolean(false), &Yaml::Boolean(true)) && option_can_be(game_hash, "DWEnableBonus", &Yaml::Boolean(false), &Yaml::Boolean(true)) {
                push_value_or_default(&mut notes, game_hash, "DWExcludeAnnoyingBonuses", "true");
            }
        }
        Some("Heretic") => push_value_or_default(&mut notes, game_hash, "pro", "false"),
        Some("Hollow Knight") => {
            let skips: Vec<_> = [
                "PreciseMovement",
                "ProficientCombat",
                "BackgroundObjectPogos",
                "EnemyPogos",
                "ObscureSkips",
                "ShadeSkips",
                "InfectionSkips",
                "FireballSkips",
                "SpikeTunnels",
                "AcidSkips",
                "DamageBoosts",
                "DangerousSkips",
                "DarkRooms",
                "ComplexSkips",
                "DifficultSkips",
            ]
            .iter()
            .filter(|option| option_can_be(game_hash, option, &Yaml::Boolean(false), &Yaml::Boolean(true)))
            .copied()
            .collect();

            if skips.is_empty() {
                notes.push(String::from("Skips: none"));
            } else {
                notes.push(format!("Skips: [{}]", skips.join(", ")));
            }
        }
        Some("Kingdom Hearts 2") => push_value_or_default(&mut notes, game_hash, "FightLogic", "normal"),
        Some("A Link to the Past") => push_value_or_default(&mut notes, game_hash, "glitches_required", "no_glitches"),
        Some("Links Awakening DX") => push_value_or_default(&mut notes, game_hash, "logic", "normal"),
        Some("Mario & Luigi Superstar Saga") => {
            game_hash.remove(&Yaml::from_str("harhall_pants"));
            push_value_or_default(&mut notes, game_hash, "difficult_logic", "FALSE");
        }
        Some("The Messenger") => push_value_or_default(&mut notes, game_hash, "logic_level", "normal"),
        Some("Muse Dash") => push_value_or_default(&mut notes, game_hash, "dlc_packs", "[]"),
        Some("Ocarina of Time") => {
            push_value_or_default(&mut notes, game_hash, "logic_rules", "glitchless");
            push_value_or_default(&mut notes, game_hash, "logic_tricks", "[]")
        }
        Some("Pokemon Red and Blue") => {
            if option_can_be(game_hash, "game_version", &Yaml::from_str("random"), &Yaml::from_str("random")) || game_hash.get(&Yaml::from_str("game_version")).is_some_and(|value| value.is_hash()) {
                let mut new_hash = LinkedHashMap::new();
                new_hash.insert(Yaml::from_str("red"), Yaml::Integer(50));
                new_hash.insert(Yaml::from_str("blue"), Yaml::Integer(50));
                game_hash.insert(Yaml::from_str("game_version"), Yaml::Hash(new_hash));
            }

            resolve_weighted_option(game_hash, "game_version");

            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("require_itemfinder")) {
                move_option_weight(goal, "true", "hard_required");
                move_option_weight(goal, "false", "not_required");
            }

            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("randomize_wilds")) {
                move_option_weight(goal, "true", "completely_random");
                move_option_weight(goal, "false", "vanilla");
            }

            push_value_or_default(&mut notes, game_hash, "game_version", "N/A");

            if option_can_be_other_than(game_hash, "trainer_name", &Yaml::from_str("choose_in_game"), &Yaml::from_str("choose_in_game")) {
                println!("'{name}.yaml' contains a chosen trainer name");
            }
        }
        Some("Risk of Rain 2") => push_value_or_default(&mut notes, game_hash, "dlc_sotv", "false"),
        Some("A Short Hike") => push_value_or_default(&mut notes, game_hash, "golden_feather_progression", "normal"),
        Some("SMZ3") => push_value_or_default(&mut notes, game_hash, "sm_logic", "normal"),
        Some("Sonic Adventure 2 Battle") => {
            push_value_or_default(&mut notes, game_hash, "logic_difficulty", "standard");
            push_value_or_default(&mut notes, game_hash, "chao_karate_difficulty", "none");
            push_value_or_default(&mut notes, game_hash, "sadx_music", "sa2b");
        }
        Some("Starcraft 2") => push_value_or_default(&mut notes, game_hash, "required_tactics", "standard"),
        Some("Super Metroid") => {
            push_value_or_default(&mut notes, game_hash, "preset", "regular");
            push_value_or_default(&mut notes, game_hash, "max_difficulty", "hardcore");
            if option_can_be(game_hash, "preset", &Yaml::from_str("regular"), &Yaml::from_str("varia_custom")) {
                push_value_or_default(&mut notes, game_hash, "varia_custom_preset", "N/A");
            }
        }
        Some("Terraria") => push_value_or_default(&mut notes, game_hash, "calamity", "false"),
        Some("TUNIC") => {
            push_value_or_default(&mut notes, game_hash, "combat_logic", "off");
            push_value_or_default(&mut notes, game_hash, "lanternless", "false");
            push_value_or_default(&mut notes, game_hash, "maskless", "false");
            push_value_or_default(&mut notes, game_hash, "laurels_zips", "false");
            push_value_or_default(&mut notes, game_hash, "ice_grappling", "off");
            push_value_or_default(&mut notes, game_hash, "ladder_storage", "off");
            push_value_or_default(&mut notes, game_hash, "ladder_storage_without_items", "off");
        }
        Some("The Wind Waker") => {
            push_value_or_default(&mut notes, game_hash, "logic_obscurity", "none");
            push_value_or_default(&mut notes, game_hash, "logic_precision", "none");
            push_value_or_default(&mut notes, game_hash, "enable_tuner_logic", "false");
        }
        Some("Yoshi's Island") => {
            push_value_or_default(&mut notes, game_hash, "stage_logic", "strict");
            push_value_or_default(&mut notes, game_hash, "item_logic", "false");
        }
        Some("A Link Between Worlds") => push_value_or_default(&mut notes, game_hash, "logic_mode", "normal"),
        Some("Banjo-Tooie") => {
            push_value_or_default(&mut notes, game_hash, "logic_type", "intended");
            change_option_name(game_hash, "game_length", "world_requirements");
        }
        Some("Duke Nukem 3D") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "medium"),
        Some("The Legend of Zelda - Oracle of Ages") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "casual"),
        Some("The Legend of Zelda - Oracle of Seasons") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "casual"),
        Some("Ori and the Blind Forest") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "casual"),
        Some("Ori and the Will of the Wisps") => push_value_or_default(&mut notes, game_hash, "difficulty", "moki"),
        Some("Outer Wilds") => {
            push_value_or_default(&mut notes, game_hash, "enable_eote_dlc", "false");
            push_value_or_default(&mut notes, game_hash, "enable_hn1_mod", "false");
            push_value_or_default(&mut notes, game_hash, "enable_outsider_mod", "false");
            push_value_or_default(&mut notes, game_hash, "enable_ac_mod", "false");
            push_value_or_default(&mut notes, game_hash, "enable_hn2_mod", "false");
            push_value_or_default(&mut notes, game_hash, "enable_fq_mod", "false");
        }
        Some("Pokemon FireRed and LeafGreen") => {
            if option_can_be(game_hash, "game_version", &Yaml::from_str("random"), &Yaml::from_str("random")) {
                let mut new_hash = LinkedHashMap::new();
                new_hash.insert(Yaml::from_str("firered"), Yaml::Integer(50));
                new_hash.insert(Yaml::from_str("leafgreen"), Yaml::Integer(50));
                game_hash.insert(Yaml::from_str("game_version"), Yaml::Hash(new_hash));
            }

            resolve_weighted_option(game_hash, "game_version");

            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("Goal")) {
                move_option_weight(goal, "elite_four", "champion");
                move_option_weight(goal, "elite_four_rematch", "champion_rematch");
            }

            if let Some(trainersanity) = game_hash.get_mut(&Yaml::from_str("trainersanity")) {
                move_option_weight(trainersanity, "true", "all");
                move_option_weight(trainersanity, "false", "off");
            }

            if let Some(provide_hints) = game_hash.get_mut(&Yaml::from_str("provide_hints")) {
                move_option_weight(provide_hints, "true", "456");
                move_option_weight(provide_hints, "false", "0");
            }

            if let Some(flash_required) = game_hash.get_mut(&Yaml::from_str("flash_required")) {
                move_option_weight(flash_required, "true", "required");
                move_option_weight(flash_required, "false", "off");
            }

            push_value_or_default(&mut notes, game_hash, "game_version", "N/A");
            push_value_or_default(&mut notes, game_hash, "evolutions_required", "[HM Requirement, Oak's Aides, Dexsanity]");
            push_value_or_default(
                &mut notes,
                game_hash,
                "evolution_methods_required",
                "[Level, Level Tyrogue, Level Wurmple, Evo Item, Evo & Held Item, Friendship]",
            );
        }
        Some("Pseudoregalia") => push_value_or_default(&mut notes, game_hash, "logic_level", "normal"),
        Some("Rusted Moss") => {
            push_value_or_default(&mut notes, game_hash, "damage_boost", "false");
            push_value_or_default(&mut notes, game_hash, "grenade_boost", "false");
            push_value_or_default(&mut notes, game_hash, "precise_movement", "false");
            push_value_or_default(&mut notes, game_hash, "precise_grapple", "true");
            push_value_or_default(&mut notes, game_hash, "bunny_hopping", "false");
            push_value_or_default(&mut notes, game_hash, "hard_combat", "false");
        }
        Some("Slay the Spire") => {
            push_value_or_default(&mut notes, game_hash, "downfall", "false");
            if option_can_be(game_hash, "use_advanced_characters", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                push_value_or_default(&mut notes, game_hash, "character", "[Ironclad]");
            }
            if option_can_be(game_hash, "use_advanced_characters", &Yaml::Boolean(false), &Yaml::Boolean(true)) {
                if let Some(advanced_characters_hash) = game_hash.get(&Yaml::from_str("advanced_characters")).and_then(Yaml::as_hash) {
                    notes.push(format!("advanced_characters: [{}]", advanced_characters_hash.keys().map(to_string).collect::<Vec<_>>().join(", ")));
                } else {
                    notes.push(String::from("advanced_characters: [ironclad]"));
                }
            }
        }
        Some("Super Metroid Map Rando") => push_value_or_default(&mut notes, game_hash, "preset", "hard"),
        Some("Sonic Adventure DX") => push_value_or_default(&mut notes, game_hash, "logic_level", "normal_logic"),
        Some("Tyrian") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "standard"),
        Some("ANIMAL WELL") => {
            push_value_or_default(&mut notes, game_hash, "tanking_damage", "false");
            push_value_or_default(&mut notes, game_hash, "bubble_jumping", "short_chains");
            push_value_or_default(&mut notes, game_hash, "disc_hopping", "off");
            push_value_or_default(&mut notes, game_hash, "wheel_tricks", "off");
            push_value_or_default(&mut notes, game_hash, "ball_throwing", "simple");
            push_value_or_default(&mut notes, game_hash, "flute_jumps", "false");
            push_value_or_default(&mut notes, game_hash, "obscure_tricks", "false");
            push_value_or_default(&mut notes, game_hash, "precise_tricks", "false");
        }
        Some("DORONKO WANKO") => push_value_or_default(&mut notes, game_hash, "logic", "standard"),
        Some("Minit") => {
            push_value_or_default(&mut notes, game_hash, "darkrooms", "minor");
            push_value_or_default(&mut notes, game_hash, "obscure", "false");
            push_value_or_default(&mut notes, game_hash, "damage_boosts", "false");
        }
        Some("Majora's Mask Recompiled") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "normal"),
        Some("Brotato") => push_value_or_default(&mut notes, game_hash, "enable_abyssal_terrors_dlc", "false"),
        Some("ULTRAKILL") => {
            if let Some(randomize_secondary_fire) = game_hash.get_mut(&Yaml::from_str("randomize_secondary_fire")) {
                move_option_weight(randomize_secondary_fire, "true", "split");
                move_option_weight(randomize_secondary_fire, "false", "disabled");
            }
        }
        Some("Against the Storm") => push_value_or_default(&mut notes, game_hash, "enable_dlc", "false"),
        Some("Guild Wars 2") => push_value_or_default(&mut notes, game_hash, "storyline", "core"),
        Some("Paper Mario") => {
            if let Some(super_multi_blocks) = game_hash.get_mut(&Yaml::from_str("super_multi_blocks")) {
                move_option_weight(super_multi_blocks, "true", "anywhere");
                move_option_weight(super_multi_blocks, "false", "off");
            }
        }
        _ => (),
    };

    notes
}

fn push_value_or_default(notes: &mut Vec<String>, hash: &LinkedHashMap<Yaml, Yaml>, key: &str, default: &str) {
    notes.push(format!("{key}: {}", get_value_or_default(hash, key, default)));
}

fn get_value_or_default(hash: &LinkedHashMap<Yaml, Yaml>, key: &str, default: &str) -> String {
    if let Some(value) = hash.get(&Yaml::from_str(key)) {
        to_string(value)
    } else {
        default.to_string()
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

fn option_can_be(hash: &LinkedHashMap<Yaml, Yaml>, key: &str, default: &Yaml, cmp: &Yaml) -> bool {
    if let Some(value) = hash.get(&Yaml::from_str(key)).cloned().map(handle_non_string_strings) {
        if value == *cmp {
            true
        } else if let Some(hash) = value.as_hash() {
            hash.iter()
                .any(|(value, weight)| handle_non_string_strings(value.clone()) == *cmp && as_i64(weight).is_some_and(|weight| weight > 0))
        } else {
            false
        }
    } else {
        default == cmp
    }
}

fn option_can_be_other_than(hash: &LinkedHashMap<Yaml, Yaml>, key: &str, default: &Yaml, cmp: &Yaml) -> bool {
    if let Some(value) = hash.get(&Yaml::from_str(key)).cloned().map(handle_non_string_strings) {
        if let Some(hash) = value.as_hash() {
            hash.iter()
                .any(|(value, weight)| handle_non_string_strings(value.clone()) != *cmp && as_i64(weight).is_some_and(|weight| weight > 0))
        } else {
            value != *cmp
        }
    } else {
        default != cmp
    }
}

fn handle_non_string_strings(yaml: Yaml) -> Yaml {
    if let Some(str) = yaml.as_str() {
        match str.to_lowercase().as_str() {
            "true" => Yaml::Boolean(true),
            "false" => Yaml::Boolean(false),
            _ => yaml,
        }
    } else {
        yaml
    }
}

fn change_option_name(hash: &mut LinkedHashMap<Yaml, Yaml>, old_name: &str, new_name: &str) {
    if let Some(trap_items) = hash.remove(&Yaml::from_str(old_name)) {
        hash.insert(Yaml::from_str(new_name), trap_items);
    }
}

fn move_option_weight_matches<T: Fn(&Yaml) -> bool>(value: &mut Yaml, from: T, to: &str) {
    let to = Yaml::from_str(to);

    if from(value) {
        *value = to;
    } else if let Some(value_hash) = value.as_mut_hash() {
        let mut new_weight = value_hash.get(&to).and_then(as_i64).unwrap_or(0);
        new_weight += value_hash.iter().filter(|(value, _)| from(value)).map(|(_, weight)| as_i64(weight).unwrap_or(0)).sum::<i64>();
        value_hash.retain(|value, _| !from(value));
        if new_weight > 0 {
            value_hash.insert(to, Yaml::Integer(new_weight));
        }
    }
}

fn move_option_weight(value: &mut Yaml, from: &str, to: &str) {
    let from = Yaml::from_str(from);
    let to = Yaml::from_str(to);

    if *value == from {
        *value = to;
    } else if let Some(hash) = value.as_mut_hash() {
        if let Some(weight) = hash.get(&from) {
            if let Some(weight) = as_i64(weight) {
                if weight > 0 {
                    hash.remove(&from);
                    if let Some(existing_weight) = hash.get_mut(&to) {
                        if let Some(existing_value) = existing_weight.as_i64() {
                            *existing_weight = Yaml::Integer(existing_value + weight);
                        } else if let Some(existing_value) = existing_weight.as_f64() {
                            *existing_weight = Yaml::Integer(existing_value as i64 + weight);
                        } else {
                            hash.insert(to, Yaml::Integer(weight));
                        }
                    } else {
                        hash.insert(to, Yaml::Integer(weight));
                    }
                }
            }
        }
    }
}
