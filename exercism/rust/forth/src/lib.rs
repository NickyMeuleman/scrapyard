pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, Clone)]
enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Number(Value),
    Custom(Value),
}

#[derive(Debug)]
struct Definition {
    name: String,
    instructions: Vec<Instruction>,
}

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    definitions: Vec<Definition>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        self.eval_vec(input.split_ascii_whitespace().collect())
    }

    fn eval_vec(&mut self, input: Vec<&str>) -> Result {
        // IF definition: split up into definition and the rest
        // IF instruction: split up into instructions and the rest
        // eval the definition/instructions and recursively call eval_vec with the rest
        if input.is_empty() {
            return Ok(());
        }
        let first = input.first().unwrap();
        if *first == ":" {
            let def_end_idx = input
                .iter()
                .position(|&s| s == ";")
                .ok_or(Error::InvalidWord)?;
            let (definition, rest) = input.split_at(def_end_idx);
            self.eval_definition(definition.iter().copied().skip(1))?;
            self.eval_vec(rest.iter().copied().skip(1).collect())
        } else {
            let def_start_idx = input.iter().position(|&s| s == ":");
            if let Some(idx) = def_start_idx {
                let (instructions, rest) = input.split_at(idx);
                self.eval_instructions(instructions.to_vec().into_iter())?;
                self.eval_vec(rest.to_vec())
            } else {
                self.eval_instructions(input.into_iter())
            }
        }
    }

    fn eval_definition<'a>(&mut self, mut iter: impl Iterator<Item = &'a str>) -> Result {
        if let Some(name) = iter.next() {
            if let Ok(_) = name.parse::<Value>() {
                return Err(Error::InvalidWord);
            }
            let definition = Definition {
                name: name.to_ascii_uppercase(),
                instructions: iter
                    .filter_map(|s| self.parse_instruction(s).ok())
                    .collect(),
            };
            self.definitions.push(definition);
        }
        Ok(())
    }

    fn eval_instructions<'a>(&mut self, iter: impl Iterator<Item = &'a str>) -> Result {
        for s in iter {
            let instr = self.parse_instruction(s)?;
            self.eval_instruction(instr)?
        }
        Ok(())
    }

    fn parse_instruction<'a>(&mut self, s: &'a str) -> std::result::Result<Instruction, Error> {
        let name = s.to_ascii_uppercase();
        let custom = self.find_instruction(&name);
        let num = name.parse::<Value>();
        match name.as_str() {
            _ if custom.is_some() => Ok(custom.unwrap()),
            "+" => Ok(Instruction::Add),
            "-" => Ok(Instruction::Sub),
            "*" => Ok(Instruction::Mul),
            "/" => Ok(Instruction::Div),
            "DUP" => Ok(Instruction::Dup),
            "DROP" => Ok(Instruction::Drop),
            "SWAP" => Ok(Instruction::Swap),
            "OVER" => Ok(Instruction::Over),
            _ if num.is_ok() => Ok(Instruction::Number(num.unwrap())),
            _ => Err(Error::UnknownWord),
        }
    }

    fn eval_instruction(&mut self, input: Instruction) -> Result {
        match input {
            Instruction::Add => self.add(),
            Instruction::Sub => self.sub(),
            Instruction::Mul => self.mul(),
            Instruction::Div => self.div(),
            Instruction::Dup => self.dup(),
            Instruction::Drop => self.drop(),
            Instruction::Swap => self.swap(),
            Instruction::Over => self.over(),
            Instruction::Number(n) => {
                self.stack.push(n);
                Ok(())
            }
            Instruction::Custom(idx) => self.custom(idx as usize),
        }
    }

    fn find_instruction(&self, name: &str) -> Option<Instruction> {
        // rposition to take the most recent one first if there are multiple
        self.definitions
            .iter()
            .rposition(|defn| defn.name == name)
            .map(|idx| Instruction::Custom(idx as i32))
    }

    fn add(&mut self) -> Result {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(rhs), Some(lhs)) => {
                self.stack.push(lhs + rhs);
                Ok(())
            }
            _ => Err(Error::StackUnderflow),
        }
    }

    fn sub(&mut self) -> Result {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(rhs), Some(lhs)) => {
                self.stack.push(lhs - rhs);
                Ok(())
            }
            _ => Err(Error::StackUnderflow),
        }
    }

    fn mul(&mut self) -> Result {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(rhs), Some(lhs)) => {
                self.stack.push(lhs * rhs);
                Ok(())
            }
            _ => Err(Error::StackUnderflow),
        }
    }

    fn div(&mut self) -> Result {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(0), Some(_)) => Err(Error::DivisionByZero),
            (Some(rhs), Some(lhs)) => {
                self.stack.push(lhs / rhs);
                Ok(())
            }
            _ => Err(Error::StackUnderflow),
        }
    }

    fn dup(&mut self) -> Result {
        let last = self.stack.pop().ok_or(Error::StackUnderflow)?;
        self.stack.push(last);
        self.stack.push(last);
        Ok(())
    }

    fn drop(&mut self) -> Result {
        self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok(())
    }

    fn swap(&mut self) -> Result {
        let last = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let previous = self.stack.pop().ok_or(Error::StackUnderflow)?;
        self.stack.push(last);
        self.stack.push(previous);
        Ok(())
    }

    fn over(&mut self) -> Result {
        let last = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let previous = self.stack.pop().ok_or(Error::StackUnderflow)?;
        self.stack.push(previous);
        self.stack.push(last);
        self.stack.push(previous);
        Ok(())
    }

    fn custom(&mut self, idx: usize) -> Result {
        let def = self.definitions[idx].instructions.clone();
        for instr in def {
            self.eval_instruction(instr)?;
        }
        Ok(())
    }
}