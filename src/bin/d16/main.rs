const ACTUAL_INPUT: &str = include_str!("input.txt");

struct PacketEvaluation {
    version_sums: u64,
    value: u64,
}

struct BitStream {
    bits: String,
    ptr: usize,
}

impl BitStream {
    fn from_input(input: &str) -> Self {
        Self {
            bits: input
                .trim()
                .chars()
                .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
                .collect::<String>(),
            ptr: 0,
        }
    }

    fn read(&mut self, count: usize) -> u64 {
        let start = self.ptr;
        let end = start + count;

        self.ptr = end;

        u64::from_str_radix(&self.bits[start..end], 2).unwrap()
    }

    fn parse_packet(&mut self) -> PacketEvaluation {
        let version = self.read(3);
        let type_id = self.read(3);

        match type_id {
            4 => {
                let mut value = 0;

                loop {
                    let sub_value = self.read(5);
                    value = value * 16 + (sub_value & 15);

                    if sub_value & 16 == 0 {
                        return PacketEvaluation {
                            version_sums: version,
                            value,
                        };
                    }
                }
            }
            _ => {
                let length_type_id = self.read(1);

                let mut inner_version_sums = 0;
                let mut inner_values = vec![];

                match length_type_id {
                    0 => {
                        let total_length_bits = self.read(15) as usize;
                        let end = self.ptr + total_length_bits;

                        while self.ptr != end {
                            let inner = self.parse_packet();
                            inner_version_sums += inner.version_sums;
                            inner_values.push(inner.value);

                            if self.ptr > end {
                                panic!("Overshot??");
                            }
                        }
                    }
                    1 => {
                        let total_packets = self.read(11);

                        (0..total_packets).for_each(|_| {
                            let inner = self.parse_packet();
                            inner_version_sums += inner.version_sums;
                            inner_values.push(inner.value);
                        });
                    }
                    _ => panic!("Unknown length_type_id {}", length_type_id),
                }

                let result = match type_id {
                    0 => inner_values.into_iter().sum(),
                    1 => inner_values.into_iter().product(),
                    2 => inner_values.into_iter().min().unwrap(),
                    3 => inner_values.into_iter().max().unwrap(),
                    5 => {
                        if inner_values[0] > inner_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if inner_values[0] < inner_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if inner_values[0] == inner_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unknown type_id {}", type_id),
                };

                PacketEvaluation {
                    version_sums: version + inner_version_sums,
                    value: result,
                }
            }
        }
    }

    fn parse_entire_bit_stream(mut self) -> PacketEvaluation {
        self.parse_packet()
    }
}

fn p1(input: &str) -> String {
    BitStream::from_input(input)
        .parse_entire_bit_stream()
        .version_sums
        .to_string()
}

fn p2(input: &str) -> String {
    BitStream::from_input(input)
        .parse_entire_bit_stream()
        .value
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        [
            ("D2FE28", 6),
            ("38006F45291200", 9),
            ("EE00D40C823060", 14),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ]
        .into_iter()
        .for_each(|(input, output)| {
            assert_eq!(p1(input), output.to_string(), "Input is: {}", input);
        })
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "866");
    }

    #[test]
    fn test_p2_sample() {
        [
            ("D2FE28", 2021),
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ]
        .into_iter()
        .for_each(|(input, output)| {
            assert_eq!(p2(input), output.to_string(), "Input is: {}", input);
        })
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1392637195518");
    }
}
