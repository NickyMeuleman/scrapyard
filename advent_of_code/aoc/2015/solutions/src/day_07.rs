use std::{collections::HashMap, fmt::Display};

use aoc_core::{AoCError, Solution};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    instructions: HashMap<String, Signal>,
}

#[derive(Debug, Clone)]
enum Signal {
    Gate(Gate),
    Wire(String),
    Value(u16),
}

#[derive(Debug, Clone)]
enum Gate {
    And(Input, Input),
    Or(Input, Input),
    Lshift(Input, Input),
    Rshift(Input, Input),
    Not(Input),
}

#[derive(Debug, Clone)]
enum Input {
    Number(u16),
    Wire(String),
}

impl Data {
    fn resolve_wire(&self, name: &str, cache: &mut HashMap<String, u16>) -> u16 {
        // find the signal that leads to the named wire
        let signal = self.instructions.get(name).unwrap();
        // get the numerical value of that signal
        // check cache first, if not in cache, compute
        let result = if let Some(num) = cache.get(name) {
            *num
        } else {
            match signal {
                Signal::Value(num) => *num,
                Signal::Gate(gate) => self.resolve_gate(gate, cache),
                Signal::Wire(name) => self.resolve_wire(name, cache),
            }
        };
        // store the computed result in the cache
        cache.insert(name.to_string(), result);
        result
    }

    fn resolve_gate(&self, gate: &Gate, cache: &mut HashMap<String, u16>) -> u16 {
        match gate {
            Gate::And(lhs, rhs) => {
                let lhs = self.resolve_input(lhs, cache);
                let rhs = self.resolve_input(rhs, cache);
                lhs & rhs
            }
            Gate::Or(lhs, rhs) => {
                let lhs = self.resolve_input(lhs, cache);
                let rhs = self.resolve_input(rhs, cache);
                lhs | rhs
            }
            Gate::Lshift(lhs, rhs) => {
                let lhs = self.resolve_input(lhs, cache);
                let rhs = self.resolve_input(rhs, cache);
                lhs << rhs
            }
            Gate::Rshift(lhs, rhs) => {
                let lhs = self.resolve_input(lhs, cache);
                let rhs = self.resolve_input(rhs, cache);
                lhs >> rhs
            }
            Gate::Not(input) => {
                let input = self.resolve_input(input, cache);
                !input
            }
        }
    }

    fn resolve_input(&self, input: &Input, cache: &mut HashMap<String, u16>) -> u16 {
        match input {
            Input::Number(num) => *num,
            Input::Wire(name) => self.resolve_wire(name, cache),
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        // map input lines as HashMap<named wire, signal to that wire>
        let mut instructions = HashMap::new();

        for line in input.lines() {
            // from: this signal
            // to: resolves to this named wire
            let (from, to) = line
                .split_once(" -> ")
                .ok_or(AoCError::Parsing)?;
            // split signal string up in parts
            let parts: Vec<_> = from.split_whitespace().collect();
            let from = match parts.len() {
                // one part: numerical value or a named wire
                1 => match parts[0].parse() {
                    Ok(num) => Signal::Value(num),
                    Err(_) => Signal::Wire(parts[0].to_string()),
                },
                // two parts: NOT
                // last part: a numerical value or a named wire
                2 => {
                    let input = match parts[1].parse() {
                        Ok(num) => Input::Number(num),
                        Err(_) => Input::Wire(parts[1].to_string()),
                    };
                    Signal::Gate(Gate::Not(input))
                }
                // three parts: AND, OR, LSHIFT, RSHIFT
                // first and last part: a numerical value or a named wire
                3 => {
                    let lhs = parts[0];
                    let gate = parts[1];
                    let rhs = parts[2];
                    let lhs = match lhs.parse() {
                        Ok(num) => Input::Number(num),
                        Err(_) => Input::Wire(lhs.to_string()),
                    };
                    let rhs = match rhs.parse() {
                        Ok(num) => Input::Number(num),
                        Err(_) => Input::Wire(rhs.to_string()),
                    };
                    let gate = match gate {
                        "AND" => Gate::And(lhs, rhs),
                        "OR" => Gate::Or(lhs, rhs),
                        "LSHIFT" => Gate::Lshift(lhs, rhs),
                        "RSHIFT" => Gate::Rshift(lhs, rhs),
                        _ => panic!("invalid input"),
                    };
                    Signal::Gate(gate)
                }
                _ => panic!("Invalid input"),
            };

            instructions.insert(to.to_string(), from);
        }

        Ok(Self { instructions })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self.resolve_wire("a", &mut HashMap::new()))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut data = self.clone();
        let mut cache: HashMap<String, u16> = HashMap::new();
        let a_result = data.resolve_wire("a", &mut cache);

        cache.clear();
        // replace the signal to b with the resolved signal to a from part 1
        data.instructions
            .insert("b".to_string(), Signal::Value(a_result));

        Ok(data.resolve_wire("a", &mut cache))
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let mut data = self;
        let mut cache: HashMap<String, u16> = HashMap::new();
        let a_result = data.resolve_wire("a", &mut cache);

        cache.clear();
        // replace the signal to b with the resolved signal to a from part 1
        data.instructions
            .insert("b".to_string(), Signal::Value(a_result));

        Ok(Solution {
            part1: Box::new(a_result),
            part2: Box::new(data.resolve_wire("a", &mut cache)),
        })
    }
}
