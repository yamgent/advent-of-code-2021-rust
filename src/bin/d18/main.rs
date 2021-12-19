use std::{iter::Peekable, str::Chars};

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq)]
enum SfValue {
    Number(i32),
    Pair(Box<SfNumber>),
}

#[derive(Debug, PartialEq, Eq)]
struct SfNumber {
    left: SfValue,
    right: SfValue,
}

impl SfNumber {
    fn parse_line(line: &str) -> Self {
        fn parse_inner(iter: &mut Peekable<Chars>) -> SfNumber {
            if iter.next().unwrap() != '[' {
                panic!("Expected [");
            }

            let left = match iter.peek().unwrap() {
                '[' => SfValue::Pair(Box::new(parse_inner(iter))),
                '0'..='9' => SfValue::Number(iter.next().unwrap() as i32 - '0' as i32),
                _ => panic!("Unknown char {}", iter.peek().unwrap()),
            };

            if iter.next().unwrap() != ',' {
                panic!("Expected ,");
            }

            let right = match iter.peek().unwrap() {
                '[' => SfValue::Pair(Box::new(parse_inner(iter))),
                '0'..='9' => SfValue::Number(iter.next().unwrap() as i32 - '0' as i32),
                _ => panic!("Unknown char {}", iter.peek().unwrap()),
            };

            if iter.next().unwrap() != ']' {
                panic!("Expected ]");
            }

            SfNumber { left, right }
        }

        parse_inner(&mut line.chars().peekable())
    }

    fn get_magnitude(&self) -> i32 {
        fn get_magnitude_inner(sf: &SfNumber) -> i32 {
            fn extract_value(sf_value: &SfValue) -> i32 {
                match sf_value {
                    SfValue::Number(num) => *num,
                    SfValue::Pair(pair) => get_magnitude_inner(pair.as_ref()),
                }
            }

            3 * extract_value(&sf.left) + 2 * extract_value(&sf.right)
        }

        get_magnitude_inner(self)
    }

    fn extract_numbers_in_pair(sf_value: &SfValue) -> Option<(i32, i32)> {
        if let SfValue::Pair(sf) = sf_value {
            if let SfValue::Number(left) = sf.left {
                if let SfValue::Number(right) = sf.right {
                    return Some((left, right));
                }
            }
        }

        None
    }

