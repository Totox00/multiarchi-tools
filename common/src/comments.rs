#[derive(Debug)]
pub struct Comment<'a> {
    last_key: Option<&'a str>,
    comment: String,
    inline: bool,
    indent: String,
}

pub fn get_comments(content: &str) -> Vec<Comment<'_>> {
    let mut last_key = None;
    let mut comments = vec![];
    let mut can_extend = true;

    for line in content.lines() {
        let mut new_key = false;
        if let Some(key) = find_key(line) {
            last_key = Some(key);
            new_key = true;
            can_extend = false;
        }
        if let Some((indent, comment)) = line.split_once('#') {
            if !indent.trim().is_empty() {
                continue;
            }

            if let Some(Comment {
                last_key: last_last_key,
                comment: last_comment,
                inline: _,
                indent: _,
            }) = comments.last_mut()
            {
                if can_extend && last_key == *last_last_key {
                    last_comment.push('\n');
                    last_comment.push_str(indent);
                    last_comment.push('#');
                    last_comment.push_str(comment);
                } else {
                    comments.push(Comment::new(last_key, comment.to_string(), new_key, indent.to_string()));
                    can_extend = true;
                }
            } else {
                comments.push(Comment::new(last_key, comment.to_string(), new_key, indent.to_string()));
                can_extend = true;
            }
        }
    }

    comments
}

pub fn insert_comments(output: String, comments: &[Comment], source: &str) -> Vec<String> {
    let mut lines: Vec<_> = output.lines().map(String::from).collect();
    let mut line_i = 0;
    'outer: for Comment { last_key, comment, inline, indent } in comments {
        if last_key.is_none() {
            lines.insert(line_i, format!("{indent}#{comment}"));
            line_i += 1;
            continue;
        }

        if line_i == lines.len() {
            println!("Failed to preserve all comments from '{source}'");
            break 'outer;
        }
        while find_key(&lines[line_i]) != *last_key {
            line_i += 1;

            if line_i == lines.len() {
                println!("Failed to preserve all comments from '{source}'");
                break 'outer;
            }
        }

        if *inline {
            lines[line_i].push_str(&format!(" #{comment}"));
        } else {
            line_i += 1;
            lines.insert(line_i, format!("{indent}#{comment}"));
        }
        line_i += 1;
    }

    lines
}

fn find_key(line: &str) -> Option<&str> {
    let not_comment = if let Some((content, _)) = line.split_once('#') { content } else { line };

    if let Some((key, _)) = not_comment.split_once(':') {
        Some(key.trim_start().trim_matches('\'').trim_matches('\"'))
    } else {
        None
    }
}

impl<'a> Comment<'a> {
    fn new(last_key: Option<&'a str>, comment: String, inline: bool, indent: String) -> Self {
        Comment { last_key, comment, inline, indent }
    }
}
