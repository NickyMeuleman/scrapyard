#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();

    let rows = lines.len();
    let cols = lines[0].len();

    if rows % 4 != 0 {
        return Err(Error::InvalidRowCount(rows));
    } else if cols % 3 != 0 {
        return Err(Error::InvalidColumnCount(cols));
    }

    let code_lines: Vec<Vec<&str>> = lines
        .chunks_exact(4)
        .into_iter()
        .map(|chunk| chunk[0..3].to_vec())
        .collect();

    let mut result = String::new();

    for code_line in code_lines.iter() {
        for idx in (0..code_line[0].len()).step_by(3) {
            let code: [&str; 3] = [
                &code_line[0][idx..idx + 3],
                &code_line[1][idx..idx + 3],
                &code_line[2][idx..idx + 3],
            ];
            let parsed = parse_digit(code);
            result.push(parsed);
        }
        result.push(',');
    }

    // delete last ","
    Ok(result[0..result.len() - 1].to_string())
}

fn parse_digit(code: [&str; 3]) -> char {
    match code {
        [" _ ", "| |", "|_|"] => '0',
        ["   ", "  |", "  |"] => '1',
        [" _ ", " _|", "|_ "] => '2',
        [" _ ", " _|", " _|"] => '3',
        ["   ", "|_|", "  |"] => '4',
        [" _ ", "|_ ", " _|"] => '5',
        [" _ ", "|_ ", "|_|"] => '6',
        [" _ ", "  |", "  |"] => '7',
        [" _ ", "|_|", "|_|"] => '8',
        [" _ ", "|_|", " _|"] => '9',
        _ => '?',
    }
}
