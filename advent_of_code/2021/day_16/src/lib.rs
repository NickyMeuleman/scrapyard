// lightly edited solution from:
// https://github.com/vodik/aoc/blob/main/aoc-2021/src/day16.rs
use std::{convert::Infallible, str::FromStr};

struct Reader<'a> {
    input: &'a [u8],
    pos: usize,
    // chunks of 4 bits so bit goes from 3 down to 0 and then resets
    bit: usize,
}

impl<'a> Reader<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            pos: 0,
            bit: 3,
        }
    }

    fn flush(&mut self) {
        self.pos += 1;
        self.bit = 3;
    }

    fn read_bit(&mut self) -> u8 {
        let mask = 1 << self.bit;
        let out = (self.input[self.pos] & mask) >> self.bit;

        if self.bit == 0 {
            self.flush();
        } else {
            self.bit -= 1;
        }

        out
    }

    fn read_byte(&mut self) -> u8 {
        let out = self.input[self.pos];
        self.flush();
        out
    }

    fn read(&mut self, size: usize) -> u32 {
        if size < 4 {
            self.read_small(size)
        } else {
            self.read_large(size)
        }
    }

    fn read_small(&mut self, size: usize) -> u32 {
        let mut out = 0;
        for _ in 0..size {
            out <<= 1;
            out |= self.read_bit() as u32
        }
        out
    }

    fn read_large(&mut self, mut size: usize) -> u32 {
        let mut out = 0;

        // read til 4 byte aligned
        match self.bit {
            0 => {
                out <<= 1;
                out |= self.input[self.pos] as u32 & 0b0001;
                size -= 1;
            }
            1 => {
                out <<= 2;
                out |= self.input[self.pos] as u32 & 0b0011;
                size -= 2;
            }
            2 => {
                out <<= 3;
                out |= self.input[self.pos] as u32 & 0b0111;
                size -= 3;
            }
            3 => {
                out <<= 4;
                out |= self.input[self.pos] as u32;
                size -= 4;
            }
            _ => unreachable!(),
        }
        self.flush();

        // read aligned
        for _ in 0..(size / 4) {
            out <<= 4;
            out |= self.read_byte() as u32;
        }

        // read leftover
        for _ in 0..(size % 4) {
            out <<= 1;
            out |= self.read_bit() as u32
        }

        out
    }

    fn parse_literal(&mut self) -> (Body, usize) {
        let mut read = 0;
        let mut value = 0;
    
        loop {
            let mark = self.read(1);
            value <<= 4;
            value |= self.read(4) as u64;
            read += 5;
    
            if mark == 0 {
                break (Body::Literal(value), read);
            }
        }
    }

    fn parse_form(&mut self, op: Op) -> (Body, usize) {
        let length_encoding = self.read(1);
        if length_encoding == 1 {
            let chunks = self.read(11);
            let mut read = 0;
            let mut packets = Vec::with_capacity(chunks as usize);
    
            for _ in 0..chunks {
                let (packet, size) = self.parse_packet();
                read += size;
                packets.push(packet);
            }
    
            (Body::Form(op, packets), 12 + read)
        } else {
            let length = self.read(15) as usize;
            let mut read = 0usize;
            let mut packets = Vec::new();
    
            while read < length {
                let (packet, size) = self.parse_packet();
                read += size;
                packets.push(packet);
            }
    
            (Body::Form(op, packets), 16 + read)
        }
    }

    fn parse_packet(&mut self) -> (Packet, usize) {
        let version = self.read(3);
        let op = self.read(3);
    
        let mut read = 6;
        let (body, size) = match op {
            0 => self.parse_form(Op::Sum),
            1 => self.parse_form(Op::Product),
            2 => self.parse_form(Op::Min),
            3 => self.parse_form(Op::Max),
            4 => self.parse_literal(),
            5 => self.parse_form(Op::GreaterThan),
            6 => self.parse_form(Op::LessThan),
            7 => self.parse_form(Op::Equal),
            _ => unreachable!(),
        };
        read += size;
    
        (Packet { version, body }, read)
    }
}

#[derive(Debug)]
pub enum Op {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug)]
pub enum Body {
    Literal(u64),
    Form(Op, Vec<Packet>),
}

#[derive(Debug)]
pub struct Packet {
    version: u32,
    body: Body,
}

impl Packet {
    fn parse(bytes: &[u8]) -> Self {
        let mut reader = Reader::new(bytes);
        let (packet, _) = reader.parse_packet();
        packet
    }

    fn sum_versions(&self) -> u32 {
        self.version
            + match &self.body {
                Body::Literal(_) => 0,
                Body::Form(_, body) => body.iter().map(Packet::sum_versions).sum(),
            }
    }

    fn eval(&self) -> u64 {
        match &self.body {
            Body::Literal(value) => *value,
            Body::Form(Op::Sum, body) => body.iter().map(Packet::eval).sum(),
            Body::Form(Op::Product, body) => body.iter().map(Packet::eval).product(),
            Body::Form(Op::Min, body) => body.iter().map(Packet::eval).min().unwrap(),
            Body::Form(Op::Max, body) => body.iter().map(Packet::eval).max().unwrap(),
            Body::Form(Op::GreaterThan, body) => {
                if body[0].eval() > body[1].eval() {
                    1
                } else {
                    0
                }
            }
            Body::Form(Op::LessThan, body) => {
                if body[0].eval() < body[1].eval() {
                    1
                } else {
                    0
                }
            }
            Body::Form(Op::Equal, body) => {
                if body[0].eval() == body[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    bytes: Vec<u8>,
}

impl Data {
    pub fn part_one(&self) -> u32 {
        let packet = Packet::parse(&self.bytes);
        packet.sum_versions()
    }

    pub fn part_two(&self) -> u64 {
        let packet = Packet::parse(&self.bytes);
        packet.eval()
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bytes: input
                .trim()
                .lines()
                .flat_map(|line| {
                    line.chars()
                        .map(|b| u8::try_from(b.to_digit(16).unwrap()).unwrap())
                })
                .collect(),
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
