use std::{
    env,
    fmt::Display,
    fs, io,
    num::{ParseIntError, TryFromIntError},
    path::{Path, PathBuf},
    time::Instant,
};

pub const DAYS: u8 = 25;
pub const LAST_YEAR: u16 = 2024;

// https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsError.html
#[derive(Debug, Clone)]
pub enum AoCError {
    Parsing,
    Solving,
    Custom(String),
}

impl AoCError {
    pub fn new<T: Into<String>>(t: T) -> Self {
        Self::Custom(t.into())
    }
}

use core::fmt;
impl std::error::Error for AoCError {}
impl Display for AoCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AoCError::Parsing => "Failed to parse",
            AoCError::Solving => "Failed to solve",
            AoCError::Custom(msg) => msg,
        };
        write!(f, "{}", message)
    }
}

impl From<io::Error> for AoCError {
    fn from(value: io::Error) -> Self {
        Self::new(value.to_string())
    }
}
impl From<ParseIntError> for AoCError {
    fn from(value: ParseIntError) -> Self {
        Self::new(value.to_string())
    }
}
impl From<TryFromIntError> for AoCError {
    fn from(value: TryFromIntError) -> Self {
        Self::new(value.to_string())
    }
}

pub type AoCResult<T> = Result<T, AoCError>;

pub struct Day {
    value: u8,
}

impl Day {
    pub fn try_new(value: u8) -> AoCResult<Self> {
        if value < 1 || value > 25 {
            return Err(AoCError::new("Invalid day"));
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

pub struct Year {
    value: u16,
}

impl Year {
    pub fn try_new(value: u16) -> AoCResult<Self> {
        if value < 2015 || value > LAST_YEAR {
            return Err(AoCError::new("Invalid year"));
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> u16 {
        self.value
    }
}

pub enum Answer {
    Part(String),
    Both(Solution),
}

pub enum Part {
    One,
    Two,
    Both,
}

impl Part {
    pub fn new(value: u8) -> Self {
        match value {
            1 => Self::One,
            2 => Self::Two,
            _ => Self::Both,
        }
    }
}

// https://github.com/rustwasm/wasm-bindgen/issues/1775
// https://stackoverflow.com/questions/68243940/rust-wasm-bindgen-struct-with-string
// Strings on a struct can not be public
// skip the fields with a wasm_bindgen macro, but implement a getter for them so you can access them in JS.
// a weird disable only to later enable, I know.
// You can do without the wasm_bindgen skip on the fields, but since my other Rust code accesses them, I need them to be public.

pub struct Solution {
    pub part1: Box<dyn Display>,
    pub part2: Box<dyn Display>,
}

pub trait AoCData<'a> {
    /// Parse an input string into a Data struct for a specific day
    fn try_new(input: &'a str) -> AoCResult<Self>
    where
        Self: Sized;

    /// part1 solution
    fn part_1(&self) -> AoCResult<impl Display>;

    /// part2 solution
    fn part_2(&self) -> AoCResult<impl Display>;

    /// both solutions
    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        // have to make sure results that come back from the part functions live long enough,
        // because those might be borrowed things that impl Display
        Ok(Solution {
            part1: Box::new(self.part_1()?.to_string()),
            part2: Box::new(self.part_2()?.to_string()),
        })
    }
}

pub fn get_input(year: &Year, day: &Day) -> Result<String, AoCError> {
    let mut input_path = workspace_dir();
    input_path.push(year.value().to_string());
    input_path.push("inputs");
    input_path.push(format!("day{:02}.txt", day.value()));
    let input = fs::read_to_string(input_path)?;
    Ok(input)
}

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(
        std::str::from_utf8(&output)
            .unwrap()
            .trim(),
    );
    cargo_path
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn print_part(
    year: &Year,
    day: &Day,
    part: &Part,
    part_solver: impl Fn(&Day, &Part, &str) -> AoCResult<Answer>,
) {
    println!(
        "Year {}, Day {}, {}",
        year.value(),
        day.value(),
        match part {
            Part::One => "part 1",
            Part::Two => "part 2",
            Part::Both => "both parts",
        }
    );
    println!("{:=<20}", "=");
    let input = get_input(year, day).unwrap();
    let now = Instant::now();
    let result = part_solver(day, &part, &input);
    let elapsed = now.elapsed();
    println!("Runtime:");
    println!("{:?}", elapsed);
    match result.unwrap_or(Answer::Part("No result!".to_string())) {
        Answer::Part(result) => {
            println!("Answer:");
            println!("{result}");
        }
        Answer::Both(solution) => {
            println!("Part 1:");
            println!("{}", solution.part1);
            println!("Part 2:");
            println!("{}", solution.part2);
        }
    }
}

pub fn part_helper<'a, T: AoCData<'a>>(part: &Part, input: &'a str) -> AoCResult<Answer> {
    let data = T::try_new(input)?;
    let answer = match part {
        Part::One => Answer::Part(data.part_1()?.to_string()),
        Part::Two => Answer::Part(data.part_2()?.to_string()),
        Part::Both => Answer::Both(data.solve()?),
    };
    Ok(answer)
}
