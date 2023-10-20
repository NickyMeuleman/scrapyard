use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Computer {
    pointer: usize,
    pub memory: Vec<i32>,
    inputs: VecDeque<i32>,
    outputs: Vec<i32>,
}

pub enum Status {
    Halted,
    NeedInput,
    Running,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
}

fn parse_first_value(memory: &Vec<i32>, pointer: usize) -> (i32, Vec<Mode>) {
    // last 2 digits are the opcode
    let num = memory[pointer];
    let mut modes = Vec::new();
    let opcode = num % 100;
    for c in num.to_string().chars().rev().skip(2) {
        let num = c.to_digit(10).unwrap();
        let mode = match num {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Unknown mode (not position or immidiate)"),
        };
        modes.push(mode);
    }
    (opcode, modes)
}

fn get_value(memory: &[i32], pointer: usize, mode: &Mode) -> i32 {
    match mode {
        Mode::Position => {
            let pos = memory[pointer];
            memory[pos as usize]
        }
        Mode::Immediate => memory[pointer],
    }
}

impl Computer {
    pub fn new() -> Self {
        Self {
            pointer: 0,
            memory: Vec::new(),
            inputs: VecDeque::new(),
            outputs: Vec::new(),
        }
    }

    pub fn set_memory(&mut self, memory: Vec<i32>) {
        self.memory = memory;
    }

    pub fn input(&mut self, num: i32) {
        self.inputs.push_back(num);
    }

    pub fn last_output(&mut self) -> Option<i32> {
        self.outputs.pop()
    }

    // does one operation and returns if program halted
    pub fn operate(&mut self) -> Status {
        let (opcode, modes) = parse_first_value(&self.memory, self.pointer);
        match opcode {
            // halt
            99 => return Status::Halted,
            // addition, 3 parameters: lhs, rhs, target position in memory
            1 => {
                let num_1 = get_value(
                    &self.memory,
                    self.pointer + 1,
                    modes.get(0).unwrap_or(&Mode::Position),
                );
                let num_2 = get_value(
                    &self.memory,
                    self.pointer + 2,
                    modes.get(1).unwrap_or(&Mode::Position),
                );
                let output_pos = self.memory[self.pointer as usize + 3];
                let result = num_1 + num_2;
                self.memory[output_pos as usize] = result;
                self.pointer += 4;
            }
            // multiplication, 3 parameters: lhs, rhs, target position in memory
            2 => {
                let num_1 = get_value(
                    &self.memory,
                    self.pointer + 1,
                    modes.get(0).unwrap_or(&Mode::Position),
                );
                let num_2 = get_value(
                    &self.memory,
                    self.pointer + 2,
                    modes.get(1).unwrap_or(&Mode::Position),
                );
                let output_pos = self.memory[self.pointer as usize + 3];
                let result = num_1 * num_2;
                self.memory[output_pos as usize] = result;
                self.pointer += 4;
            }
            // consume input, 1 parameter: target position in memory
            // Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
            3 => {
                let output_pos = self.memory[self.pointer as usize + 1];
                if let Some(input) = self.inputs.pop_front() {
                    self.memory[output_pos as usize] = input;
                    self.pointer += 2;
                } else {
                    return Status::NeedInput;
                }
            }
            // produce output, 1 parameter: value of param is added to outputs
            4 => {
                let value = get_value(
                    &self.memory,
                    self.pointer + 1,
                    modes.get(0).unwrap_or(&Mode::Position),
                );
                self.outputs.push(value);
                self.pointer += 2;
            }
            // jump-if-true, 2 parameters: check-value, jump-value
            5 => {
                let val_1 = get_value(
                    &self.memory,
                    self.pointer + 1,
                    modes.get(0).unwrap_or(&Mode::Position),
                );
                if val_1 != 0 {
                    let val_2 = get_value(
                        &self.memory,
                        self.pointer + 2,
                        modes.get(1).unwrap_or(&Mode::Position),
                    );
                    self.pointer = val_2 as usize;
                } else {
                    self.pointer += 3;
                }
            }
            // jump-if-false, 2 parameters: check-value, jump-value
            6 => {
                let val_1 = get_value(
                    &self.memory,
                    self.pointer + 1,
                    modes.get(0).unwrap_or(&Mode::Position),
                );
                if val_1 == 0 {
                    let val_2 = get_value(
                        &self.memory,
                        self.pointer + 2,
                        modes.get(1).unwrap_or(&Mode::Position),
                    );
                    self.pointer = val_2 as usize;
                } else {
                    self.pointer += 3;
                }
            }
            // less-than, 3 parameters: lhs, rhs, target position in memory
            7 => {
                let val_1 = get_value(
                    &self.memory,
                    self.pointer + 1,
                    modes.get(0).unwrap_or(&Mode::Position),
                );
                let val_2 = get_value(
                    &self.memory,
                    self.pointer + 2,
                    modes.get(1).unwrap_or(&Mode::Position),
                );
                let pos_1 = self.memory[self.pointer as usize + 3];
                if val_1 < val_2 {
                    self.memory[pos_1 as usize] = 1;
                } else {
                    self.memory[pos_1 as usize] = 0;
                }
                self.pointer += 4;
            }
            // equals, 3 parameters: lhs, rhs, target position in memory
            8 => {
                let val_1 = get_value(
                    &self.memory,
                    self.pointer + 1,
                    modes.get(0).unwrap_or(&Mode::Position),
                );
                let val_2 = get_value(
                    &self.memory,
                    self.pointer + 2,
                    modes.get(1).unwrap_or(&Mode::Position),
                );
                let pos_1 = self.memory[self.pointer as usize + 3];
                if val_1 == val_2 {
                    self.memory[pos_1 as usize] = 1;
                } else {
                    self.memory[pos_1 as usize] = 0;
                }
                self.pointer += 4;
            }
            _ => panic!("disco"),
        }
        Status::Running
    }

    /// run until program halts from a 99 opcode or needs an input to continue, return last outut
    pub fn run(&mut self) -> Option<i32> {
        while let Status::Running = self.operate() {}
        self.last_output()
    }
}
