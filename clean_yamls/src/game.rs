use hashlink::LinkedHashMap;
use rand::thread_rng;
use rand_distr::{Distribution, WeightedIndex};
use yaml_rust2::Yaml;

use crate::valid_games::VALID_GAMES;

pub fn choose_game(doc: &mut Yaml) -> Option<Yaml> {
    let game_key = Yaml::from_str("game");

    if let Some(hash) = doc.as_mut_hash() {
        rename_game(hash, "PokéPark Wii: Pikachu's Adventure [JP]", "PokePark");
        rename_game(hash, "PokéPark", "PokePark");
    }

    if let Some(games) = doc.as_mut_hash()?.get_mut(&game_key) {
        let game = match games {
            Yaml::Hash(games) => {
                let mut rng = thread_rng();

                let mut games: Vec<_> = games
                    .iter()
                    .filter_map(|(k, v)| match (k.as_str(), v.as_f64(), v.as_i64()) {
                        (Some(game), Some(weight), None) => Some((game, weight as i64)),
                        (Some(game), None, Some(weight)) => Some((game, weight)),
                        _ => None,
                    })
                    .collect();

                for (game, _) in &mut games {
                    if *game == "PokéPark Wii: Pikachu's Adventure [JP]" || *game == "PokéPark" {
                        *game = "PokePark";
                    }
                }

                if games.iter().any(|(game, weight)| *weight > 0 && VALID_GAMES.contains(game)) {
                    games.retain(|(game, _)| VALID_GAMES.contains(game));
                }

                let dist = WeightedIndex::new(games.iter().map(|(_, weight)| weight)).expect("Failed to create index");
                Yaml::from_str(games[dist.sample(&mut rng)].0)
            }
            Yaml::String(game) => {
                if *game == "PokéPark Wii: Pikachu's Adventure [JP]" || *game == "PokéPark" {
                    *game = String::from("PokePark");
                }
                Yaml::from_str(game)
            }
            _ => return None,
        };

        *games = game.clone();
        Some(game)
    } else {
        None
    }
}

fn rename_game(hash: &mut LinkedHashMap<Yaml, Yaml>, from: &str, to: &str) {
    if let Some(options) = hash.remove(&Yaml::from_str(from)) {
        hash.insert(Yaml::from_str(to), options);
    }
}
