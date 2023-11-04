use std::{collections::HashMap, fmt::Display};

use aoc_core::AoCError;
use itertools::{Either, Itertools, MinMaxResult};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Instruction>);

#[derive(Debug, Clone)]
enum Instruction {
    Start(u8, u8),                          // (chip_num, bot_num)
    Transfer(u8, Destination, Destination), // (chip_num, low_dest, high_dest)
}

#[derive(Debug, Clone)]
enum Destination {
    Output(u8),
    Bot(u8),
}

fn part_1_helper(instructions: &[Instruction], target_low: u8, target_high: u8) -> u8 {
    let mut state = HashMap::new();
    let (start_instructions, transfer_instructions): (Vec<_>, Vec<_>) = instructions
        .iter()
        .partition_map(|ins| match ins {
            Instruction::Start(val, bot) => Either::Left((*val, *bot)),
            Instruction::Transfer(val, low, high) => Either::Right((*val, low, high)),
        });

    for (bot_num, _, _) in &transfer_instructions {
        state.insert(*bot_num, vec![]);
    }

    for (val, bot_num) in start_instructions {
        if let Some(chips) = state.get_mut(&bot_num) {
            chips.push(val);
        }
    }
    loop {
        for (bot_num, low_dest, high_dest) in &transfer_instructions {
            let chips = state.get_mut(bot_num).unwrap();
            if chips.len() == 2 {
                if let MinMaxResult::MinMax(min, max) = chips.drain(..).minmax() {
                    if min == target_low && max == target_high {
                        return *bot_num;
                    }
                    if let Destination::Bot(n) = low_dest {
                        state
                            .entry(*n)
                            .and_modify(|chips| chips.push(min));
                    }
                    if let Destination::Bot(n) = high_dest {
                        state
                            .entry(*n)
                            .and_modify(|chips| chips.push(max));
                    }
                }
            }
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let instruction = match line.starts_with("value") {
                true => {
                    let rest = line
                        .strip_prefix("value ")
                        .ok_or(AoCError::Parsing)?;
                    let (value, bot) = rest
                        .split_once(" goes to bot ")
                        .ok_or(AoCError::Parsing)?;
                    Instruction::Start(value.parse()?, bot.parse()?)
                }
                false => {
                    let rest = line
                        .strip_prefix("bot ")
                        .ok_or(AoCError::Parsing)?;
                    let (bot, rest) = rest
                        .split_once(" gives low to ")
                        .ok_or(AoCError::Parsing)?;
                    let (low_dest, high_dest) = rest
                        .split_once(" and high to ")
                        .ok_or(AoCError::Parsing)?;
                    let low_dest = if let Some(low_dest) = low_dest.strip_prefix("bot ") {
                        Destination::Bot(low_dest.parse()?)
                    } else {
                        let low_dest = low_dest
                            .strip_prefix("output ")
                            .ok_or(AoCError::Parsing)?;
                        Destination::Output(low_dest.parse()?)
                    };
                    let high_dest = if let Some(high_dest) = high_dest.strip_prefix("bot ") {
                        Destination::Bot(high_dest.parse()?)
                    } else {
                        let high_dest = high_dest
                            .strip_prefix("output ")
                            .ok_or(AoCError::Parsing)?;
                        Destination::Output(high_dest.parse()?)
                    };
                    Instruction::Transfer(bot.parse()?, low_dest, high_dest)
                }
            };
            instructions.push(instruction);
        }
        Ok(Self(instructions))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(part_1_helper(&self.0, 17, 61))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut state = HashMap::new();
        let mut outputs = [None; 3];
        let (start_instructions, transfer_instructions): (Vec<_>, Vec<_>) = self
            .0
            .iter()
            .partition_map(|ins| match ins {
                Instruction::Start(val, bot) => Either::Left((*val, *bot)),
                Instruction::Transfer(val, low, high) => Either::Right((*val, low, high)),
            });

        for (bot_num, _, _) in &transfer_instructions {
            state.insert(*bot_num, vec![]);
        }

        for (val, bot_num) in start_instructions {
            if let Some(chips) = state.get_mut(&bot_num) {
                chips.push(val);
            }
        }

        loop {
            if let Some(vec) = outputs
                .into_iter()
                .collect::<Option<Vec<u8>>>()
            {
                return Ok(vec
                    .iter()
                    .map(|n| u32::from(*n))
                    .product::<u32>());
            }
            for (bot_num, low_dest, high_dest) in &transfer_instructions {
                let chips = state.get_mut(bot_num).unwrap();
                if chips.len() == 2 {
                    if let MinMaxResult::MinMax(min, max) = chips.drain(..).minmax() {
                        match low_dest {
                            Destination::Bot(n) => {
                                state
                                    .entry(*n)
                                    .and_modify(|chips| chips.push(min));
                            }
                            Destination::Output(n) if *n < 3 => {
                                outputs[*n as usize] = Some(min);
                            }
                            _ => {}
                        }
                        match high_dest {
                            Destination::Bot(n) => {
                                state
                                    .entry(*n)
                                    .and_modify(|chips| chips.push(max));
                            }
                            Destination::Output(n) if *n < 3 => {
                                outputs[*n as usize] = Some(max);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_1_helper(&data.0, 2, 5), 2);
    }

    #[test]
    fn part_2() {
        let input = "bot 88 gives low to bot 51 and high to bot 42
bot 13 gives low to bot 4 and high to bot 167
bot 90 gives low to bot 78 and high to bot 199
bot 84 gives low to bot 205 and high to bot 201
bot 41 gives low to bot 48 and high to bot 15
bot 15 gives low to bot 156 and high to bot 54
bot 70 gives low to output 10 and high to bot 4
bot 140 gives low to bot 206 and high to bot 189
value 67 goes to bot 187
bot 124 gives low to bot 99 and high to bot 102
bot 203 gives low to bot 55 and high to bot 33
bot 118 gives low to bot 131 and high to bot 79
bot 78 gives low to bot 15 and high to bot 122
bot 82 gives low to bot 112 and high to bot 127
bot 207 gives low to bot 136 and high to bot 50
bot 57 gives low to output 16 and high to bot 3
bot 101 gives low to bot 165 and high to bot 209
bot 60 gives low to output 12 and high to bot 5
bot 31 gives low to bot 104 and high to bot 44
bot 104 gives low to bot 115 and high to bot 35
value 31 goes to bot 97
bot 117 gives low to bot 0 and high to bot 85
bot 175 gives low to bot 74 and high to bot 92
bot 160 gives low to bot 116 and high to bot 69
bot 96 gives low to bot 107 and high to bot 8
bot 55 gives low to bot 13 and high to bot 126
bot 36 gives low to bot 140 and high to bot 91
bot 111 gives low to bot 63 and high to bot 131
bot 159 gives low to bot 59 and high to bot 30
bot 105 gives low to bot 132 and high to bot 34
value 29 goes to bot 86
bot 187 gives low to bot 40 and high to bot 87
bot 191 gives low to bot 148 and high to bot 39
bot 73 gives low to bot 105 and high to bot 20
bot 194 gives low to output 7 and high to bot 107
bot 62 gives low to bot 61 and high to bot 55
bot 89 gives low to bot 36 and high to bot 141
bot 170 gives low to bot 1 and high to bot 139
bot 51 gives low to bot 147 and high to bot 113
bot 108 gives low to bot 92 and high to bot 162
bot 162 gives low to bot 180 and high to bot 68
bot 0 gives low to bot 77 and high to bot 152
bot 112 gives low to bot 33 and high to bot 127
bot 52 gives low to bot 101 and high to bot 49
bot 71 gives low to bot 169 and high to bot 41
bot 195 gives low to output 4 and high to bot 66
bot 93 gives low to bot 17 and high to bot 123
bot 192 gives low to bot 47 and high to bot 124
bot 209 gives low to bot 188 and high to bot 16
bot 49 gives low to bot 209 and high to bot 154
bot 99 gives low to output 19 and high to bot 142
bot 97 gives low to bot 32 and high to bot 129
bot 198 gives low to bot 110 and high to bot 67
value 73 goes to bot 12
bot 20 gives low to bot 34 and high to bot 137
bot 50 gives low to bot 153 and high to bot 0
bot 171 gives low to bot 67 and high to bot 146
bot 138 gives low to bot 100 and high to bot 74
bot 106 gives low to bot 83 and high to bot 82
bot 151 gives low to bot 197 and high to bot 101
value 47 goes to bot 36
bot 26 gives low to bot 98 and high to bot 104
bot 134 gives low to bot 149 and high to bot 143
value 3 goes to bot 29
bot 7 gives low to bot 30 and high to bot 110
value 43 goes to bot 206
bot 150 gives low to output 11 and high to bot 53
bot 44 gives low to bot 35 and high to bot 18
bot 127 gives low to bot 134 and high to bot 143
bot 74 gives low to bot 157 and high to bot 192
bot 129 gives low to bot 37 and high to bot 158
bot 17 gives low to bot 27 and high to bot 73
bot 196 gives low to bot 208 and high to bot 83
value 71 goes to bot 89
bot 81 gives low to bot 181 and high to bot 71
bot 2 gives low to bot 12 and high to bot 45
bot 176 gives low to bot 191 and high to bot 172
value 59 goes to bot 207
bot 76 gives low to bot 41 and high to bot 78
value 41 goes to bot 136
bot 66 gives low to output 15 and high to bot 100
bot 115 gives low to bot 163 and high to bot 10
bot 141 gives low to bot 91 and high to bot 119
bot 46 gives low to bot 172 and high to bot 98
bot 158 gives low to bot 93 and high to bot 123
bot 4 gives low to output 14 and high to bot 194
bot 33 gives low to bot 126 and high to bot 134
bot 83 gives low to bot 184 and high to bot 82
bot 137 gives low to bot 31 and high to bot 44
bot 25 gives low to bot 141 and high to bot 22
bot 131 gives low to bot 135 and high to bot 128
value 37 goes to bot 43
bot 135 gives low to bot 162 and high to bot 130
bot 120 gives low to bot 88 and high to bot 178
bot 189 gives low to bot 114 and high to bot 176
bot 130 gives low to bot 68 and high to bot 208
bot 64 gives low to bot 85 and high to bot 59
value 13 goes to bot 103
bot 1 gives low to bot 202 and high to bot 185
value 2 goes to bot 57
bot 197 gives low to bot 57 and high to bot 165
bot 163 gives low to bot 7 and high to bot 198
bot 22 gives low to bot 119 and high to bot 132
bot 8 gives low to bot 94 and high to bot 173
bot 152 gives low to bot 65 and high to bot 133
value 19 goes to bot 151
bot 167 gives low to bot 194 and high to bot 96
bot 116 gives low to bot 79 and high to bot 69
bot 193 gives low to bot 38 and high to bot 200
bot 80 gives low to bot 183 and high to bot 9
bot 114 gives low to bot 161 and high to bot 191
bot 113 gives low to bot 125 and high to bot 1
bot 32 gives low to bot 103 and high to bot 129
bot 136 gives low to bot 86 and high to bot 153
bot 182 gives low to bot 45 and high to bot 17
bot 18 gives low to bot 171 and high to bot 146
bot 155 gives low to bot 46 and high to bot 26
bot 148 gives low to bot 201 and high to bot 168
bot 54 gives low to bot 175 and high to bot 108
bot 201 gives low to bot 117 and high to bot 64
bot 174 gives low to bot 139 and high to bot 160
bot 103 gives low to bot 187 and high to bot 37
bot 28 gives low to bot 9 and high to bot 147
bot 132 gives low to bot 155 and high to bot 23
bot 34 gives low to bot 23 and high to bot 137
bot 39 gives low to bot 168 and high to bot 58
bot 161 gives low to bot 84 and high to bot 148
bot 98 gives low to bot 109 and high to bot 115
bot 199 gives low to bot 122 and high to bot 111
bot 12 gives low to bot 89 and high to bot 25
bot 188 gives low to bot 150 and high to bot 21
bot 9 gives low to bot 76 and high to bot 90
bot 27 gives low to bot 22 and high to bot 105
bot 23 gives low to bot 26 and high to bot 31
value 61 goes to bot 2
bot 154 gives low to bot 16 and high to bot 81
bot 157 gives low to output 8 and high to bot 47
bot 109 gives low to bot 58 and high to bot 163
bot 75 gives low to bot 178 and high to bot 56
bot 156 gives low to bot 138 and high to bot 175
bot 95 gives low to bot 5 and high to bot 166
bot 11 gives low to bot 176 and high to bot 46
bot 91 gives low to bot 189 and high to bot 11
bot 86 gives low to bot 151 and high to bot 52
bot 149 gives low to bot 96 and high to bot 145
bot 173 gives low to bot 60 and high to bot 95
bot 67 gives low to bot 120 and high to bot 75
value 5 goes to bot 140
value 11 goes to bot 121
bot 100 gives low to output 13 and high to bot 157
value 53 goes to bot 197
bot 186 gives low to bot 170 and high to bot 174
bot 185 gives low to bot 118 and high to bot 116
bot 19 gives low to bot 195 and high to bot 190
bot 126 gives low to bot 167 and high to bot 149
bot 143 gives low to bot 145 and high to bot 204
bot 40 gives low to bot 2 and high to bot 182
bot 122 gives low to bot 54 and high to bot 63
bot 16 gives low to bot 21 and high to bot 181
bot 110 gives low to bot 177 and high to bot 120
bot 208 gives low to bot 72 and high to bot 184
bot 205 gives low to bot 50 and high to bot 117
bot 168 gives low to bot 64 and high to bot 159
bot 29 gives low to bot 43 and high to bot 84
bot 79 gives low to bot 128 and high to bot 24
value 23 goes to bot 97
bot 94 gives low to output 5 and high to bot 60
bot 178 gives low to bot 42 and high to bot 186
bot 200 gives low to bot 28 and high to bot 51
bot 63 gives low to bot 108 and high to bot 135
bot 5 gives low to output 17 and high to bot 166
bot 30 gives low to bot 193 and high to bot 177
bot 47 gives low to output 3 and high to bot 99
bot 24 gives low to bot 196 and high to bot 106
bot 128 gives low to bot 130 and high to bot 196
bot 3 gives low to output 6 and high to bot 150
bot 77 gives low to bot 49 and high to bot 65
bot 61 gives low to bot 70 and high to bot 13
bot 190 gives low to bot 66 and high to bot 138
bot 142 gives low to output 2 and high to bot 70
bot 87 gives low to bot 182 and high to bot 93
bot 121 gives low to bot 29 and high to bot 161
bot 204 gives low to bot 173 and high to bot 95
bot 139 gives low to bot 185 and high to bot 160
bot 35 gives low to bot 10 and high to bot 18
bot 37 gives low to bot 87 and high to bot 158
bot 179 gives low to bot 81 and high to bot 183
bot 10 gives low to bot 198 and high to bot 171
bot 38 gives low to bot 80 and high to bot 28
bot 92 gives low to bot 192 and high to bot 180
bot 177 gives low to bot 200 and high to bot 88
bot 166 gives low to output 20 and high to output 9
bot 42 gives low to bot 113 and high to bot 170
bot 14 gives low to bot 133 and high to bot 38
bot 172 gives low to bot 39 and high to bot 109
bot 165 gives low to bot 3 and high to bot 188
bot 169 gives low to bot 19 and high to bot 48
bot 125 gives low to bot 199 and high to bot 202
bot 85 gives low to bot 152 and high to bot 14
bot 102 gives low to bot 142 and high to bot 61
bot 180 gives low to bot 124 and high to bot 144
value 7 goes to bot 40
bot 68 gives low to bot 144 and high to bot 72
bot 72 gives low to bot 62 and high to bot 203
bot 56 gives low to bot 186 and high to bot 174
bot 184 gives low to bot 203 and high to bot 112
bot 65 gives low to bot 154 and high to bot 179
bot 58 gives low to bot 159 and high to bot 7
bot 147 gives low to bot 90 and high to bot 125
bot 206 gives low to bot 121 and high to bot 114
bot 202 gives low to bot 111 and high to bot 118
bot 21 gives low to bot 53 and high to bot 164
bot 153 gives low to bot 52 and high to bot 77
bot 119 gives low to bot 11 and high to bot 155
bot 164 gives low to bot 6 and high to bot 19
bot 146 gives low to bot 75 and high to bot 56
bot 123 gives low to bot 73 and high to bot 20
bot 145 gives low to bot 8 and high to bot 204
bot 183 gives low to bot 71 and high to bot 76
bot 69 gives low to bot 24 and high to bot 106
bot 6 gives low to output 18 and high to bot 195
bot 144 gives low to bot 102 and high to bot 62
bot 43 gives low to bot 207 and high to bot 205
bot 59 gives low to bot 14 and high to bot 193
value 17 goes to bot 32
bot 181 gives low to bot 164 and high to bot 169
bot 107 gives low to output 1 and high to bot 94
bot 45 gives low to bot 25 and high to bot 27
bot 133 gives low to bot 179 and high to bot 80
bot 53 gives low to output 0 and high to bot 6
bot 48 gives low to bot 190 and high to bot 156
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "13727");
    }
}
