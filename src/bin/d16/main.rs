const ACTUAL_INPUT: &str = include_str!("input.txt");

fn read(bits: &str, start: usize, count: usize) -> u64 {
    u64::from_str_radix(&bits[start..(start + count)], 2).unwrap()
}

fn read_packet(bits: &str, start: usize, end: usize) -> (u64, usize) {
    let mut ptr = start;

    let version = read(bits, ptr, 3);
    ptr += 3;
    let type_id = read(bits, ptr, 3);
    ptr += 3;

    match type_id {
        4 => loop {
            let value = read(bits, ptr, 5);
            ptr += 5;

            if value & 16 == 0 {
                return (version, ptr);
            }
        },
        _ => {
            let length_type_id = read(bits, ptr, 1);
            ptr += 1;

            match length_type_id {
                0 => {
                    let total_length_bits = read(bits, ptr, 15) as usize;
                    ptr += 15;
                    let end = ptr + total_length_bits;

                    let mut inner_version_sum = 0;

                    while ptr != end {
                        let (inner_version, inner_end) = read_packet(bits, ptr, end);
                        inner_version_sum += inner_version;
                        ptr = inner_end;

                        if ptr > end {
                            panic!("Overshot??");
                        }
                    }

                    (version + inner_version_sum, end)
                }
                1 => {
                    let total_packets = read(bits, ptr, 11);
                    ptr += 11;

                    let mut inner_version_sum = 0;

                    (0..total_packets).for_each(|_| {
                        let (inner_version, inner_end) = read_packet(bits, ptr, end);
                        inner_version_sum += inner_version;
                        ptr = inner_end;

                        if ptr > end {
                            panic!("Overshot??");
                        }
                    });

                    (version + inner_version_sum, ptr)
                }
                _ => panic!("Unknown length_type_id {}", length_type_id),
            }
        }
    }
}

fn get_bits_string(input: &str) -> String {
    input
        .trim()
        .chars()
        .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect::<String>()
}

fn p1(input: &str) -> String {
    let bits = get_bits_string(input);
    read_packet(&bits, 0, bits.len()).0.to_string()
}

fn read_packet_p2(bits: &str, start: usize, end: usize) -> (u64, usize) {
    let mut ptr = start;

    let _version = read(bits, ptr, 3);
    ptr += 3;
    let type_id = read(bits, ptr, 3);
    ptr += 3;

    match type_id {
        4 => {
            let mut whole_value = 0;
            loop {
                let value = read(bits, ptr, 5);
                whole_value = whole_value * 16 + (value & 15);

                ptr += 5;

                if value & 16 == 0 {
                    return (whole_value, ptr);
                }
            }
        }
        _ => {
            let length_type_id = read(bits, ptr, 1);
            ptr += 1;

            let mut inner_values = vec![];

            match length_type_id {
                0 => {
                    let total_length_bits = read(bits, ptr, 15) as usize;
                    ptr += 15;
                    let end = ptr + total_length_bits;

                    while ptr != end {
                        let (inner_value, inner_end) = read_packet_p2(bits, ptr, end);
                        inner_values.push(inner_value);
                        ptr = inner_end;

                        if ptr > end {
                            panic!("Overshot??");
                        }
                    }
                }
                1 => {
                    let total_packets = read(bits, ptr, 11);
                    ptr += 11;

                    (0..total_packets).for_each(|_| {
                        let (inner_value, inner_end) = read_packet_p2(bits, ptr, end);
                        inner_values.push(inner_value);
                        ptr = inner_end;

                        if ptr > end {
                            panic!("Overshot??");
                        }
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

            (result, ptr)
        }
    }
}

fn p2(input: &str) -> String {
    let bits = get_bits_string(input);
    read_packet_p2(&bits, 0, bits.len()).0.to_string()
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
