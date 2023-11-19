use std::collections::VecDeque;

use aoc_core::{AoCError, AoCResult};

#[derive(Debug, Clone, Default)]
pub struct Computer {
    instruction_pointer: usize,
    pub memory: Vec<i64>,
    inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,
    relative_base: i64,
}

enum Opcode {
    Halt,
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBase,
}

impl Opcode {
    fn try_new(n: i64) -> AoCResult<Self> {
        match n {
            99 => Ok(Self::Halt),
            1 => Ok(Self::Addition),
            2 => Ok(Self::Multiplication),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            9 => Ok(Self::RelativeBase),
            n => Err(AoCError::Custom(format!("Invalid opcode, found: {n}"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Halted,
    NeedInput,
    Running,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn try_new(n: i64) -> AoCResult<Self> {
        match n {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            n => Err(AoCError::Custom(format!(
                "Invalid parameter mode, found: {n}"
            ))),
        }
    }
}

impl Computer {
    fn get_param_info(&self, operation: i64, offset: usize) -> AoCResult<(i64, Mode)> {
        let param = self.read_raw(self.instruction_pointer + offset);
        let mode = Mode::try_new((operation / 10i64.pow(offset as u32 + 1)) % 10)?;
        Ok((param, mode))
    }

    fn read(&mut self, operation: i64, offset: usize) -> AoCResult<i64> {
        let (param, mode) = self.get_param_info(operation, offset)?;
        let result = match mode {
            Mode::Position => self.read_raw(param as usize),
            Mode::Immediate => param,
            Mode::Relative => self.read_raw((self.relative_base + param) as usize),
        };
        Ok(result)
    }

    pub fn read_raw(&self, pointer: usize) -> i64 {
        *self.memory.get(pointer).unwrap_or(&0)
    }

    fn write(&mut self, operation: i64, offset: usize, val: i64) -> AoCResult<()> {
        let (param, mode) = self.get_param_info(operation, offset)?;
        match mode {
            Mode::Position => {
                self.write_raw(param as usize, val);
                Ok(())
            }
            Mode::Immediate => Err(AoCError::Custom(
                "Attempted to write in immediate mode".to_string(),
            )),
            Mode::Relative => {
                self.write_raw((self.relative_base + param) as usize, val);
                Ok(())
            }
        }
    }

    pub fn write_raw(&mut self, pointer: usize, val: i64) {
        if pointer >= self.memory.len() {
            self.memory.resize(pointer + 1, 0);
        }
        self.memory[pointer] = val;
    }

    pub fn set_memory(&mut self, memory: Vec<i64>) {
        self.memory = memory;
    }

    pub fn input(&mut self, num: i64) {
        self.inputs.push_back(num);
    }

    pub fn last_output(&self) -> Option<i64> {
        self.outputs.back().copied()
    }

    pub fn consume_output(&mut self) -> Option<i64> {
        self.outputs.pop_front()
    }

    // does one operation and returns if program stops running
    pub fn operate(&mut self) -> AoCResult<Status> {
        let operation = self.read_raw(self.instruction_pointer);
        let opcode = Opcode::try_new(operation % 100)?;
        match opcode {
            Opcode::Halt => return Ok(Status::Halted),
            // addition, 3 parameters: lhs, rhs, target position in memory
            Opcode::Addition => {
                let num_1 = self.read(operation, 1)?;
                let num_2 = self.read(operation, 2)?;
                let val = num_1 + num_2;
                self.write(operation, 3, val)?;
                self.instruction_pointer += 4;
            }
            // multiplication, 3 parameters: lhs, rhs, target position in memory
            Opcode::Multiplication => {
                let num_1 = self.read(operation, 1)?;
                let num_2 = self.read(operation, 2)?;
                let val = num_1 * num_2;
                self.write(operation, 3, val)?;
                self.instruction_pointer += 4;
            }
            // consume input, 1 parameter: target position in memory
            // Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
            Opcode::Input => {
                if let Some(input) = self.inputs.pop_front() {
                    self.write(operation, 1, input)?;
                    self.instruction_pointer += 2;
                } else {
                    return Ok(Status::NeedInput);
                }
            }
            // produce output, 1 parameter: value of param is added to outputs
            Opcode::Output => {
                let value = self.read(operation, 1)?;
                self.outputs.push_back(value);
                self.instruction_pointer += 2;
            }
            // jump-if-true, 2 parameters: check-value, jump-value
            Opcode::JumpIfTrue => {
                let val_1 = self.read(operation, 1)?;
                if val_1 != 0 {
                    let val_2 = self.read(operation, 2)?;
                    self.instruction_pointer = val_2 as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            // jump-if-false, 2 parameters: check-value, jump-value
            Opcode::JumpIfFalse => {
                let val_1 = self.read(operation, 1)?;
                if val_1 == 0 {
                    let val_2 = self.read(operation, 2)?;
                    self.instruction_pointer = val_2 as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            // less-than, 3 parameters: lhs, rhs, target position in memory
            Opcode::LessThan => {
                let val_1 = self.read(operation, 1)?;
                let val_2 = self.read(operation, 2)?;
                if val_1 < val_2 {
                    self.write(operation, 3, 1)?;
                } else {
                    self.write(operation, 3, 0)?;
                }
                self.instruction_pointer += 4;
            }
            // equals, 3 parameters: lhs, rhs, target position in memory
            Opcode::Equals => {
                let val_1 = self.read(operation, 1)?;
                let val_2 = self.read(operation, 2)?;
                if val_1 == val_2 {
                    self.write(operation, 3, 1)?;
                } else {
                    self.write(operation, 3, 0)?;
                }
                self.instruction_pointer += 4;
            }
            Opcode::RelativeBase => {
                let val = self.read(operation, 1)?;
                self.relative_base += val;
                self.instruction_pointer += 2;
            }
        }
        Ok(Status::Running)
    }

    /// run until program halts from a 99 opcode or needs an input to continue
    pub fn run(&mut self) -> AoCResult<Status> {
        while let Ok(status) = self.operate() {
            match status {
                Status::Running => continue,
                _ => return Ok(status),
            }
        }
        return Err(AoCError::Solving);
    }
}
