use log::debug;
use std::fs;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let mut bit_string: String = "".to_string();

    for c in data.chars() {
        bit_string += to_binary(c);
    }

    let mut packet = Packet::new(&bit_string);

    packet.parse_packet();

    //debug!("Bit String: {}", bit_string);
    println!("P1 Solution: {}", packet.get_version_sum());
    packet.calculate();
    println!("P2 Solution: {}", packet.packet_literal);
}

struct Packet {
    pub bit_string: String,
    pub version: u8,
    pub packet_type: u8,
    pub bit_length: u8,
    pub length_or_number: u64,
    pub packet_literal: u64,
    pub sub_packets: Vec<Packet>,
}

impl Packet {
    #[allow(clippy::all)]
    pub fn calculate(&mut self) -> u64 {
        if self.packet_type == 4 {
            return self.packet_literal as u64;
        }

        let mut sub_packet_literals: Vec<u64> = Vec::new();
        for packet in &mut self.sub_packets {
            let result = packet.calculate();
            sub_packet_literals.push(result);
        }

        if sub_packet_literals.len() == 1 {
            self.packet_literal = sub_packet_literals.pop().unwrap();
        } else if self.packet_type == 0 {
            self.packet_literal = sub_packet_literals.iter().sum::<u64>();
        } else if self.packet_type == 1 {
            self.packet_literal = sub_packet_literals.iter().product::<u64>();
        } else if self.packet_type == 2 {
            self.packet_literal = sub_packet_literals.iter().min().unwrap().to_owned();
        } else if self.packet_type == 3 {
            self.packet_literal = sub_packet_literals.iter().max().unwrap().to_owned();
        } else if self.packet_type == 5 {
            self.packet_literal =
                if sub_packet_literals.pop().unwrap() < sub_packet_literals.pop().unwrap() {
                    1
                } else {
                    0
                };
        } else if self.packet_type == 6 {
            self.packet_literal =
                if sub_packet_literals.pop().unwrap() > sub_packet_literals.pop().unwrap() {
                    1
                } else {
                    0
                };
        } else if self.packet_type == 7 {
            self.packet_literal =
                if sub_packet_literals.pop().unwrap() == sub_packet_literals.pop().unwrap() {
                    1
                } else {
                    0
                };
        } else {
            eprintln!("ERROR in calcuate");
        }

        self.packet_literal
    }

    pub fn get_version_sum(&self) -> u64 {
        let mut version_sum: u64 = self.version as u64;
        for packet in self.sub_packets.iter() {
            version_sum += packet.get_version_sum();
        }
        version_sum
    }

    pub fn new(bit_string: &str) -> Self {
        Packet {
            bit_string: bit_string.to_string(),
            version: 0,
            packet_type: 0,
            bit_length: 0,
            length_or_number: 0,
            packet_literal: 0,
            sub_packets: Vec::new(),
        }
    }

    pub fn parse_packet(&mut self) -> String {
        let mut left_over_bits = "".to_string();
        if self.bit_string.is_empty()
            || self.bit_string.len() < 6
            || Packet::bit_string_is_zeros(&self.bit_string)
        {
            return left_over_bits;
        }

        self.version = u8::from_str_radix(&self.bit_string[..3], 2).unwrap();
        debug!("Version: {}, {}", &self.bit_string[..3], self.version);
        self.packet_type = u8::from_str_radix(&self.bit_string[3..6], 2).unwrap();
        debug!(
            "packet type: {}, {}",
            &self.bit_string[3..6],
            self.packet_type
        );

        if self.packet_type == 4 {
            let mut literal_bits: String = self.bit_string[6..].to_string();

            let mut number = "".to_string();
            loop {
                let five_bits = literal_bits[0..5].to_string();
                literal_bits = literal_bits[5..].to_string();
                number += five_bits[1..].to_string().as_str();
                if five_bits.starts_with('0') {
                    break;
                }
            }
            self.packet_literal = u64::from_str_radix(&number, 2).unwrap();
            debug!("LITERAL {}", self.packet_literal);
            left_over_bits = literal_bits;
        } else {
            self.bit_length = if u8::from_str_radix(&self.bit_string[6..7], 2).unwrap() == 1 {
                11
            } else {
                15
            };

            debug!("bit length: {}", &self.bit_string[6..7]);
            self.length_or_number =
                u64::from_str_radix(&self.bit_string[7..(7 + self.bit_length) as usize], 2)
                    .unwrap();
            debug!(
                "number of sub packets: {},{}",
                &self.bit_string[7..(7 + self.bit_length) as usize],
                self.length_or_number
            );

            if self.bit_length == 15 {
                let mut sub_packet_bit_string: String = self.bit_string[(7 + self.bit_length
                    as usize)
                    ..(7 + self.bit_length as usize + self.length_or_number as usize)]
                    .to_string();
                while self.bit_string.len() > 6
                    && !self.bit_string.is_empty()
                    && !Packet::bit_string_is_zeros(&sub_packet_bit_string)
                {
                    let mut c_packet = Packet::new(&sub_packet_bit_string);
                    sub_packet_bit_string = c_packet.parse_packet();
                    self.sub_packets.push(c_packet);
                }

                left_over_bits = self.bit_string
                    [(7 + self.bit_length as usize + self.length_or_number as usize)..]
                    .to_string();
            } else {
                let mut remaining_bit_string =
                    self.bit_string[7 + self.bit_length as usize..].to_string();

                for _ in 0..self.length_or_number {
                    let mut c_packet = Packet::new(&remaining_bit_string);
                    remaining_bit_string = c_packet.parse_packet();
                    self.sub_packets.push(c_packet);
                }

                left_over_bits = remaining_bit_string;
            }
        }
        left_over_bits
    }

    fn bit_string_is_zeros(bit_string: &str) -> bool {
        for bit in bit_string.chars() {
            if bit == '1' {
                return false;
            }
        }
        true
    }
}

fn to_binary(c: char) -> &'static str {
    match c {
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
    }
}
