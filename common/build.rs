use std::process::exit;

include!("./src/valid_games.rs");
const SPECIAL_STR: &str = include_str!("./src/special.rs");

// validate that all special rules are for valid games
fn main() {
    let mut skip_next = false;

    for (idx, line) in SPECIAL_STR.lines().enumerate() {
        if line.trim() == "// IGNORE" {
            skip_next = true;
            continue;
        }

        if let Some((empty, rest)) = line.split_once("        Some(\"")
            && empty.is_empty()
            && let Some((game, empty)) = rest.split_once("\") => {")
            && empty.is_empty()
            && !VALID_GAMES.contains(&game)
            && !skip_next
        {
            println!("{game} is not valid on line {idx}");
            exit(1);
        }

        skip_next = false;
    }
}
