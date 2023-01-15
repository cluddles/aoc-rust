extern crate aoc_lib;

use anyhow::{anyhow, bail, Result};

use aoc_lib::harness::*;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub struct Day16;

type Input = String;
type Output = u64;

impl Solution<Input, Output> for Day16 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Packet Decoder", 2021, 16)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        Ok(hex_to_bit_str(&resource.as_str()?))
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(Bits::new(input).read_packet()?.sum_versions())
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Bits::new(input).read_packet()?.eval()
    }
}

/// Convert hex string to bit string representation
fn hex_to_bit_str(hex: &str) -> String {
    hex.chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        })
        .collect()
}

/// Convert bit string to numeric representation
fn bit_str_to_val(bits: &str) -> u64 {
    bits.chars().fold(0, |acc, x| acc * 2 + u64::from(x == '1'))
}

/// Packet Header struct
struct Header {
    version: u8,
    type_id: PType,
}

/// Packet types
#[derive(FromPrimitive, Debug, Copy, Clone)]
enum PType {
    Sum = 0,
    Product,
    Min,
    Max,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

/// Packet representation
enum Packet {
    Literal(Header, u64),
    Operator(Header, Vec<Packet>),
}

impl Packet {
    /// Sum versions of this packet and any sub-packets
    fn sum_versions(&self) -> u64 {
        match self {
            Packet::Literal(header, _) => header.version as u64,
            Packet::Operator(header, sub_packets) => {
                header.version as u64 + sub_packets.iter().map(|x| x.sum_versions()).sum::<u64>()
            }
        }
    }

    /// Evaluate this packet's value
    fn eval(&self) -> Result<u64> {
        match self {
            Packet::Literal(_, val) => Ok(*val),
            Packet::Operator(header, sub) => match header.type_id {
                PType::Sum => sub.iter().map(|x| x.eval()).sum(),
                PType::Product => sub.iter().map(|x| x.eval()).product(),
                PType::Min => sub
                    .iter()
                    .map(|x| x.eval())
                    .try_fold(None, |acc: Option<u64>, x| {
                        x.map(|ok| if let Some(v) = acc { Some(v.min(ok)) } else { Some(ok) })
                    })?
                    .ok_or_else(|| anyhow!("No min found")),
                PType::Max => sub
                    .iter()
                    .map(|x| x.eval())
                    .try_fold(None, |acc: Option<u64>, x| {
                        x.map(|ok| if let Some(v) = acc { Some(v.max(ok)) } else { Some(ok) })
                    })?
                    .ok_or_else(|| anyhow!("No max found")),
                PType::Literal => bail!("Packet::Operator cannot use PacketType::Literal"),
                PType::GreaterThan => self.eval_op_sub_packet_pair(sub, u64::gt),
                PType::LessThan => self.eval_op_sub_packet_pair(sub, u64::lt),
                PType::EqualTo => self.eval_op_sub_packet_pair(sub, u64::eq),
            },
        }
    }

    /// Evaluate an operator packet where the operation expects exactly 2 sub-packets
    fn eval_op_sub_packet_pair(&self, sub: &[Packet], f: fn(&u64, &u64) -> bool) -> Result<u64> {
        if sub.len() != 2 {
            bail!("Expected 2 sub-packets");
        }
        Ok(u64::from(f(&sub[0].eval()?, &sub[1].eval()?)))
    }
}

/// Simple bit string consumer
struct Bits {
    str: String,
    pos: usize,
}

impl Bits {
    fn new(str: &str) -> Bits {
        Bits { str: str.to_string(), pos: 0 }
    }

    /// Take the next X bits and move the current position accordingly
    fn take(&mut self, bits: usize) -> &str {
        let result = &self.str[self.pos..self.pos + bits];
        self.pos += bits;
        result
    }

