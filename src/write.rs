use std::io::Write;

pub fn write_to_output_list<T: Write>(writer: &mut T, name: &str, games: &[(String, u32)]) {
    if let Err(err) = write!(writer, "{name}\t") {
        println!("Failed to write to output file: {err}");
    }

    if games.len() == 1 {
        if games[0].1 > 1 {
            if let Err(err) = writeln!(writer, "{} *{}", games[0].0, games[0].1) {
                println!("Failed to write to output file: {err}");
            }
        } else if let Err(err) = writeln!(writer, "{}", games[0].0) {
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
            if let Err(err) = writeln!(writer, "{} *{}\"", game, count) {
                println!("Failed to write to output file: {err}");
            }
        } else if let Err(err) = writeln!(writer, "{}\"", game) {
            println!("Failed to write to output file: {err}");
        }
    }
}
