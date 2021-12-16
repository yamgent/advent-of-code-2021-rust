const ACTUAL_INPUT: &str = include_str!("input.txt");

fn read(bits: &str, start: usize, count: usize) -> u32 {
    u32::from_str_radix(&bits[start..(start + count)], 2).unwrap()
}

fn read_packet(bits: &str, start: usize, end: usize) -> (u32, usize) {
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

fn p1(input: &str) -> String {
    let bits = input
        .trim()
        .chars()
        .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect::<String>();

    read_packet(&bits, 0, bits.len()).0.to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "";

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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