    /// Read the next packet
    fn read_packet(&mut self) -> Result<Packet> {
        let header = Header {
            version: bit_str_to_val(self.take(3)) as u8,
            type_id: PType::from_u64(bit_str_to_val(self.take(3)))
                .ok_or_else(|| anyhow!("Unrecognised PacketType"))?,
        };
        match header.type_id {
            PType::Literal => Ok(Packet::Literal(header, self.read_literal()?)),
            _ => Ok(Packet::Operator(header, self.read_operator_packets()?)),
        }
    }

    /// Read the next literal value
    fn read_literal(&mut self) -> Result<u64> {
        let mut result = String::new();
        for _ in (0..).step_by(5) {
            let id = bit_str_to_val(self.take(1));
            result.push_str(self.take(4));
            if id == 0 {
                return Ok(bit_str_to_val(&result));
            }
        }
        bail!("Could not parse literal");
    }

    /// Read operator sub-packets, with behaviour determines by length type id bit
    fn read_operator_packets(&mut self) -> Result<Vec<Packet>> {
        let len_type_id = bit_str_to_val(self.take(1));
        match len_type_id {
            0 => {
                let len = bit_str_to_val(self.take(15));
                self.read_sub_packets_by_len(len)
            }
            _ => {
                let amt = bit_str_to_val(self.take(11));
                self.read_sub_packets_by_amt(amt)
            }
        }
    }

    /// Read sub-packets with given total length
    fn read_sub_packets_by_len(&mut self, len: u64) -> Result<Vec<Packet>> {
        let start = self.pos;
        let mut result = Vec::new();
        while self.pos < start + len as usize {
            result.push(self.read_packet()?);
        }
        Ok(result)
    }

    /// Read the given number of sub-packets
    fn read_sub_packets_by_amt(&mut self, amt: u64) -> Result<Vec<Packet>> {
        (0..amt).map(|_| self.read_packet()).collect::<Result<_, _>>()
    }
}

fn main() -> Result<()> {
    run_solution(&Day16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bits() {
        assert_eq!(hex_to_bit_str("D2FE28"), "110100101111111000101000");
    }

    #[test]
    fn test_bits_to_val() {
        assert_eq!(bit_str_to_val("110"), 6);
        assert_eq!(bit_str_to_val("011111100101"), 2021);
    }

    #[test]
    fn part1_1() {
        assert_eq!(test_inline(&Day16, SolutionPart::One, "8A004A801A8002F478"), 16);
    }
    #[test]
    fn part1_2() {
        assert_eq!(test_inline(&Day16, SolutionPart::One, "620080001611562C8802118E34"), 12);
    }
    #[test]
    fn part1_3() {
        assert_eq!(test_inline(&Day16, SolutionPart::One, "C0015000016115A2E0802F182340"), 23);
    }
    #[test]
    fn part1_4() {
        assert_eq!(test_inline(&Day16, SolutionPart::One, "A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn part2_1() {
        assert_eq!(test_inline(&Day16, SolutionPart::Two, "C200B40A82"), 3);
    }
    #[test]
    fn part2_2() {
        assert_eq!(test_inline(&Day16, SolutionPart::Two, "04005AC33890"), 54);
    }
    #[test]
    fn part2_3() {
        assert_eq!(test_inline(&Day16, SolutionPart::Two, "880086C3E88112"), 7);
    }
    #[test]
    fn part2_4() {
        assert_eq!(test_inline(&Day16, SolutionPart::Two, "CE00C43D881120"), 9);
    }
    #[test]
    fn part2_5() {
        assert_eq!(test_inline(&Day16, SolutionPart::Two, "D8005AC2A8F0"), 1);
    }
    #[test]
    fn part2_6() {
        assert_eq!(test_inline(&Day16, SolutionPart::Two, "F600BC2D8F"), 0);
    }
    #[test]
    fn part2_7() {
        assert_eq!(test_inline(&Day16, SolutionPart::Two, "9C005AC2F8F0"), 0);
    }
    #[test]
    fn part2_8() {
        assert_eq!(test_inline(&Day16, SolutionPart::Two, "9C0141080250320F1802104A08"), 1);
    }
}
