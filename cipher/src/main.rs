use std::env::args;

const CAPITAL: char = '√';
const REPEAT: char = 'Ω';

fn main() {
    let mut args = args().skip(1);
    let mode = args.next().expect("A mode must be provided");
    let msg = args.next().expect("A message must be provided");

    println!(
        "{}",
        match mode.as_str() {
            "encode" => encode(&msg),
            "decode" => decode(&msg),
            _ => String::from("Invalid mode"),
        }
    );
}

fn encode(msg: &str) -> String {
    let mut buf = String::new();
    let mut idx = 0;

    for char in msg.chars() {
        if char.is_whitespace() {
            idx += 1;
            buf.push_str(&idx.to_string());
            continue;
        }

        let encoded = encoded(char.to_ascii_lowercase());
        if buf.ends_with(encoded) {
            buf.push(REPEAT);
            continue;
        }

        if char.is_uppercase() {
            buf.push(CAPITAL);
        }

        buf.push(encoded);
    }

    buf
}

fn decode(msg: &str) -> String {
    let mut buf = String::new();
    let mut in_space = false;
    let mut next_capital = false;

    for char in msg.chars() {
        if char == CAPITAL {
            in_space = false;
            next_capital = true;
            continue;
        }

        if char == REPEAT {
            in_space = false;
            buf.push(buf.chars().next_back().expect("Cannot repeat at the start"));
            continue;
        }

        if char.is_ascii_digit() {
            if !in_space {
                in_space = true;
                buf.push(' ');
            }
            continue;
        }

        let decoded = decoded(char);
        buf.push(if next_capital { decoded.to_ascii_uppercase() } else { decoded });

        in_space = false;
        next_capital = false;
    }

    buf
}

fn encoded(chr: char) -> char {
    match chr {
        'a' => '*',
        'b' => '-',
        'c' => '/',
        'd' => '%',
        'e' => '≥',
        'f' => '≤',
        'g' => '~',
        'h' => '╔',
        'i' => '$',
        'j' => '£',
        'k' => '╚',
        'l' => ']',
        'm' => '}',
        'n' => '[',
        'o' => '{',
        'p' => '>',
        'q' => '<',
        'r' => '¿',
        's' => '@',
        't' => '^',
        'u' => ')',
        'v' => '(',
        'w' => '!',
        'x' => '¬',
        'y' => '&',
        'z' => '+',
        '\'' => '"',
        '.' => ':',
        ',' => ';',
        '?' => '?',
        _ => panic!("{chr} cannot be encoded"),
    }
}

fn decoded(chr: char) -> char {
    match chr {
        '*' => 'a',
        '-' => 'b',
        '/' => 'c',
        '%' => 'd',
        '≥' => 'e',
        '≤' => 'f',
        '~' => 'g',
        '╔' => 'h',
        '$' => 'i',
        '£' => 'j',
        '╚' => 'k',
        ']' => 'l',
        '}' => 'm',
        '[' => 'n',
        '{' => 'o',
        '>' => 'p',
        '<' => 'q',
        '¿' => 'r',
        '@' => 's',
        '^' => 't',
        ')' => 'u',
        '(' => 'v',
        '!' => 'w',
        '¬' => 'x',
        '&' => 'y',
        '+' => 'z',
        '"' => '\'',
        ':' => '.',
        ';' => ',',
        '?' => '?',
        _ => panic!("{chr} cannot be decoded"),
    }
}