    fn add(self, another: SfNumber) -> SfNumber {
        fn reduce(sf: SfNumber, level: i32, residue: (i32, i32)) -> (SfNumber, (i32, i32)) {
            fn explode(left: SfValue, right: SfValue) -> (SfNumber, (i32, i32)) {
                match (left, right) {
                    (SfValue::Number(num), SfValue::Pair(pair)) => {
                        let (left, right) = SfNumber::extract_numbers_in_pair(&right).unwrap();
                        (
                            SfNumber {
                                left: SfValue::Number(num + left),
                                right: SfValue::Number(0),
                            },
                            (0, right),
                        )
                    }
                    (SfValue::Pair(pair), SfValue::Number(num)) => {
                        let (left, right) = SfNumber::extract_numbers_in_pair(&left).unwrap();
                        (
                            SfNumber {
                                left: SfValue::Number(0),
                                right: SfValue::Number(num + right),
                            },
                            (left, 0),
                        )
                    }
                    _ => (SfNumber { left, right }, (0, 0)),
                }
            }

            fn split(
                sf_value: SfValue,
                level: i32,
                residue: (i32, i32),
                residue_current: i32,
            ) -> SfValue {
                match sf_value {
                    SfValue::Number(num) => {
                        if num >= 10 {
                            SfValue::Pair(Box::new(SfNumber {
                                left: SfValue::Number(num / 2),
                                right: SfValue::Number(num / 2 + num % 2),
                            }))
                        } else {
                            sf_value
                        }
                    }
                    SfValue::Pair(sf) => SfValue::Pair(Box::new(reduce(*sf, level + 1, residue))),
                }
            }

            let left = split(sf.left, level, residue, residue.0);
            let right = split(sf.right, level, residue, residue.1);

            if level == 4 {
                explode(left, right)
            } else {
                (SfNumber { left, right }, (0, 0))
            }
        }

        SfNumber {
            left: SfValue::Pair(Box::new(reduce(self, 1, (0, 0)))),
            right: SfValue::Pair(Box::new(reduce(another, 1, (0, 0)))),
        }

        // TODO: Delete this
        // self
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(SfNumber::parse_line)
        .reduce(|acc, current| acc.add(current))
        .unwrap()
        .get_magnitude()
        .to_string()
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

    #[test]
    fn test_sfnumber_parse_line() {
        assert_eq!(
            SfNumber::parse_line("[1,2]"),
            SfNumber {
                left: SfValue::Number(1),
                right: SfValue::Number(2)
            }
        );

        assert_eq!(
            SfNumber::parse_line("[9,[8,7]]"),
            SfNumber {
                left: SfValue::Number(9),
                right: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(8),
                    right: SfValue::Number(7)
                }))
            }
        );

        assert_eq!(
            SfNumber::parse_line("[[1,2],3]"),
            SfNumber {
                left: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(1),
                    right: SfValue::Number(2)
                })),
                right: SfValue::Number(3)
            }
        );

        assert_eq!(
            SfNumber::parse_line("[[1,9],[8,5]]"),
            SfNumber {
                left: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(1),
                    right: SfValue::Number(9)
                })),
                right: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(8),
                    right: SfValue::Number(5)
                })),
            }
        );
    }

    #[test]
    fn test_sfnumber_get_magnitude() {
        assert_eq!(
            SfNumber {
                left: SfValue::Number(9),
                right: SfValue::Number(1)
            }
            .get_magnitude(),
            29
        );

        assert_eq!(
            SfNumber {
                left: SfValue::Number(1),
                right: SfValue::Number(9)
            }
            .get_magnitude(),
            21
        );

        assert_eq!(
            SfNumber {
                left: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(9),
                    right: SfValue::Number(1)
                })),
                right: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(1),
                    right: SfValue::Number(9)
                }))
            }
            .get_magnitude(),
            129
        );

        // for the following tests, we assume
        // that SfNumber::parse_line() is working
        // as intended
        assert_eq!(
            SfNumber::parse_line("[[1,2],[[3,4],5]]").get_magnitude(),
            143
        );
        assert_eq!(
            SfNumber::parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").get_magnitude(),
            1384
        );
        assert_eq!(
            SfNumber::parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]").get_magnitude(),
            445
        );
        assert_eq!(
            SfNumber::parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]").get_magnitude(),
            791
        );
        assert_eq!(
            SfNumber::parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]").get_magnitude(),
            1137
        );
        assert_eq!(
            SfNumber::parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .get_magnitude(),
            3488
        );
    }

    #[test]
    fn test_sfnumber_extract_numbers_in_pair() {
        assert_eq!(
            SfNumber::extract_numbers_in_pair(&SfValue::Pair(Box::new(SfNumber {
                left: SfValue::Number(1),
                right: SfValue::Number(2),
            }))),
            Some((1, 2))
        );

        assert_eq!(SfNumber::extract_numbers_in_pair(&SfValue::Number(0)), None);
        assert_eq!(
            SfNumber::extract_numbers_in_pair(&SfValue::Pair(Box::new(SfNumber {
                left: SfValue::Number(1),
                right: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(1234),
                    right: SfValue::Number(1234),
                }))
            }))),
            None
        );
        assert_eq!(
            SfNumber::extract_numbers_in_pair(&SfValue::Pair(Box::new(SfNumber {
                left: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(1234),
                    right: SfValue::Number(1234),
                })),
                right: SfValue::Number(1),
            }))),
            None
        );
        assert_eq!(
            SfNumber::extract_numbers_in_pair(&SfValue::Pair(Box::new(SfNumber {
                left: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(1234),
                    right: SfValue::Number(1234),
                })),
                right: SfValue::Pair(Box::new(SfNumber {
                    left: SfValue::Number(1234),
                    right: SfValue::Number(1234),
                })),
            }))),
            None
        );
    }

    const SAMPLE_INPUT: &str = r"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "4140");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "");
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
