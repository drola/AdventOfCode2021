/// Solution to an Advent of Code problem, day 16, 2021
/// https://adventofcode.com/2021/day/16
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, PartialEq)]
enum Packet {
    LiteralValue {
        version: u64,
        contents: u64,
    },
    Operator {
        version: u64,
        operation: Operation,
        contents: Vec<Packet>,
    },
}

fn iter_hex_to_bits<'a, I: std::iter::Iterator<Item = char> + 'a>(
    i: I,
) -> (impl std::iter::Iterator<Item = bool> + 'a) {
    i.flat_map(|hexdec| match hexdec {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'A' => [true, false, true, false],
        'B' => [true, false, true, true],
        'C' => [true, true, false, false],
        'D' => [true, true, false, true],
        'E' => [true, true, true, false],
        'F' => [true, true, true, true],
        _ => [false, false, false, false],
    })
}

fn try_parse_nbit_number<I: std::iter::Iterator<Item = bool>>(i: &mut I, n: usize) -> Option<u64> {
    let (digits_count, number) = i.take(n).fold((0, 0), |(digits_count, number), bit| {
        (digits_count + 1, number * 2 + bit as u64)
    });

    if digits_count == n {
        Some(number)
    } else {
        None
    }
}

fn try_parse_version<I: std::iter::Iterator<Item = bool>>(i: &mut I) -> Option<u64> {
    try_parse_nbit_number(i, 3)
}

fn try_parse_type_id<I: std::iter::Iterator<Item = bool>>(i: &mut I) -> Option<u64> {
    try_parse_nbit_number(i, 3)
}

fn try_parse_literal_packet_contents<I: std::iter::Iterator<Item = bool>>(
    i: &mut I,
) -> Option<u64> {
    let mut ret = 0;
    let mut continue_parsing = true;
    while continue_parsing {
        continue_parsing = i.next()?;
        ret = (ret << 4) + try_parse_nbit_number(i, 4)?;
    }
    Some(ret)
}

fn try_parse_operator_packet_contents<I: std::iter::Iterator<Item = bool>>(
    i: &mut I,
) -> Option<Vec<Packet>> {
    let mut result = vec![];

    let length_type = i.next()?;
    if length_type {
        let number_of_subpackets = try_parse_nbit_number(i, 11)?;
        for _ in 0..number_of_subpackets {
            result.push(try_parse_packet(i)?);
        }
    } else {
        let number_of_bits = try_parse_nbit_number(i, 15)?;
        let mut bits_iter = i
            .take(number_of_bits as usize)
            // Without the following allocation Rust's type system explodes with recursive types
            .collect::<Vec<bool>>()
            .into_iter();
        while let Some(packet) = try_parse_packet(&mut bits_iter) {
            result.push(packet);
        }
    }

    Some(result)
}

impl TryFrom<u64> for Operation {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Sum),
            1 => Ok(Operation::Product),
            2 => Ok(Operation::Minimum),
            3 => Ok(Operation::Maximum),
            5 => Ok(Operation::GreaterThan),
            6 => Ok(Operation::LessThan),
            7 => Ok(Operation::EqualTo),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Packet {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        try_parse_packet(&mut iter_hex_to_bits(s.chars())).ok_or(())
    }
}

fn try_parse_packet<I: std::iter::Iterator<Item = bool>>(i: &mut I) -> Option<Packet> {
    let version = try_parse_version(i)?;
    let type_id = try_parse_type_id(i)?;
    match type_id {
        4 => Some(Packet::LiteralValue {
            version,
            contents: try_parse_literal_packet_contents(i)?,
        }),
        _ => Some(Packet::Operator {
            version,
            operation: Operation::try_from(type_id).ok()?,
            contents: try_parse_operator_packet_contents(i)?,
        }),
    }
}

impl Packet {
    fn versions_sum(&self) -> u64 {
        match self {
            Packet::LiteralValue { version, .. } => *version,
            Packet::Operator {
                version, contents, ..
            } => *version + contents.iter().map(Packet::versions_sum).sum::<u64>(),
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Packet::LiteralValue { contents, .. } => *contents,
            Packet::Operator {
                contents,
                operation,
                ..
            } => {
                let mut evaluated_contents = contents.iter().map(Packet::evaluate);
                match operation {
                    Operation::Sum => evaluated_contents.sum(),
                    Operation::Product => evaluated_contents.product(),
                    Operation::Minimum => evaluated_contents.min().unwrap(),
                    Operation::Maximum => evaluated_contents.max().unwrap(),
                    Operation::GreaterThan => {
                        (evaluated_contents.next() > evaluated_contents.next()) as u64
                    }
                    Operation::LessThan => {
                        (evaluated_contents.next() < evaluated_contents.next()) as u64
                    }
                    Operation::EqualTo => {
                        (evaluated_contents.next() == evaluated_contents.next()) as u64
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let packet = Packet::try_from(contents.as_str()).unwrap();

    println!("[part 1] packet version sum: {}", packet.versions_sum());
    println!("[part 2] evaluated result: {}", packet.evaluate());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() {
        assert_eq!(
            Packet::try_from("D2FE28").unwrap(),
            Packet::LiteralValue {
                version: 6,
                contents: 2021
            }
        );

        assert_eq!(
            Packet::try_from("8A004A801A8002F478").unwrap(),
            Packet::Operator {
                version: 4,
                operation: Operation::Minimum,
                contents: vec![Packet::Operator {
                    version: 1,
                    operation: Operation::Minimum,
                    contents: vec![Packet::Operator {
                        version: 5,
                        operation: Operation::Minimum,
                        contents: vec![Packet::LiteralValue {
                            version: 6,
                            contents: 15
                        }]
                    }]
                }]
            }
        );
    }

    #[test]
    fn test_packet_versions_sum() {
        assert_eq!(Packet::try_from("D2FE28").unwrap().versions_sum(), 6);
        assert_eq!(
            Packet::try_from("8A004A801A8002F478")
                .unwrap()
                .versions_sum(),
            16
        );
        assert_eq!(
            Packet::try_from("620080001611562C8802118E34")
                .unwrap()
                .versions_sum(),
            12
        );
        assert_eq!(
            Packet::try_from("C0015000016115A2E0802F182340")
                .unwrap()
                .versions_sum(),
            23
        );
        assert_eq!(
            Packet::try_from("A0016C880162017C3686B18A3D4780")
                .unwrap()
                .versions_sum(),
            31
        );
    }

    #[test]
    fn test_evaluate_packet() {
        assert_eq!(Packet::try_from("C200B40A82").unwrap().evaluate(), 3);
        assert_eq!(Packet::try_from("04005AC33890").unwrap().evaluate(), 54);
        assert_eq!(Packet::try_from("880086C3E88112").unwrap().evaluate(), 7);
        assert_eq!(Packet::try_from("CE00C43D881120").unwrap().evaluate(), 9);
        assert_eq!(Packet::try_from("D8005AC2A8F0").unwrap().evaluate(), 1);
        assert_eq!(Packet::try_from("F600BC2D8F").unwrap().evaluate(), 0);
        assert_eq!(Packet::try_from("9C005AC2F8F0").unwrap().evaluate(), 0);
        assert_eq!(
            Packet::try_from("9C0141080250320F1802104A08")
                .unwrap()
                .evaluate(),
            1
        );
    }
}
