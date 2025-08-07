use std::io::Write;

use crate::valid_games::VALID_GAMES;

pub fn write_to_output_list<T: Write>(writer: &mut T, name: &str, games: &[(String, u32)], notes: &[String]) {
    if let Err(err) = write!(writer, "{name}\t") {
        println!("Failed to write to output file: {err}");
    }

    let mut invalid_games = vec![];
    for (game, count) in games {
        if *count > 0 && !VALID_GAMES.contains(&game.as_str()) {
            invalid_games.push(game.as_str());
        }
    }

    match invalid_games.len() {
        1 => println!("'{name}.yaml' contains invalid game: {}", invalid_games[0]),
        2.. => println!("'{name}.yaml' contains invalid games: [{}]", invalid_games.join(", ")),
        _ => (),
    }

    if games.len() == 1 {
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

        for (game, count) in &games[0..games.len() - 1] {
            if *count > 1 {
                if let Err(err) = write!(writer, "{} *{} AND\n ", game, count) {
                    println!("Failed to write to output file: {err}");
                }
            } else if let Err(err) = write!(writer, "{} AND\n ", game) {
                println!("Failed to write to output file: {err}");
            }
        }

        let (game, count) = games.last().expect("Last game does not exist");
        if *count > 1 {
            if let Err(err) = write!(writer, "{} *{}\"", game, count) {
                println!("Failed to write to output file: {err}");
            }
        } else if let Err(err) = write!(writer, "{}\"", game) {
            println!("Failed to write to output file: {err}");
        }
    }

    if notes.is_empty() {
        if let Err(err) = writeln!(writer) {
            println!("Failed to write to output file: {err}");
        }
    } else if let Err(err) = writeln!(writer, "\t{}", notes.iter().map(String::as_str).collect::<Vec<_>>().join(", ")) {
        println!("Failed to write to output file: {err}");
    }
}
