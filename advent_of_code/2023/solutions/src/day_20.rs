use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseStrength {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'a> {
    FlipFlop {
        on: bool,
    },
    Conjunction {
        memory: HashMap<&'a str, PulseStrength>,
    },
    Broadcaster,
}

#[derive(Debug, Clone, Copy)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    strength: PulseStrength,
}

impl<'a> Pulse<'a> {
    fn send(
        self,
        modulemap: &mut HashMap<&'a str, Module>,
        destmap: &HashMap<&'a str, Vec<&'a str>>,
        q: &mut VecDeque<Pulse<'a>>,
    ) -> AoCResult<()> {
        let Some(module) = modulemap.get_mut(self.to) else {
            // hit the rx module, it doesn't send anything
            return Ok(());
        };

        // figure out which pulse to send, if any
        let send = match module {
            Module::FlipFlop { on } => match self.strength {
                // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                PulseStrength::High => None,
                // However, if a flip-flop module receives a low pulse, it flips between on and off.
                // If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
                PulseStrength::Low => {
                    *on = !*on;
                    let strength = if *on {
                        PulseStrength::High
                    } else {
                        PulseStrength::Low
                    };
                    Some(strength)
                }
            },
            Module::Conjunction { memory } => {
                // When a pulse is received, the conjunction module first updates its memory for that input.
                *memory
                    .get_mut(self.from)
                    .ok_or(AoCError::Solving)? = self.strength;
                // then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                let strength = if memory
                    .values()
                    .all(|&strength| strength == PulseStrength::High)
                {
                    PulseStrength::Low
                } else {
                    PulseStrength::High
                };
                Some(strength)
            }
            // There is a single broadcast module (named broadcaster).
            // When it receives a pulse, it sends the same pulse to all of its destination modules.
            Module::Broadcaster => Some(self.strength),
        };

        // send pulses to all destinations
        if let Some(strength) = send {
            for &to in destmap
                .get(self.to)
                .ok_or(AoCError::Solving)?
            {
                let pulse = Pulse {
                    from: self.to,
                    to,
                    strength,
                };
                q.push_back(pulse);
            }
        };
        Ok(())
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[derive(Debug, Clone)]
pub struct Data<'a> {
    destmap: HashMap<&'a str, Vec<&'a str>>,
    modulemap: HashMap<&'a str, Module<'a>>,
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        fn parse_line(s: &str) -> AoCResult<(&str, Module, Vec<&str>)> {
            let (lhs, rhs) = s
                .split_once(" -> ")
                .ok_or(AoCError::Parsing)?;
            let outputs: Vec<&str> = rhs.split(", ").collect();
            let module = match &lhs[0..1] {
                "b" => Module::Broadcaster,
                "%" => Module::FlipFlop { on: false },
                "&" => Module::Conjunction {
                    memory: HashMap::new(),
                },
                _ => return Err(AoCError::Parsing),
            };
            let name = if module == Module::Broadcaster {
                lhs
            } else {
                &lhs[1..]
            };

            Ok((name, module, outputs))
        }

        let mut destmap = HashMap::new();
        let mut modulemap = HashMap::new();

        for parsed_line in input
            .lines()
            .map(|line| parse_line(line))
        {
            let (name, module, destinations) = parsed_line?;
            modulemap.insert(name, module);
            destmap.insert(name, destinations);
        }

        // set the initial remembered pulses to a low pulse for every module that's connected to a conjuction module
        for (source, destinations) in &destmap {
            for destination in destinations {
                if let Some(Module::Conjunction { memory }) = modulemap.get_mut(destination) {
                    memory.insert(source, PulseStrength::Low);
                }
            }
        }

        Ok(Self { destmap, modulemap })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let Data { modulemap, destmap } = self;
        let mut modulemap = modulemap.clone();

        let mut low_count = 0;
        let mut high_count = 0;

        for _ in 0..1_000 {
            let mut q = VecDeque::new();
            q.push_back(Pulse {
                from: "button",
                to: "broadcaster",
                strength: PulseStrength::Low,
            });
            while let Some(pulse) = q.pop_front() {
                match pulse.strength {
                    PulseStrength::Low => low_count += 1,
                    PulseStrength::High => high_count += 1,
                }
                pulse.send(&mut modulemap, destmap, &mut q)?
            }
        }

        Ok(low_count * high_count)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let Data { modulemap, destmap } = self;
        let mut modulemap = modulemap.clone();

        let (before_rx, _) = destmap
            .iter()
            .find(|(_, destinations)| destinations.contains(&"rx"))
            .ok_or(AoCError::Solving)?;
        let Some(Module::Conjunction { memory }) = modulemap.get(before_rx) else {
            return Err(AoCError::Solving);
        };

        // since the module before rx is a conjunction, is needs all its inputs to have sent a high signal in order to send a low signal
        // remember after how many presses an input to before_rx sent a high signal
        let mut tracker: HashMap<&str, Option<u64>> = memory
            .keys()
            .map(|&name| (name, None))
            .collect();

        for presses in 1.. {
            let mut q = VecDeque::new();
            q.push_back(Pulse {
                from: "button",
                to: "broadcaster",
                strength: PulseStrength::Low,
            });

            while let Some(pulse) = q.pop_front() {
                if pulse.to == *before_rx && pulse.strength == PulseStrength::High {
                    *tracker
                        .get_mut(pulse.from)
                        .ok_or(AoCError::Solving)? = Some(presses);
                    // if all inputs to before_rx have a known presscount, figure out when they all send a high signal at the same time
                    if tracker
                        .values()
                        .all(|presses| presses.is_some())
                    {
                        return Ok(tracker
                            .values()
                            .map(|presses| presses.unwrap())
                            .fold(1, |acc, curr| lcm(acc, curr)));
                    }
                }

                pulse.send(&mut modulemap, &destmap, &mut q)?
            }
        }

        Err(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "11687500");
    }
}
