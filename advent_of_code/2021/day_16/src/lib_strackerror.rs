// lightly edited solution from:
// https://github.com/Strackeror/aoc_2021_rust/blob/main/src/day16.rs
use std::{convert::Infallible, str::FromStr};
use std::collections::VecDeque;
#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(u32, u64),
    Operator(u32, u32, Vec<Packet>),
}

fn value(bits: impl IntoIterator<Item = bool>) -> u64 {
    bits.into_iter()
        .fold(0, |acc, bit| acc * 2 + if bit { 1 } else { 0 })
}

fn parse_literal(version: u32, bits: &mut VecDeque<bool>) -> Result<Packet, ()> {
    let mut result = 0;
    loop {
        let mut chunk = bits.drain(0..5);
        let cont = chunk.next().unwrap();

        result = result * 16 + value(chunk);
        if !cont {
            break;
        }
    }

    Ok(Packet::Literal(version, result))
}

fn parse_operator(version: u32, packet_type: u32, bits: &mut VecDeque<bool>) -> Result<Packet, ()> {
    let len_type = bits.drain(0..1).next().unwrap();
    let mut sub_packets = Vec::new();
    if !len_type {
        let len = value(bits.drain(0..15)) as usize;
        let mut sub_slice = bits.drain(0..len).collect::<VecDeque<_>>();
        while !sub_slice.is_empty() {
            sub_packets.push(parse(&mut sub_slice)?);
        }
    } else {
        let count = value(bits.drain(0..11)) as usize;
        for _ in 0..count {
            sub_packets.push(parse(bits)?);
        }
    }
    Ok(Packet::Operator(version, packet_type, sub_packets))
}

fn parse(bits: &mut VecDeque<bool>) -> Result<Packet, ()> {
    let version = value(bits.drain(0..3));
    let packet_type = value(bits.drain(0..3));

    match packet_type {
        4 => parse_literal(version as _, bits),
        _ => parse_operator(version as _, packet_type as _, bits),
    }
}

fn version_sum(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal(version, _) => *version,
        Packet::Operator(version, _, sub_packets) => {
            *version + sub_packets.iter().map(version_sum).sum::<u32>()
        }
    }
}

fn eval_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(_, val) => *val,
        Packet::Operator(_, op_type, sub_packets) => {
            let mut sub_values = sub_packets.iter().map(eval_packet);
            match op_type {
                0 => sub_values.sum::<u64>(),
                1 => sub_values.product::<u64>(),
                2 => sub_values.min().unwrap(),
                3 => sub_values.max().unwrap(),
                5 => (sub_values.next() > sub_values.next()) as u64,
                6 => (sub_values.next() < sub_values.next()) as u64,
                7 => (sub_values.next() == sub_values.next()) as u64,
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    bits: VecDeque<bool>
}

impl Data {
    pub fn part_one(&self) -> u32 {
        let mut bits = self.bits.clone();
        let packet = parse(&mut bits).unwrap();
        version_sum(&packet)
    }

    pub fn part_two(&self) -> u64 {
        let mut bits = self.bits.clone();
        let packet = parse(&mut bits).unwrap();
        eval_packet(&packet)
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bits: input
            .trim()
            .chars()
            .flat_map(|c| {
                let i = i8::from_str_radix(&String::from_iter([c]), 16).unwrap();
                (0..4).map(move |bit| (i & (8 >> bit)) != 0)
            })
            .collect()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example1() {
        let input = "8A004A801A8002F478";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 16);
    }

    #[test]
    fn part_one_example2() {
        let input = "620080001611562C8802118E34";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 12);
    }

    #[test]
    fn part_one_example3() {
        let input = "C0015000016115A2E0802F182340";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 23);
    }

    #[test]
    fn part_one_example4() {
        let input = "A0016C880162017C3686B18A3D4780";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 31);
    }

    #[test]
    fn part_two_sum() {
        let input = "C200B40A82";
        let data: Data = input.parse().unwrap();
        let result = 1 + 2;
        assert_eq!(data.part_two(), result);
    }

    #[test]
    fn part_two_product() {
        let input = "04005AC33890";
        let data: Data = input.parse().unwrap();
        let result = 6 * 9;
        assert_eq!(data.part_two(), result);
    }

    #[test]
    fn part_two_min() {
        let input = "880086C3E88112";
        let data: Data = input.parse().unwrap();
        let result = [7, 8, 9].into_iter().min().unwrap();
        assert_eq!(data.part_two(), result);
    }

    #[test]
    fn part_two_max() {
        let input = "CE00C43D881120";
        let data: Data = input.parse().unwrap();
        let result = [7, 8, 9].into_iter().max().unwrap();
        assert_eq!(data.part_two(), result);
    }

    #[test]
    fn part_two_less_than() {
        let input = "D8005AC2A8F0";
        let data: Data = input.parse().unwrap();
        let result = (5 < 15) as u64;
        assert_eq!(data.part_two(), result);
    }

    #[test]
    fn part_two_greater_than() {
        let input = "F600BC2D8F";
        let data: Data = input.parse().unwrap();
        let result = (5 > 15) as u64;
        assert_eq!(data.part_two(), result);
    }

    #[test]
    fn part_two_equal() {
        let input = "9C005AC2F8F0";
        let data: Data = input.parse().unwrap();
        let result = (5 == 15) as u64;
        assert_eq!(data.part_two(), result);
    }

    #[test]
    fn part_two_example() {
        let input = "9C0141080250320F1802104A08";
        let data: Data = input.parse().unwrap();
        let result = (1 + 3 == 2 * 2) as u64;
        assert_eq!(data.part_two(), result);
    }
}
