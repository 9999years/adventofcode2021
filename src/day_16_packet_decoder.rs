use std::{
    ops::{Add, Shl},
    str::FromStr,
};

use crate::*;
use bitvec::prelude::*;
use tap::Tap;

type Bytes = BitSlice<Msb0, u8>;

fn str_to_bits(data: &str) -> Result<BitVec<Msb0, u8>, String> {
    data.chars()
        .tuples()
        .map(|(hi, lo)| {
            hi.parse()
                .map(|digit| digit << 4)
                .and_then(|byte| lo.parse().map(|digit| byte + digit))
        })
        .collect::<Result<_, _>>()
        .map(|v| BitVec::from_vec(v))
}

const MIN_PACKET_SIZE: usize = 11;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<u8> for Operator {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Operator::*;
        match value {
            0 => Ok(Sum),
            1 => Ok(Product),
            2 => Ok(Minimum),
            3 => Ok(Maximum),
            5 => Ok(GreaterThan),
            6 => Ok(LessThan),
            7 => Ok(EqualTo),
            _ => Err(format!("Bad type ID: {}", value)),
        }
    }
}

impl From<Operator> for u8 {
    fn from(value: Operator) -> Self {
        // Safety: `Operator` is `#[repr(u8)]`
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: u8, // 3 bits
    data: PacketData,
}

impl Packet {
    pub fn get_value(&self) -> usize {
        match &self.data {
            PacketData::Literal(value) => *value,
            PacketData::Operator(operator, inner) => {
                let mut iter = inner.iter().map(Packet::get_value);
                match operator {
                    Operator::Sum => iter.sum(),
                    Operator::Product => iter.product(),
                    Operator::Minimum => iter.min().expect("Min packet has 0 subpackets"),
                    Operator::Maximum => iter.max().expect("Max packet has 0 subpackets"),
                    Operator::GreaterThan => {
                        if iter.next().expect("GT packet has 0 subpackets")
                            > iter.next().expect("GT packet has 1 subpacket, not 2")
                        {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::LessThan => {
                        if iter.next().expect("LT packet has 0 subpackets")
                            < iter.next().expect("LT packet has 1 subpacket, not 2")
                        {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::EqualTo => {
                        if iter.next().expect("Eq packet has 0 subpackets")
                            == iter.next().expect("Eq packet has 1 subpacket, not 2")
                        {
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

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        str_to_bits(s)?.as_bitslice().try_into()
    }
}

#[derive(Debug, PartialEq)]
pub enum PacketData {
    Literal(usize),
    Operator(Operator, Vec<Packet>),
}

#[derive(Debug)]
enum PacketLength {
    Bits(usize),
    Count(usize),
}

fn parse_packets(data: &Bytes, length: PacketLength) -> Result<(Vec<Packet>, &Bytes), String> {
    let mut rest = data;
    let mut ret = Vec::new();

    let should_keep_parsing_packets: Box<dyn Iterator<Item = usize>> = match length {
        PacketLength::Bits(bits) => {
            rest = &rest[..bits];
            Box::new(std::iter::repeat(0))
        }

        PacketLength::Count(count) => {
            ret = Vec::with_capacity(count);
            Box::new(0..count)
        }
    };

    for _ in should_keep_parsing_packets {
        // rest = dbg!(rest);
        if rest.len() < MIN_PACKET_SIZE {
            break;
        }
        let (packet, next_rest) = parse_one_packet(rest)?;
        ret.push(packet);
        rest = next_rest;
    }

    Ok((
        ret,
        match length {
            PacketLength::Bits(bits) => {
                if data.len() > bits {
                    &data[bits..]
                } else {
                    &data[..0]
                }
            }
            PacketLength::Count(_) => rest,
        },
    ))
}

fn parse_one_packet(data: &Bytes) -> Result<(Packet, &Bytes), String> {
    // Why isn't this in bitvec...?
    fn bits_to_int<T>(bits: &Bytes) -> T
    where
        T: Shl<Output = T> + Add<Output = T> + From<u8>,
    {
        bits.iter()
            .by_val()
            .map(|b| if b { 1.into() } else { 0.into() })
            .fold(0.into(), |acc, el| (acc << 1.into()) + el)
    }

    let version = bits_to_int(&data[0..3]);
    let type_id = bits_to_int(&data[3..6]);

    if let 4 = type_id {
        let mut num: usize = 0;
        // Type ID 4 indicates a literal value (single binary number)
        const CHUNK_SIZE: usize = 5;
        let mut chunks = (&data[6..]).chunks(CHUNK_SIZE);
        let mut offset = 6;
        while let Some(chunk) = chunks.next() {
            num <<= 4;
            num += bits_to_int::<usize>(&chunk[1..5]);
            offset += chunk.len();
            if !chunk[0] {
                break;
            }
        }
        let rest = if data[offset..].len() < MIN_PACKET_SIZE {
            &data[..0]
        } else {
            &data[offset..]
        };
        Ok((
            Packet {
                version,
                data: PacketData::Literal(num),
            },
            rest,
        ))
    } else {
        // All other type IDs indicate an operator
        let length_type_id = data[6];
        let rest;
        let length = if length_type_id {
            // 1: next 11 bits represent number of sub-packets
            let subpackets_count: u16 = bits_to_int(&data[7..18]);
            rest = &data[18..];
            PacketLength::Count(subpackets_count as usize)
        } else {
            // 0: next 15 bits represent total length in bits of sub-packets
            let subpackets_size: u16 = bits_to_int(&data[7..22]);
            rest = &data[22..];
            PacketLength::Bits(subpackets_size as usize)
        };
        let (packets, rest) = parse_packets(rest, length)?;
        Ok((
            Packet {
                version,
                data: PacketData::Operator(type_id.try_into()?, packets),
            },
            rest,
        ))
    }
}

impl TryFrom<&BitSlice<Msb0, u8>> for Packet {
    type Error = String;

    fn try_from(data: &BitSlice<Msb0, u8>) -> Result<Self, Self::Error> {
        let (packet, rest) = parse_one_packet(data)?;
        if rest.any() {
            Err(format!("Unparsed data at end of transmission: {:?}", rest))
        } else {
            Ok(packet)
        }
    }
}

type Input = Packet;

pub fn part_1(input: Input) -> usize {
    input.version as usize
        + match input.data {
            PacketData::Literal(_) => 0,
            PacketData::Operator(_, packets) => {
                packets.into_iter().map(|packet| part_1(packet)).sum()
            }
        }
}

pub fn part_2(input: Input) -> usize {
    input.get_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Input {
        input!("day_16_packet_decoder").trim().parse().unwrap()
    }

    fn parse_packet(data: &str) -> Packet {
        data.parse().unwrap()
    }

    #[test]
    fn test_str_to_bits() {
        assert_eq!(
            str_to_bits("D2FE28").unwrap(),
            bitvec![
                Msb0, u8;
                1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1,
                0, 0, 0,
            ]
        );
    }

    #[test]
    fn test_parse() {
        let packet: Packet = parse_packet("D2FE28");
        assert_eq!(
            packet,
            Packet {
                version: 6,
                data: PacketData::Literal(2021)
            }
        );

        let packet: Packet = parse_packet("38006F45291200");
        assert_eq!(
            packet,
            Packet {
                version: 1,
                data: PacketData::Operator(
                    Operator::LessThan,
                    vec![
                        Packet {
                            version: 6,
                            data: PacketData::Literal(10),
                        },
                        Packet {
                            version: 2,
                            data: PacketData::Literal(20),
                        }
                    ]
                )
            }
        );
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(parse_packet("8A004A801A8002F478")), 16);
        assert_eq!(part_1(parse_packet("620080001611562C8802118E34")), 12);
        assert_eq!(part_1(parse_packet("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part_1(parse_packet("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 852);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(parse_packet("C200B40A82")), 3);
        assert_eq!(part_2(parse_packet("04005AC33890")), 54);
        assert_eq!(part_2(parse_packet("880086C3E88112")), 7);
        assert_eq!(part_2(parse_packet("CE00C43D881120")), 9);
        assert_eq!(part_2(parse_packet("D8005AC2A8F0")), 1);
        assert_eq!(part_2(parse_packet("F600BC2D8F")), 0);
        assert_eq!(part_2(parse_packet("9C005AC2F8F0")), 0);
        assert_eq!(part_2(parse_packet("9C0141080250320F1802104A08")), 1);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 19348959966392);
    }
}
