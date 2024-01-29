pub fn transform(input: &str, line_width: u32) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for word in words {
        let extra_space = if !current_line.is_empty() { 1 } else { 0 };
        let new_line_length = current_line.len() + word.len() + extra_space;

        if new_line_length > line_width as usize {
            lines.push(justify_line(&current_line, line_width as usize));
            current_line.clear();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(justify_line(&current_line, line_width as usize));
    }

    lines.join("\n")
}

fn justify_line(line: &str, width: usize) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() == 1 {
        return format!("{:width$}", line, width = width);
    }

    let spaces = width - line.chars().filter(|c| !c.is_whitespace()).count();
    let space_slots = words.len() - 1;
    let mut result = String::new();

    let base_space = spaces / space_slots;
    let extra_space = spaces % space_slots;

    for (i, word) in words.iter().enumerate() {
        result.push_str(word);
        if i < space_slots {
            // Calculate the number of spaces to add to the current word
            let spaces_to_add = base_space + if i < extra_space { 1 } else { 0 };

            result.push_str(&" ".repeat(spaces_to_add));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
        ];

        for &(input, line_width, expected) in &test_cases {
            assert_eq!(transform(input, line_width), expected);
        }
    }
}
