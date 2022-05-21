use crate::AoCData;

pub struct Data {
    bits: Vec<bool>,
}

enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

struct LiteralPacket {
    version: u8,
    val: u64,
}

enum OpType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

struct OperatorPacket {
    version: u8,
    optype: OpType,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn sum_versions(&self) -> u64 {
        match self {
            Packet::Literal(lit) => lit.version as u64,
            Packet::Operator(op) => {
                let sub_sum: u64 = op.subpackets.iter().map(|sub| sub.sum_versions()).sum();
                op.version as u64 + sub_sum
            }
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Packet::Literal(lit) => lit.val,
            Packet::Operator(op) => {
                let mut sub_vals = op.subpackets.iter().map(|sub| sub.eval());
                match op.optype {
                    OpType::Sum => sub_vals.sum(),
                    OpType::Product => sub_vals.product(),
                    OpType::Min => sub_vals.min().unwrap(),
                    OpType::Max => sub_vals.max().unwrap(),
                    OpType::GreaterThan => {
                        if sub_vals.next().unwrap() > sub_vals.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    OpType::LessThan => {
                        if sub_vals.next().unwrap() < sub_vals.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    OpType::EqualTo => {
                        if sub_vals.next().unwrap() == sub_vals.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

fn parse_num(bits: &mut dyn Iterator<Item = bool>, nbits: usize) -> usize {
    // concatenate all bits. With bit logic instead of string concatenation because that's faster
    bits.take(nbits)
        .fold(0, |acc, bit| (acc << 1) | if bit { 1 } else { 0 })
}

fn parse_literal(bits: &mut dyn Iterator<Item = bool>) -> u64 {
    // note: looked for a way to make take_while() work, I'm convinced there is one, but this works so I moved on
    let mut lit_bits = Vec::new();
    // From question: Each group is prefixed by a 1 bit except the last group, which is prefixed by a 0 bit.
    // consume the next bit, if it's a 1 (parsed to true), add the 4 next ones to the literal bits
    while bits.next().unwrap() {
        lit_bits.extend(bits.take(4));
    }

    // the prefix bit was a 0 (so, false), add 4 more bits to the literal bits, then parse that into a decimal number
    lit_bits
        .into_iter()
        .chain(bits.take(4))
        .fold(0, |acc, bit| (acc << 1) | if bit { 1 } else { 0 })
}

fn parse_operator_subpackets(bits: &mut dyn Iterator<Item = bool>) -> Vec<Packet> {
    let mut res = Vec::new();

    // From question:
    // An operator packet contains one or more packets. To indicate which subsequent binary data represents its sub-packets,
    // an operator packet can use one of two modes indicated by the bit immediately after the packet header; this is called the length type ID:
    // If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
    // If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
    let length_type_id = bits.next().unwrap();
    match length_type_id {
        false => {
            let nbits = parse_num(bits, 15);
            let mut sub_bits = bits.take(nbits).peekable();
            while sub_bits.peek().is_some() {
                res.push(parse_packet(&mut sub_bits));
            }
        }
        true => {
            for _ in 0..parse_num(bits, 11) {
                res.push(parse_packet(bits));
            }
        }
    }

    res
}

fn parse_packet(bits: &mut dyn Iterator<Item = bool>) -> Packet {
    // From question: Every packet begins with a standard header: the first three bits encode the packet version, and the next three bits encode the packet type ID.
    let version = parse_num(bits, 3) as u8;
    let type_id = parse_num(bits, 3);

    // Packets with type ID 4 represent a literal value.
    // Every other type of packet (any packet with a type ID other than 4) represent an operator that performs some calculation on one or more sub-packets contained within.
    match type_id {
        4 => Packet::Literal(LiteralPacket {
            version,
            val: parse_literal(bits),
        }),
        _ => Packet::Operator(OperatorPacket {
            version,
            optype: match type_id {
                0 => OpType::Sum,
                1 => OpType::Product,
                2 => OpType::Min,
                3 => OpType::Max,
                5 => OpType::GreaterThan,
                6 => OpType::LessThan,
                7 => OpType::EqualTo,
                _ => unimplemented!(),
            },
            subpackets: parse_operator_subpackets(bits),
        }),
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let bits = input
            .trim()
            .chars()
            .map(|char| {
                // turn a hexadecimal digit into a decimal digit
                let val = char.to_digit(16)?;
                // turn the decimal digit into a vector of booleans representing binary, true for 1, false for 0
                Some(vec![
                    val & 0b1000 != 0,
                    val & 0b0100 != 0,
                    val & 0b0010 != 0,
                    val & 0b0001 != 0,
                ])
            })
            .collect::<Option<Vec<Vec<bool>>>>()?;
        let bits = bits.into_iter().flatten().collect();

        Some(Self { bits })
    }

    fn part_1(&self) -> String {
        let mut bits = self.bits.clone().into_iter();
        // pass a mutable reference to parse_packet, the bits vector gets consumed as it is parsed
        let packet = parse_packet(bits.by_ref());
        let result = packet.sum_versions();

        result.to_string()
    }

    fn part_2(&self) -> String {
        let mut bits = self.bits.clone().into_iter();
        let packet = parse_packet(bits.by_ref());
        let result = packet.eval();

        result.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_example() {
        let input = "D2FE28".to_string();
        let data = Data::try_new(input).unwrap();
        let mut bits = data.bits.into_iter();
        // 110100101111111000101000
        // VVVTTTAAAAABBBBBCCCCC
        // The three bits labeled V (110) are the packet version, 6.
        assert_eq!(parse_num(&mut bits, 3), 6);
        // The three bits labeled T (100) are the packet type ID, 4, which means the packet is a literal value.
        assert_eq!(parse_num(&mut bits, 3), 4);
        // The five bits labeled A (10111) start with a 1 (not the last group, keep reading) and contain the first four bits of the number, 0111.
        // The five bits labeled B (11110) start with a 1 (not the last group, keep reading) and contain four more bits of the number, 1110.
        // The five bits labeled C (00101) start with a 0 (last group, end of packet) and contain the last four bits of the number, 0101.
        // The three unlabeled 0 bits at the end are extra due to the hexadecimal representation and should be ignored.
        // So, this packet represents a literal value with binary representation 011111100101, which is 2021 in decimal.
        assert_eq!(parse_literal(&mut bits), 2021);
    }

    #[test]
    fn part_1() {
        let input = "38006F45291200".to_string();
        let data = Data::try_new(input).unwrap();
        let mut bits = data.bits.into_iter();
        // 00111000000000000110111101000101001010010001001000000000
        // VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
        // The three bits labeled V (001) are the packet version, 1.
        assert_eq!(parse_num(&mut bits, 3), 1);
        // The three bits labeled T (110) are the packet type ID, 6, which means the packet is an operator
        assert_eq!(parse_num(&mut bits, 3), 6);
        // The bit labeled I (0) is the length type ID, which indicates that the length is a 15-bit number representing the number of bits in the sub-packets.
        // The 15 bits labeled L (000000000011011) contain the length of the sub-packets in bits, 27.
        // The 11 bits labeled A contain the first sub-packet, a literal value representing the number 10.
        // The 16 bits labeled B contain the second sub-packet, a literal value representing the number 20.
        let sub_packets = parse_operator_subpackets(&mut bits);

        let val = match sub_packets[0] {
            Packet::Literal(LiteralPacket { val, .. }) => val,
            _ => panic!("sub packet 0 was not a literal"),
        };
        assert_eq!(val, 10);
        let val = match sub_packets[1] {
            Packet::Literal(LiteralPacket { val, .. }) => val,
            _ => panic!("sub packet 1 was not a literal"),
        };
        assert_eq!(val, 20);
    }

    #[test]
    fn part_1_1() {
        let input = "8A004A801A8002F478".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "16");
    }

    #[test]
    fn part_1_2() {
        let input = "620080001611562C8802118E34".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "12");
    }

    #[test]
    fn part_1_3() {
        let input = "C0015000016115A2E0802F182340".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "23");
    }

    #[test]
    fn part_1_4() {
        let input = "A0016C880162017C3686B18A3D4780".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "31");
    }

    #[test]
    fn sum() {
        let input = "C200B40A82".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "3");
    }

    #[test]
    fn product() {
        let input = "04005AC33890".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "54");
    }

    #[test]
    fn min() {
        let input = "880086C3E88112".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "7");
    }

    #[test]
    fn max() {
        let input = "CE00C43D881120".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "9");
    }

    #[test]
    fn less_than() {
        let input = "D8005AC2A8F0".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "1");
    }

    #[test]
    fn greater_than() {
        let input = "F600BC2D8F".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "0");
    }

    #[test]
    fn equal() {
        let input = "9C005AC2F8F0".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "0");
    }

    #[test]
    fn sum_equals_product() {
        let input = "9C0141080250320F1802104A08".to_string();
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "1");
    }
}
