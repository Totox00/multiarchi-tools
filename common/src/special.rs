use hashlink::LinkedHashMap;
use phf::phf_map;
use serde_json::Value;
use yaml_rust2::Yaml;

use crate::util::{as_i64, resolve_weighted_option};

const ARCHIPELA_GO_DISTANCES: phf::Map<&'static str, i64> = phf_map!(
    "2k" => 2000,
    "5k" => 5000,
    "10k" => 10000,
    "half_marathon" => 21098,
    "marathon" => 42195,
    "50k" => 50000,
    "50_miler" => 80467,
    "100k" => 100000,
    "100_miler" => 160934,
);

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
            println!(
                "'{name}.yaml' contains an OpenRCT2 with scenario: {}",
                get_value_or_default(game_hash, "scenario", "archipelago_madness_vanilla")
            );
        }
        Some("Stardew Valley") => {
            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("goal")) {
                move_option_weight(goal, "perfection", "random");
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

            if let Some(minimum_difficulty) = game_hash.remove(&Yaml::from_str("minimum_difficulty")) {
                game_hash.insert(Yaml::from_str("minimum_difficulty_standard"), minimum_difficulty.clone());
                game_hash.insert(Yaml::from_str("minimum_difficulty_catch"), minimum_difficulty.clone());
                game_hash.insert(Yaml::from_str("minimum_difficulty_taiko"), minimum_difficulty.clone());
                game_hash.insert(Yaml::from_str("minimum_difficulty_4k"), minimum_difficulty.clone());
                game_hash.insert(Yaml::from_str("minimum_difficulty_7k"), minimum_difficulty.clone());
                game_hash.insert(Yaml::from_str("minimum_difficulty_other_keys"), minimum_difficulty);
            }

            if let Some(maximum_difficulty) = game_hash.remove(&Yaml::from_str("maximum_difficulty")) {
                game_hash.insert(Yaml::from_str("maximum_difficulty_standard"), maximum_difficulty.clone());
                game_hash.insert(Yaml::from_str("maximum_difficulty_catch"), maximum_difficulty.clone());
                game_hash.insert(Yaml::from_str("maximum_difficulty_taiko"), maximum_difficulty.clone());
                game_hash.insert(Yaml::from_str("maximum_difficulty_4k"), maximum_difficulty.clone());
                game_hash.insert(Yaml::from_str("maximum_difficulty_7k"), maximum_difficulty.clone());
                game_hash.insert(Yaml::from_str("maximum_difficulty_other_keys"), maximum_difficulty);
            }

            push_value_or_default(&mut notes, game_hash, "minimum_grade", "off");
            push_value_or_default(&mut notes, game_hash, "disable_difficulty_reduction", "false");
            if option_can_be(game_hash, "exclude_standard", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                notes.push(format!(
                    "standard: {}-{}",
                    get_value_or_default(game_hash, "minimum_difficulty_standard", "0"),
                    get_value_or_default(game_hash, "maximum_difficulty_standard", "1000")
                ));
            }
            if option_can_be(game_hash, "exclude_catch", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                notes.push(format!(
                    "catch: {}-{}",
                    get_value_or_default(game_hash, "minimum_difficulty_catch", "0"),
                    get_value_or_default(game_hash, "maximum_difficulty_catch", "1000")
                ));
            }
            if option_can_be(game_hash, "exclude_taiko", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                notes.push(format!(
                    "taiko: {}-{}",
                    get_value_or_default(game_hash, "minimum_difficulty_taiko", "0"),
                    get_value_or_default(game_hash, "maximum_difficulty_taiko", "1000")
                ));
            }
            if option_can_be(game_hash, "exclude_4k", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                notes.push(format!(
                    "4k: {}-{}",
                    get_value_or_default(game_hash, "minimum_difficulty_4k", "0"),
                    get_value_or_default(game_hash, "maximum_difficulty_4k", "1000")
                ));
            }
            if option_can_be(game_hash, "exclude_7k", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                notes.push(format!(
                    "7k: {}-{}",
                    get_value_or_default(game_hash, "minimum_difficulty_7k", "0"),
                    get_value_or_default(game_hash, "maximum_difficulty_7k", "1000")
                ));
            }
            if option_can_be(game_hash, "exclude_other_keys", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                notes.push(format!(
                    "other_keys: {}-{}",
                    get_value_or_default(game_hash, "minimum_difficulty_other_keys", "0"),
                    get_value_or_default(game_hash, "maximum_difficulty_other_keys", "1000")
                ));
            }
        }
        Some("Keymaster's Keep") => {
            if option_can_be(game_hash, "include_adult_only_or_unrated_games", &Yaml::Boolean(false), &Yaml::Boolean(true)) {
                println!("'{name}.yaml' may have adult only or unrated games in Keymaster's Keep");
            }

            game_hash.insert(Yaml::from_str("include_adult_only_or_unrated_games"), Yaml::Boolean(false));
        }
        Some("Overcooked! 2") => {
            if let Some(star_threshold_scale) = game_hash.get_mut(&Yaml::from_str("star_threshold_scale")) {
                move_option_weight_matches(star_threshold_scale, |yaml| as_i64(yaml).is_some_and(|v| v >= 90), "random-range-50-90");
            }

            push_value_or_default(&mut notes, game_hash, "include_dlcs", "['Story', 'Seasonal']");
            push_value_or_default(&mut notes, game_hash, "ramp_tricks", "false");
        }
        Some("Blasphemous") => push_value_or_default(&mut notes, game_hash, "difficulty", "normal"),
        Some("Bomb Rush Cyberfunk") => push_value_or_default(&mut notes, game_hash, "logic", "glitchless"),
        Some("Celeste 64") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "Standard"),
        Some("Dark Souls III") => push_value_or_default(&mut notes, game_hash, "enable_dlc", "false"),
        Some("DLCQuest") => push_value_or_default(&mut notes, game_hash, "double_jump_glitch", "none"),
        Some("DOOM 1993") => push_value_or_default(&mut notes, game_hash, "pro", "false"),
        Some("DOOM II") => push_value_or_default(&mut notes, game_hash, "pro", "false"),
        Some("Final Fantasy") => push_value_or_default(&mut notes, game_hash, "permalink", "N/A"),
        Some("Final Fantasy Mystic Quest") => push_value_or_default(&mut notes, game_hash, "logic", "standard"),
        Some("Final Fantasy 12 Open World") => push_value_or_default(&mut notes, game_hash, "character_progression_scaling", "true"),
        Some("A Hat in Time") => {
            push_value_or_default(&mut notes, game_hash, "LogicDifficulty", "normal");

            if option_can_be(game_hash, "EnableDeathWish", &Yaml::Boolean(false), &Yaml::Boolean(true)) && !option_can_be(game_hash, "EnableDLC1", &Yaml::Boolean(false), &Yaml::Boolean(true)) {
                notes.push(String::from("EnableDLC1: deathwishonly"));
            } else {
                push_value_or_default(&mut notes, game_hash, "EnableDLC1", "false");
            }

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
        Some("Kingdom Hearts") => {
            if let Some(cups) = game_hash.get_mut(&Yaml::from_str("cups")) {
                move_option_weight(cups, "false", "off");
                move_option_weight(cups, "true", "cups");
            }
        }
        Some("A Link to the Past") => push_value_or_default(&mut notes, game_hash, "glitches_required", "no_glitches"),
        Some("Links Awakening DX") => push_value_or_default(&mut notes, game_hash, "logic", "normal"),
        Some("Mario & Luigi Superstar Saga") => {
            game_hash.remove(&Yaml::from_str("harhall_pants"));
            push_value_or_default(&mut notes, game_hash, "difficult_logic", "FALSE");
        }
        Some("The Messenger") => push_value_or_default(&mut notes, game_hash, "logic_level", "normal"),
        Some("Muse Dash") => {
            push_value_or_default(&mut notes, game_hash, "dlc_packs", "[]");
            game_hash.remove(&Yaml::from_str("available_trap_types"));
        }
        Some("Ocarina of Time") => {
            push_value_or_default(&mut notes, game_hash, "logic_rules", "glitchless");
            push_value_or_default(&mut notes, game_hash, "logic_tricks", "[]")
        }
        Some("Pokemon Red and Blue") => {
            resolve_weighted_option(game_hash, "game_version");

            if option_can_be(game_hash, "game_version", &Yaml::from_str("random"), &Yaml::from_str("random")) {
                let mut new_hash = LinkedHashMap::new();
                new_hash.insert(Yaml::from_str("red"), Yaml::Integer(50));
                new_hash.insert(Yaml::from_str("blue"), Yaml::Integer(50));
                game_hash.insert(Yaml::from_str("game_version"), Yaml::Hash(new_hash));
            }

            resolve_weighted_option(game_hash, "game_version");

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
        Some("Starcraft 2") => {
            if let Some(mission_order) = game_hash.get_mut(&Yaml::from_str("mission_order")) {
                move_option_weight(mission_order, "tiny_grid", "grid");
                move_option_weight(mission_order, "mini_grid", "grid");
                move_option_weight(mission_order, "medium_grid", "grid");
                move_option_weight(mission_order, "mini_gauntlet", "gauntlet");
            }

            if let Some(kerrigan_presence) = game_hash.get_mut(&Yaml::from_str("kerrigan_presence")) {
                move_option_weight(kerrigan_presence, "not_present_and_no_passives", "not_present");
                move_option_weight(kerrigan_presence, "kerrigan_max_passive_abilities", "0");
            }

            if let Some(spear_of_adun_presence) = game_hash.get_mut(&Yaml::from_str("spear_of_adun_presence")) {
                move_option_weight(spear_of_adun_presence, "lotv_protoss", "vanilla");
            }

            if let Some(grant_story_tech) = game_hash.get_mut(&Yaml::from_str("grant_story_tech")) {
                move_option_weight(grant_story_tech, "false", "no_grant");
                move_option_weight(grant_story_tech, "true", "grant");
            }

            if let Some(vanilla_locations) = game_hash.get_mut(&Yaml::from_str("vanilla_locations")) {
                move_option_weight(vanilla_locations, "resources", "filler");
            }

            if let Some(extra_locations) = game_hash.get_mut(&Yaml::from_str("extra_locations")) {
                move_option_weight(extra_locations, "resources", "filler");
            }

            if let Some(challenge_locations) = game_hash.get_mut(&Yaml::from_str("challenge_locations")) {
                move_option_weight(challenge_locations, "resources", "filler");
            }

            if let Some(mastery_locations) = game_hash.get_mut(&Yaml::from_str("mastery_locations")) {
                move_option_weight(mastery_locations, "resources", "filler");
            }

            let key = Yaml::from_str("enabled_campaigns");
            if !game_hash.contains_key(&key) {
                game_hash.insert(
                    key,
                    Yaml::Array(
                        [
                            ("enable_wol_missions", "Wings of Liberty"),
                            ("enable_prophecy_missions", "Prophecy"),
                            ("enable_hots_missions", "Heart of the Swarm"),
                            ("enable_lotv_prologue_missions", "Whispers of Oblivion (Legacy of the Void: Prologue)"),
                            ("enable_lotv_missions", "Legacy of the Void"),
                            ("enable_epilogue_missions", "Into the Void (Legacy of the Void: Epilogue)"),
                            ("enable_nco_missions", "Nova Covert Ops"),
                        ]
                        .into_iter()
                        .filter(|(option_name, _)| option_can_be(game_hash, option_name, &Yaml::Boolean(true), &Yaml::Boolean(true)))
                        .map(|(_, campaign_name)| Yaml::from_str(campaign_name))
                        .collect(),
                    ),
                );
            }

            if let Some(grid_two_start_positions) = game_hash.remove(&Yaml::from_str("grid_two_start_positions")) {
                game_hash.insert(Yaml::from_str("two_start_positions"), grid_two_start_positions);
            }

            push_value_or_default(&mut notes, game_hash, "required_tactics", "standard");
        }
        Some("Super Metroid") => {
            push_value_or_default(&mut notes, game_hash, "preset", "regular");
            push_value_or_default(&mut notes, game_hash, "max_difficulty", "hardcore");
            if option_can_be(game_hash, "preset", &Yaml::from_str("regular"), &Yaml::from_str("varia_custom")) {
                push_value_or_default(&mut notes, game_hash, "varia_custom_preset", "N/A");
            }
        }
        Some("Terraria") => {
            if option_can_be(game_hash, "goal", &Yaml::Null, &Yaml::from_str("boss_rush")) {
                notes.push(String::from("calamity: true"));
            } else {
                push_value_or_default(&mut notes, game_hash, "calamity", "false")
            };
        }
        Some("TUNIC") => {
            game_hash.remove(&Yaml::from_str("logic_rules"));

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
            if let Some(randomize_world_entrance_loading_zone) = game_hash.remove(&Yaml::from_str("randomize_world_entrance_loading_zone")) {
                game_hash.insert(Yaml::from_str("randomize_world_entrance_loading_zones"), randomize_world_entrance_loading_zone);
            }

            if let Some(randomize_boss_loading_zone) = game_hash.remove(&Yaml::from_str("randomize_boss_loading_zone")) {
                game_hash.insert(Yaml::from_str("randomize_boss_loading_zones"), randomize_boss_loading_zone);
            }

            push_value_or_default(&mut notes, game_hash, "logic_type", "intended");
            change_option_name(game_hash, "game_length", "world_requirements");
            if let Some(open_silos) = game_hash.get_mut(&Yaml::from_str("open_silos")) {
                move_option_weight(open_silos, "none", "1");
                move_option_weight(open_silos, "one", "2");
                move_option_weight(open_silos, "all", "7");
            }
        }
        Some("Duke Nukem 3D") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "medium"),
        Some("The Legend of Zelda - Oracle of Ages") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "casual"),
        Some("The Legend of Zelda - Oracle of Seasons") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "casual"),
        Some("Ori and the Blind Forest") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "casual"),
        Some("Ori and the Will of the Wisps") => push_value_or_default(&mut notes, game_hash, "difficulty", "moki"),
        Some("Outer Wilds") => {
            game_hash.insert(Yaml::from_str("enable_hn2_mod"), Yaml::Boolean(false));

            push_value_or_default(&mut notes, game_hash, "enable_eote_dlc", "false");
            push_value_or_default(&mut notes, game_hash, "enable_hn1_mod", "false");
            push_value_or_default(&mut notes, game_hash, "enable_outsider_mod", "false");
            push_value_or_default(&mut notes, game_hash, "enable_ac_mod", "false");
            push_value_or_default(&mut notes, game_hash, "enable_hn2_mod", "false");
            push_value_or_default(&mut notes, game_hash, "enable_fq_mod", "false");
        }
        Some("Pokemon FireRed and LeafGreen") => {
            resolve_weighted_option(game_hash, "game_version");

            if option_can_be(game_hash, "game_version", &Yaml::from_str("random"), &Yaml::from_str("random")) {
                let mut new_hash = LinkedHashMap::new();
                new_hash.insert(Yaml::from_str("firered"), Yaml::Integer(50));
                new_hash.insert(Yaml::from_str("leafgreen"), Yaml::Integer(50));
                game_hash.insert(Yaml::from_str("game_version"), Yaml::Hash(new_hash));
            }

            game_hash.remove(&Yaml::from_str("shop_prices"));

            resolve_weighted_option(game_hash, "game_version");

            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("goal")) {
                move_option_weight(goal, "elite_four", "champion");
                move_option_weight(goal, "elite_four_rematch", "champion_rematch");
            }

            if let Some(trainersanity) = game_hash.get_mut(&Yaml::from_str("trainersanity")) {
                move_option_weight(trainersanity, "true", "456");
                move_option_weight(trainersanity, "false", "0");
            }

            if let Some(provide_hints) = game_hash.get_mut(&Yaml::from_str("provide_hints")) {
                move_option_weight(provide_hints, "true", "all");
                move_option_weight(provide_hints, "false", "off");
            }

            if let Some(flash_required) = game_hash.get_mut(&Yaml::from_str("flash_required")) {
                move_option_weight(flash_required, "true", "required");
                move_option_weight(flash_required, "false", "off");
            }

            if let Some(randomize_fly_destinations) = game_hash.get_mut(&Yaml::from_str("randomize_fly_destinations")) {
                move_option_weight(randomize_fly_destinations, "true", "completely_random");
                move_option_weight(randomize_fly_destinations, "false", "off");
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
            if let Some(character) = game_hash.remove(&Yaml::from_str("character")) {
                game_hash.insert(Yaml::from_str("characters"), character);
            }
            push_value_or_default(&mut notes, game_hash, "downfall", "false");
            if option_can_be(game_hash, "use_advanced_characters", &Yaml::Boolean(false), &Yaml::Boolean(false)) {
                push_value_or_default(&mut notes, game_hash, "characters", "[Ironclad]");
            }
            if option_can_be(game_hash, "use_advanced_characters", &Yaml::Boolean(false), &Yaml::Boolean(true)) {
                if let Some(advanced_characters_hash) = game_hash.get(&Yaml::from_str("advanced_characters")).and_then(Yaml::as_hash) {
                    notes.push(format!("advanced_characters: [{}]", advanced_characters_hash.keys().map(to_string).collect::<Vec<_>>().join(", ")));
                } else {
                    notes.push(String::from("advanced_characters: [ironclad]"));
                }
            }

            let ascension = Yaml::from_str("ascension");
            if !game_hash.contains_key(&ascension) {
                game_hash.insert(ascension, Yaml::Integer(0));
            }
        }
        Some("Super Metroid Map Rando") => {
            push_value_or_default(&mut notes, game_hash, "preset", "hard");
            println!("'{name}.yaml' contains a Super Metroid Map Rando");
        }
        Some("Sonic Adventure DX") => {
            push_value_or_default(&mut notes, game_hash, "logic_level", "normal_logic");

            if let Some(lazy_fishing) = game_hash.get_mut(&Yaml::from_str("lazy_fishing")) {
                move_option_weight(lazy_fishing, "true", "enabled_all");
                move_option_weight(lazy_fishing, "false", "disabled");
            }
        }
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

            game_hash.remove(&Yaml::from_str("goal"));
            game_hash.remove(&Yaml::from_str("include_secret_mission_completion"));
            game_hash.remove(&Yaml::from_str("boss_rewards"));
            game_hash.remove(&Yaml::from_str("starting_weapon"));
        }
        Some("Against the Storm") => push_value_or_default(&mut notes, game_hash, "enable_dlc", "false"),
        Some("Guild Wars 2") => {
            game_hash.insert(Yaml::from_str("achievement_weight"), Yaml::Integer(0));
            push_value_or_default(&mut notes, game_hash, "storyline", "core");
        }
        Some("Paper Mario") => {
            if let Some(super_multi_blocks) = game_hash.get_mut(&Yaml::from_str("super_multi_blocks")) {
                move_option_weight(super_multi_blocks, "true", "anywhere");
                move_option_weight(super_multi_blocks, "false", "off");
            }
        }
        Some("Gauntlet Legends") => {
            if let Some(traps_frequency) = game_hash.get_mut(&Yaml::from_str("traps_frequency")) {
                move_option_weight(traps_frequency, "normal", "10");
                move_option_weight(traps_frequency, "large", "15");
                move_option_weight(traps_frequency, "extreme", "50");
            }
        }
        Some("Pokemon Crystal") => {
            if let Some(trainer_name) = game_hash.get(&Yaml::from_str("trainer_name")).and_then(|name| name.as_str())
                && !trainer_name.is_empty()
            {
                println!("'{name}.yaml' has trainer_name '{trainer_name}'");
            }

            if let Some(require_itemfinder) = game_hash.get_mut(&Yaml::from_str("require_itemfinder")) {
                move_option_weight(require_itemfinder, "true", "hard_required");
                move_option_weight(require_itemfinder, "false", "not_required");
            }

            if let Some(randomize_wilds) = game_hash.get_mut(&Yaml::from_str("randomize_wilds")) {
                move_option_weight(randomize_wilds, "true", "completely_random");
                move_option_weight(randomize_wilds, "false", "vanilla");
            }

            if let Some(randomize_music) = game_hash.get_mut(&Yaml::from_str("randomize_music")) {
                move_option_weight(randomize_music, "true", "completely_random");
                move_option_weight(randomize_music, "false", "off");
            }
        }
        Some("The Witness") => {
            if let Some(elevators_come_to_you) = game_hash.get_mut(&Yaml::from_str("elevators_come_to_you")) {
                move_option_weight_to_yaml(
                    elevators_come_to_you,
                    "true",
                    Yaml::Array(vec![Yaml::from_str("Quarry Elevator"), Yaml::from_str("Swamp Long Bridge"), Yaml::from_str("Bunker Elevator")]),
                );
                move_option_weight_to_yaml(elevators_come_to_you, "false", Yaml::Hash(LinkedHashMap::new()));
            }
        }
        Some("Final Fantasy Tactics Advance") => {
            if let Some(progressive_shop_battle_unlock) = game_hash.get_mut(&Yaml::from_str("progressive_shop_battle_unlock")) {
                move_option_weight(progressive_shop_battle_unlock, "true", "enabled");
                move_option_weight(progressive_shop_battle_unlock, "false", "disabled");
            }
        }
        Some("Rain World") => {
            game_hash.remove(&Yaml::from_str("which_game_version"));
            push_value_or_default(&mut notes, game_hash, "which_game_version", "1_10_4");
            push_value_or_default(&mut notes, game_hash, "is_msc_enabled", "false");
            push_value_or_default(&mut notes, game_hash, "is_watcher_enabled", "false");
        }
        Some("Sentinels of the Multiverse") => {
            if let Some(filler_weights) = game_hash.get_mut(&Yaml::from_str("filler_weights")).and_then(|yaml| yaml.as_mut_vec()) {
                for entry in filler_weights {
                    if let Some(hash) = entry.as_mut_hash() {
                        hash.remove(&Yaml::from_str("typed"));
                    }
                }
            }
            if let Some(pool_size) = game_hash.get_mut(&Yaml::from_str("pool_size")).and_then(|yaml| yaml.as_mut_hash()) {
                for (_, v) in pool_size {
                    if let Some(str) = v.as_str()
                        && let Some((val, _)) = str.split_once('+')
                    {
                        *v = Yaml::from_str(val);
                    }
                }
            }
            if let Some(location_density) = game_hash.get_mut(&Yaml::from_str("location_density")).and_then(|yaml| yaml.as_mut_hash()) {
                let key = Yaml::from_str("hero");
                if !location_density.contains_key(&key) {
                    location_density.insert(key, Yaml::Integer(0));
                }
                if let Some(value) = location_density.remove(&Yaml::from_str("variant")) {
                    location_density.insert(Yaml::from_str("variant_unlock"), value);
                } else {
                    location_density.insert(Yaml::from_str("variant_unlock"), Yaml::Integer(0));
                }
            }
            push_value_or_default(&mut notes, game_hash, "enabled_sets", "[Official]");
        }
        Some("Hatsune Miku Project Diva Mega Mix+") => {
            game_hash.remove(&Yaml::from_str("exclude_singers"));

            push_value_or_default(&mut notes, game_hash, "allow_megamix_dlc_songs", "false");
            let mod_str = if let Some(yaml) = game_hash.get_mut(&Yaml::from_str("megamix_mod_data")) {
                if let Some(Value::Object(mut map)) = yaml.as_str().and_then(|str| serde_json::from_str(str).ok()) {
                    let mut changed = false;
                    changed |= map.remove("EdenDarkPack").is_some();
                    let mod_str = map.keys().cloned().collect::<Vec<_>>().join(", ");
                    if changed {
                        if let Ok(new) = serde_json::to_string(&Value::Object(map)) {
                            *yaml = Yaml::from_str(&new);
                        } else {
                            game_hash.remove(&Yaml::from_str("megamix_mod_data"));

                            println!("Failed to serialize new mod list for '{name}.yaml', all mods have been removed");
                        }
                    }
                    mod_str
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            notes.push(format!("megamix_mod_data: [{mod_str}]",));
        }
        Some("Jigsaw") => {
            game_hash.remove(&Yaml::from_str("permillage_of_checks_out_of_logic"));
            game_hash.remove(&Yaml::from_str("maximum_number_of_real_items"));
            game_hash.remove(&Yaml::from_str("minimum_number_of_pieces_per_real_item"));
            game_hash.remove(&Yaml::from_str("enable_forced_local_filler_items"));

            let grid_type_yaml = game_hash.remove(&Yaml::from_str("grid_type"));
            let rotatoins_yaml = game_hash.remove(&Yaml::from_str("rotations"));

            let grid_type = grid_type_yaml.as_ref().and_then(Yaml::as_str).unwrap_or("square");
            let rotations = rotatoins_yaml.as_ref().and_then(Yaml::as_str).unwrap_or("no_rotation");

            game_hash.insert(
                Yaml::from_str("grid_type_and_rotations"),
                Yaml::from_str(match (grid_type, rotations) {
                    ("square", "no_rotation") => "square_no_rotation",
                    ("square", "per_90_degrees") => "square_90_rotation",
                    ("square", "per_180_degrees") => "square_180_rotation",
                    ("hexagonal", "no_rotation") => "hex_no_rotation",
                    ("hexagonal", "per_90_degrees") => "hex_60_rotation",
                    ("hexagonal", "per_180_degrees") => "hex_180_rotation",
                    ("meme_one_row", "no_rotation") => "meme_one_row_no_rotation",
                    ("meme_one_row", "per_180_degrees") => "meme_one_row_180_rotation",
                    ("meme_one_column", "no_rotation") => "meme_one_column_no_rotation",
                    ("meme_one_column", "per_180_degrees") => "meme_one_column_180_rotation",
                    (_, _) => "square_no_rotations",
                }),
            );
        }
        Some("Psychonauts") => {
            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("Goal")) {
                move_option_weight(goal, "braintank_and_brainhunt", "asylum_brain_tank_and_brain_hunt");
            }
        }
        Some("Luigi's Mansion") => {
            if let Some(door_rando) = game_hash.get_mut(&Yaml::from_str("door_rando")) {
                move_option_weight(door_rando, "true", "randomized");
                move_option_weight(door_rando, "false", "off");
            }
        }
        Some("Factorio") => {
            if let Some(world_gen) = game_hash.get_mut(&Yaml::from_str("world_gen")).and_then(|yaml| yaml.as_mut_hash()) {
                world_gen.remove(&Yaml::from_str("terrain_segmentation"));
            }
        }
        Some("Ty the Tasmanian Tiger") => push_value_or_default(&mut notes, game_hash, "logic_difficulty", "standard"),
        Some("Paper Mario The Thousand Year Door") | Some("Paper Mario The Thousand-Year Door") => {
            if let Some(chapter_clears) = game_hash.remove(&Yaml::from_str("chapter_clears")) {
                game_hash.insert(Yaml::from_str("goal_stars"), chapter_clears.clone());
                game_hash.insert(Yaml::from_str("palace_stars"), chapter_clears);
                game_hash.insert(Yaml::from_str("goal"), Yaml::from_str("crystal_stars"));
            }

            if let Some(starting_partner) = game_hash.get_mut(&Yaml::from_str("starting_partner")) {
                move_option_weight(starting_partner, "from_partner", "random");
            }

            if let Some(yoshi_color) = game_hash.get_mut(&Yaml::from_str("yoshi_color")) {
                move_option_weight(yoshi_color, "random_color", "random");
            }

            if let Some(star_shuffle) = game_hash.get_mut(&Yaml::from_str("star_shuffle")) {
                move_option_weight(star_shuffle, "false", "vanilla");
                move_option_weight(star_shuffle, "true", "all");
            }

            if option_can_be_other_than(game_hash, "yoshi_name", &Yaml::from_str("Yoshi"), &Yaml::from_str("Yoshi")) {
                println!("'{name}.yaml' has a modified yoshi name");
            }
        }
        Some("Golden Sun The Lost Age") => {
            if let Some(enemy_elemental_resistance) = game_hash.get_mut(&Yaml::from_str("enemy_elemental_resistance")) {
                move_option_weight(enemy_elemental_resistance, "shuffle_elemmental_res", "shuffle_elemental_res");
            }
        }
        Some("The Minish Cap") => {
            if option_can_be(game_hash, "goal_vaati", &Yaml::Boolean(true), &Yaml::Boolean(false)) {
                game_hash.insert(Yaml::from_str("goal"), Yaml::from_str("pedestal"));
            } else {
                game_hash.remove(&Yaml::from_str("goal_vaati"));
            }
        }
        Some("Minishoot Adventures") => {
            if let Some(completion_goals) = game_hash.get_mut(&Yaml::from_str("completion_goals")) {
                move_option_weight(completion_goals, "both", "dungeon_5_and_snow");
            }
        }
        Some("Stacklands") => {
            if option_can_be(game_hash, "goal", &Yaml::from_str(""), &Yaml::from_str("kill_wicked_witch")) {
                game_hash.insert(Yaml::from_str("boards"), Yaml::from_str("mainland_and_forest"));
            } else if option_can_be(game_hash, "goal", &Yaml::from_str(""), &Yaml::from_str("kill_demon")) {
                game_hash.insert(Yaml::from_str("boards"), Yaml::from_str("mainland_only"));
            }

            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("goal")) {
                move_option_weight(goal, "kill_demon", "random_boss");
                move_option_weight(goal, "kill_wicked_witch", "random_boss");
            }
        }
        Some("Oxygen Not Included") => {
            push_value_or_default(&mut notes, game_hash, "spaced_out", "true");
            push_value_or_default(&mut notes, game_hash, "frosty", "true");
            push_value_or_default(&mut notes, game_hash, "bionic", "false");
        }
        Some("Monster Sanctuary") => {
            push_value_or_default(&mut notes, game_hash, "logic_difficulty", "casual");
            push_value_or_default(&mut notes, game_hash, "tedious_checks", "false");
        }
        Some("Spelunky 2") => push_value_or_default(&mut notes, game_hash, "include_hard_locations", "false"),
        Some("Metroid Zero Mission") => {
            if let Some(walljumps_in_logic) = game_hash.remove(&Yaml::from_str("walljumps_in_logic")) {
                game_hash.insert(Yaml::from_str("walljumps"), walljumps_in_logic);
            }

            push_value_or_default(&mut notes, game_hash, "walljumps", "enabled");
            push_value_or_default(&mut notes, game_hash, "logic_difficulty", "simple");
            push_value_or_default(&mut notes, game_hash, "combat_logic_difficulty", "relaxed");
            push_value_or_default(&mut notes, game_hash, "ibj_in_logic", "none");
            push_value_or_default(&mut notes, game_hash, "hazard_runs", "disabled");
            push_value_or_default(&mut notes, game_hash, "tricky_shinesparks", "false");
            push_value_or_default(&mut notes, game_hash, "tricks_allowed", "[]");
        }
        Some("Cuphead") => {
            if let Some(dlc_boss_chalice_checks) = game_hash.get_mut(&Yaml::from_str("dlc_boss_chalice_checks")) {
                move_option_weight(dlc_boss_chalice_checks, "true", "enabled");
                move_option_weight(dlc_boss_chalice_checks, "false", "disabled");
            }

            if let Some(dlc_rungun_chalice_checks) = game_hash.get_mut(&Yaml::from_str("dlc_rungun_chalice_checks")) {
                move_option_weight(dlc_rungun_chalice_checks, "true", "enabled");
                move_option_weight(dlc_rungun_chalice_checks, "false", "disabled");
            }

            if let Some(dlc_kingdice_chalice_checks) = game_hash.get_mut(&Yaml::from_str("dlc_kingdice_chalice_checks")) {
                move_option_weight(dlc_kingdice_chalice_checks, "true", "enabled");
                move_option_weight(dlc_kingdice_chalice_checks, "false", "disabled");
            }

            if let Some(dlc_chess_chalice_checks) = game_hash.get_mut(&Yaml::from_str("dlc_chess_chalice_checks")) {
                move_option_weight(dlc_chess_chalice_checks, "true", "enabled");
                move_option_weight(dlc_chess_chalice_checks, "false", "disabled");
            }

            if let Some(level_shuffle) = game_hash.get_mut(&Yaml::from_str("level_shuffle")) {
                move_option_weight(level_shuffle, "true", "enabled");
                move_option_weight(level_shuffle, "false", "disabled");
            }

            push_value_or_default(&mut notes, game_hash, "dlc_boss_chalice_checks", "disabled");
            push_value_or_default(&mut notes, game_hash, "dlc_rungun_chalice_checks", "disabled");
            push_value_or_default(&mut notes, game_hash, "dlc_kingdice_chalice_checks", "disabled");
            push_value_or_default(&mut notes, game_hash, "dlc_chess_chalice_checks", "disabled");
            push_value_or_default(&mut notes, game_hash, "dlc_cactusgirl_quest", "false");
        }
        Some("Metroid Fusion") => {
            if let Some(tricky_shinesparks_in_region_logic) = game_hash.remove(&Yaml::from_str("TrickyShinesparksInRegionLogic")) {
                game_hash.insert(Yaml::from_str("ShinesparkTrickDifficulty"), tricky_shinesparks_in_region_logic);
            }

            push_value_or_default(&mut notes, game_hash, "PointOfNoReturnsInLogic", "true");
            push_value_or_default(&mut notes, game_hash, "ShinesparkTrickDifficulty", "none");
            push_value_or_default(&mut notes, game_hash, "WallJumpTrickDifficulty", "none");
            push_value_or_default(&mut notes, game_hash, "CombatDifficulty", "beginner");
        }
        Some("The Simpsons Hit And Run") => {
            if let Some(goal) = game_hash.get_mut(&Yaml::from_str("goal")) {
                move_option_weight(goal, "goal: all missions complete!", "goal_all_missions_complete");
                move_option_weight(goal, "goal: all story missions complete!", "goal_all_story_missions_complete");
                move_option_weight(goal, "goal: final mission(l7m7)", "goal_final_missionl7m7");
                move_option_weight(goal, "goal: wasps and cards collected!", "goal_wasps_and_cards_collected");
            }
        }
        Some("Satisfactory") => {
            if let Some(mut final_elevator_package) = game_hash.remove(&Yaml::from_str("final_elevator_package")) {
                move_option_weight(&mut final_elevator_package, "one package (tiers 1-2)", "phase 1 (tiers 1-2)");
                move_option_weight(&mut final_elevator_package, "two packages (tiers 1-4)", "phase 2 (tiers 1-4");
                move_option_weight(&mut final_elevator_package, "three packages (tiers 1-6)", "phase 3 (tiers 1-6");
                move_option_weight(&mut final_elevator_package, "four packages (tiers 1-8)", "phase 4 (tiers 1-8)");
                move_option_weight(&mut final_elevator_package, "five packages (tiers 1-9)", "phase 5 (tiers 1-9)");
                game_hash.insert(Yaml::from_str("final_elevator_phase"), final_elevator_package);
            }
        }
        Some("Trackmania") => {
            if let Some(disable_bronze) = game_hash.remove(&Yaml::from_str("disable_bronze")) {
                game_hash.insert(Yaml::from_str("disable_bronze_locations"), disable_bronze.clone());
                game_hash.insert(Yaml::from_str("disable_bronze_medals"), disable_bronze);
            }

            if let Some(disable_silver) = game_hash.remove(&Yaml::from_str("disable_silver")) {
                game_hash.insert(Yaml::from_str("disable_silver_locations"), disable_silver.clone());
                game_hash.insert(Yaml::from_str("disable_silver_medals"), disable_silver);
            }

            if let Some(disable_gold) = game_hash.remove(&Yaml::from_str("disable_gold")) {
                game_hash.insert(Yaml::from_str("disable_gold_locations"), disable_gold.clone());
                game_hash.insert(Yaml::from_str("disable_gold_medals"), disable_gold);
            }

            if let Some(disable_author) = game_hash.remove(&Yaml::from_str("disable_author")) {
                game_hash.insert(Yaml::from_str("disable_author_locations"), disable_author);
            }
        }
        Some("The Legend of Zelda - Phantom Hourglass") => {
            if let Some(randomize_harrow) = game_hash.get_mut(&Yaml::from_str("randomize_harrow")) {
                move_option_weight(randomize_harrow, "false", "no_harrow");
                move_option_weight(randomize_harrow, "true", "randomize_without_hints");
            }

            if let Some(goal_requirements) = game_hash.get_mut(&Yaml::from_str("goal_requirements")) {
                move_option_weight(goal_requirements, "complete_dungeons", "defeat_bosses");
            }

            game_hash.remove(&Yaml::from_str("dungeon_hints"));

            if let Some(shuffle_dungeon_entrances) = game_hash.get_mut(&Yaml::from_str("shuffle_dungeon_entrances")) {
                move_option_weight(shuffle_dungeon_entrances, "false", "no_shuffle");
                move_option_weight(shuffle_dungeon_entrances, "true", "shuffle");
            }

            if let Some(additional_metal_names) = game_hash.get_mut(&Yaml::from_str("additional_metal_names")) {
                move_option_weight(additional_metal_names, "custom_unique", "custom_prefer_vanilla");
            }

            if option_can_be(game_hash, "shuffle_island_entrances", &Yaml::Boolean(false), &Yaml::Boolean(true)) {
                game_hash.insert(Yaml::from_str("shuffle_ports"), Yaml::from_str("shuffle"));
                game_hash.insert(Yaml::from_str("shuffle_caves"), Yaml::from_str("shuffle"));
                game_hash.insert(Yaml::from_str("shuffle_houses"), Yaml::from_str("shuffle"));
                game_hash.remove(&Yaml::from_str("shuffle_island_entrances"));
            }

            push_value_or_default(&mut notes, game_hash, "logic", "normal");
        }
        Some("Astalon") => {
            if let Some(fast_blood_chalice) = game_hash.get_mut(&Yaml::from_str("fast_blood_chalice")) {
                move_option_weight(fast_blood_chalice, "false", "off");
                move_option_weight(fast_blood_chalice, "true", "always");
            }
        }
        Some("Anodyne") => {
            if let Some(red_cave_access) = game_hash.remove(&Yaml::from_str("red_cave_access")) {
                game_hash.insert(Yaml::from_str("red_grotto_access"), red_cave_access);
            }
        }
        Some("Ape Escape 3") => {
            if let Some(goal_target_override) = game_hash.get_mut(&Yaml::from_str("goal_target_override")) {
                move_option_weight(goal_target_override, "disable", "1");
            }
        }
        Some("Super Mario Land 2") => {
            if let Some(mario_coin_fragment_percentage) = game_hash.get_mut(&Yaml::from_str("mario_coin_fragment_percentage")) {
                move_option_weight_matches(mario_coin_fragment_percentage, |yaml| as_i64(yaml).is_some_and(|v| v >= 50), "50");
            }
        }
        Some("Donkey Kong 64") => {
            push_value_or_default(&mut notes, game_hash, "logic_type", "glitchless");
            push_value_or_default(&mut notes, game_hash, "glitches_selected", "[]");
            push_value_or_default(&mut notes, game_hash, "hard_shooting", "false");
        }
        Some("Pokemon Black and White") => {
            resolve_weighted_option(game_hash, "version");

            if option_can_be(game_hash, "version", &Yaml::from_str("random"), &Yaml::from_str("random")) {
                let mut new_hash = LinkedHashMap::new();
                new_hash.insert(Yaml::from_str("black"), Yaml::Integer(50));
                new_hash.insert(Yaml::from_str("white"), Yaml::Integer(50));
                game_hash.insert(Yaml::from_str("version"), Yaml::Hash(new_hash));
            }

            resolve_weighted_option(game_hash, "version");

            push_value_or_default(&mut notes, game_hash, "version", "N/A");
        }
        Some("Mario Kart 64") => {
            if let Some(logic_difficulty) = game_hash.get_mut(&Yaml::from_str("logic_difficulty")) {
                move_option_weight(logic_difficulty, "generous", "basic");
            }
        }
        Some("Archipela-Go!") => {
            let max_key = Yaml::from_str("maximum_distance");
            let min_key = Yaml::from_str("minimum_distance");

            resolve_weighted_option(game_hash, "maximum_distance");
            resolve_weighted_option(game_hash, "minimum_distance");

            let mut max_distance = match game_hash.get(&max_key) {
                Some(Yaml::Integer(value)) => *value,
                Some(Yaml::String(value)) => ARCHIPELA_GO_DISTANCES.get(value).copied().unwrap_or(5000),
                _ => 5000,
            };

            let mut min_distance = match game_hash.get(&min_key) {
                Some(Yaml::Integer(value)) => *value,
                Some(Yaml::String(value)) => ARCHIPELA_GO_DISTANCES.get(value).copied().unwrap_or(500),
                _ => 500,
            };

            if max_distance > 60000 {
                max_distance = 45000;
            }

            if min_distance > max_distance {
                min_distance = max_distance / 2;
            } else if max_distance > 10000 && min_distance > 20000 {
                min_distance = 20000;
            } else if max_distance > 5000 && min_distance > 5000 {
                min_distance = 5000;
            }

            game_hash.insert(max_key, Yaml::Integer(max_distance));
            game_hash.insert(min_key, Yaml::Integer(min_distance));

            notes.push(String::from(match max_distance {
                ..=5000 => "Walk",
                5001..=10000 => "Bike",
                10001.. => "Car Trip",
            }));
        }
        Some("Crystal Project") => {
            if let Some(regionsanity) = game_hash.get_mut(&Yaml::from_str("regionsanity")) {
                move_option_weight(regionsanity, "false", "disabled");
                move_option_weight(regionsanity, "true", "enabled");
            }
        }
        Some("Yu-Gi-Oh! 2006") => {
            game_hash.remove(&Yaml::from_str("starter_deck"));
            game_hash.remove(&Yaml::from_str("normalize_booster_pack_prices"));
            game_hash.remove(&Yaml::from_str("normalize_booster_pack_rarities"));
            game_hash.remove(&Yaml::from_str("randomize_pack_contents"));
            game_hash.remove(&Yaml::from_str("custom_structure_deck"));
            game_hash.remove(&Yaml::from_str("custom_starter_deck"));
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
                    if weight_string == "~" { to_string(yaml) } else { format!("{}: {weight_string}", to_string(yaml)) }
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

fn move_option_weight(value: &mut Yaml, from_str: &str, to_str: &str) {
    move_option_weight_to_yaml(value, from_str, Yaml::from_str(to_str));
}

fn move_option_weight_to_yaml(value: &mut Yaml, from_str: &str, to: Yaml) {
    let from = Yaml::from_str(from_str);
    let from_str = Yaml::String(String::from(from_str));

    if *value == from || *value == from_str {
        *value = to;
    } else if let Some(hash) = value.as_mut_hash() {
        if let Some(weight) = hash.remove(&from) {
            if let Some(weight) = as_i64(&weight)
                && weight > 0
            {
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
        } else if let Some(weight) = hash.remove(&from_str)
            && let Some(weight) = as_i64(&weight)
            && weight > 0
        {
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
