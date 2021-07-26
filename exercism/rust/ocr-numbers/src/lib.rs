const CODE_WIDTH: usize = 3;
const CODE_HEIGHT: usize = 4;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();

    match (lines.len(), lines[0].len()) {
        (rows, _) if rows % CODE_HEIGHT != 0 => Err(Error::InvalidRowCount(rows)),
        (_, cols) if cols % CODE_WIDTH != 0 => Err(Error::InvalidColumnCount(cols)),
        (_, _) => Ok(lines
            .chunks_exact(CODE_HEIGHT)
            // the bottom line is always left blank, only take the lines above
            .map(|chunk| &chunk[0..CODE_HEIGHT - 1])
            // map each coded line-bundle into a String
            .map(|coded| {
                // step through each coded line-bundle a CODE_WIDTH at a time
                // each step, create a single code, parse it to a char
                (0..coded[0].len())
                    .step_by(CODE_WIDTH)
                    .map(|col_idx| {
                        let code: Vec<_> = (0..coded.len())
                            .map(|row_idx| &coded[row_idx][col_idx..col_idx + CODE_WIDTH])
                            .collect();
                        code_to_char(&code)
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(",")),
    }
}

fn code_to_char(code: &[&str]) -> char {
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
