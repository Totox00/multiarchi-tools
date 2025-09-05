use phf::phf_map;
use std::io::Write;

use crate::valid_games::VALID_GAMES;

const SKIPPED_GAMES: [&str; 1] = ["Clique"];
const POINTS_OVERRIDE: phf::Map<&'static str, u32> = phf_map! {
    "Clique" => 0,
    "Autopelago" => 0,
    "ArchipIDLE" => 0,
    "Archipelago" => 0,
    "APBingo" => 0,
    "Keymaster's Keep" => 2,
    "Stardew Valley" => 2
};

pub fn write_to_output_list<T: Write>(writer: &mut T, name: &str, games: &[(String, u32, Vec<String>)]) {
    if let Err(err) = write!(writer, "{name}\t") {
        println!("Failed to write to output file: {err}");
    }

    let mut counted_games = 0;
    let mut points = 1;
    let mut invalid_games = vec![];
    for (game, count, _) in games {
        if *count > 0 && !VALID_GAMES.contains(&game.as_str()) {
            invalid_games.push(game.as_str());
        }

        if counted_games < 8 && !SKIPPED_GAMES.contains(&game.as_str()) {
            counted_games += 1;
            if let Some(new_points) = POINTS_OVERRIDE.get(game.as_str()) {
                points += *new_points;
            } else {
                points += 1;
            }
        }
    }

    match invalid_games.len() {
        1 => println!("'{name}.yaml' contains invalid game: {}", invalid_games[0]),
        2.. => println!("'{name}.yaml' contains invalid games: [{}]", invalid_games.join(", ")),
        _ => (),
    }

    if games.is_empty() {
        println!("'{name}.yaml' has no game specified");
    } else if games.len() == 1 {
        if games[0].1 > 1 {
            if let Err(err) = write!(writer, "{} *{}", games[0].0, games[0].1) {
                println!("Failed to write to output file: {err}");
            }
        } else if let Err(err) = write!(writer, "{}", games[0].0) {
            println!("Failed to write to output file: {err}");
        }
    } else {
        if let Err(err) = write!(writer, "\"") {
            println!("Failed to write to output file: {err}");
        }

        for (game, count, _) in &games[0..games.len() - 1] {
            if *count > 1 {
                if let Err(err) = write!(writer, "{} *{} AND\n ", game, count) {
                    println!("Failed to write to output file: {err}");
                }
            } else if let Err(err) = write!(writer, "{} AND\n ", game) {
                println!("Failed to write to output file: {err}");
            }
        }

        let (game, count, _) = games.last().expect("Last game does not exist");
        if *count > 1 {
            if let Err(err) = write!(writer, "{} *{}\"", game, count) {
                println!("Failed to write to output file: {err}");
            }
        } else if let Err(err) = write!(writer, "{}\"", game) {
            println!("Failed to write to output file: {err}");
        }
    }

    let mut note_lines = vec![];
    for (_, _, notes) in games {
        if !notes.is_empty() {
            note_lines.push(notes.join(", "));
        }
    }

    if note_lines.is_empty() {
        if let Err(err) = write!(writer, "\t") {
            println!("Failed to write to output file: {err}");
        }
    } else if note_lines.len() == 1 {
        if let Err(err) = write!(writer, "\t{}", note_lines[0]) {
            println!("Failed to write to output file: {err}");
        }
    } else if let Err(err) = write!(writer, "\t\"{}\"", note_lines.iter().map(String::as_str).collect::<Vec<_>>().join("\n")) {
        println!("Failed to write to output file: {err}");
    }

    if let Err(err) = writeln!(writer, "\t{points}") {
        println!("Failed to write to output file: {err}");
    }
}

pub fn write_to_bot_output<T: Write>(writer: &mut T, name: &str, games: &[(String, u32, Vec<String>)]) {
    let mut counted_games = 0;
    let mut points = 1;
    for (game, _, _) in games {
        if counted_games < 8 && !SKIPPED_GAMES.contains(&game.as_str()) {
            counted_games += 1;
            if let Some(new_points) = POINTS_OVERRIDE.get(game.as_str()) {
                points += *new_points;
            } else {
                points += 1;
            }
        }
    }

    if let Err(err) = writeln!(
        writer,
        "{name}\n{}\n{}\n{points}",
        games.iter().map(|(game, count, _)| format!("{game} x{count}")).collect::<Vec<_>>().join(", "),
        games.iter().flat_map(|(_, _, notes)| notes).map(|string| string.as_str()).collect::<Vec<_>>().join(", ")
    ) {
        println!("Failed to write to bot output file: {err}");
    }
}
