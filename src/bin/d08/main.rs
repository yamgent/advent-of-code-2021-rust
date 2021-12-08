use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|pattern| matches!(pattern.len(), 2 | 3 | 4 | 7))
                .count() as i32
        })
        .sum::<i32>()
        .to_string()
}

struct SegmentMapping {
    good_to_bad: HashMap<char, char>,
    bad_to_good: HashMap<char, char>,
}

impl SegmentMapping {
    fn new() -> Self {
        Self {
            good_to_bad: HashMap::new(),
            bad_to_good: HashMap::new(),
        }
    }

    fn add_bad_to_good(&mut self, bad: char, good: char) {
        if self.bad_to_good.contains_key(&bad) {
            panic!("Reassigning of bad {}", bad);
        }

        if self.good_to_bad.contains_key(&good) {
            panic!("Reassigning of good {}", good);
        }

        self.bad_to_good.insert(bad, good);
        self.good_to_bad.insert(good, bad);
    }

    fn has_bad(&self, bad: char) -> bool {
        self.bad_to_good.contains_key(&bad)
    }
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line
                .split('|')
                .map(|part| part.trim().split(' ').collect::<Vec<_>>());
            let unique_patterns = parts
                .next()
                .unwrap()
                .into_iter()
                .map(|x| {
                    let mut x = x.chars().collect::<Vec<_>>();
                    x.sort_unstable();
                    x.iter().collect::<String>()
                })
                .collect::<Vec<_>>();
            let outputs = parts.next().unwrap().into_iter().map(|x| {
                let mut x = x.chars().collect::<Vec<_>>();
                x.sort_unstable();
                x.iter().collect::<String>()
            });

            if parts.next().is_some() {
                panic!("Residue parts after splitting |");
            }

            let mut segment_mapping = SegmentMapping::new();

            let faulty_segment_counts = unique_patterns
                .iter()
                .fold(&mut HashMap::new(), |acc, pattern| {
                    pattern.chars().for_each(|c| {
                        let new_count = match acc.get(&c) {
                            Some(&count) => count + 1,
                            None => 1,
                        };
                        acc.insert(c, new_count);
                    });
                    acc
                })
                .to_owned();

            faulty_segment_counts
                .iter()
                .for_each(|(&faulty_segment, &count)| match count {
                    6 => {
                        segment_mapping.add_bad_to_good(faulty_segment, 'b');
                    }
                    4 => {
                        segment_mapping.add_bad_to_good(faulty_segment, 'e');
                    }
                    9 => {
                        segment_mapping.add_bad_to_good(faulty_segment, 'f');
                    }
                    _ => (),
                });

            unique_patterns
                .iter()
                .find(|pattern| pattern.len() == 2)
                .unwrap()
                .chars()
                .for_each(|faulty_segment| {
                    if !segment_mapping.has_bad(faulty_segment) {
                        segment_mapping.add_bad_to_good(faulty_segment, 'c');
                    }
                });

            unique_patterns
                .iter()
                .find(|pattern| pattern.len() == 3)
                .unwrap()
                .chars()
                .for_each(|faulty_segment| {
                    if !segment_mapping.has_bad(faulty_segment) {
                        segment_mapping.add_bad_to_good(faulty_segment, 'a');
                    }
                });
            unique_patterns
                .iter()
                .find(|pattern| pattern.len() == 4)
                .unwrap()
                .chars()
                .for_each(|faulty_segment| {
                    if !segment_mapping.has_bad(faulty_segment) {
                        segment_mapping.add_bad_to_good(faulty_segment, 'd');
                    }
                });
            unique_patterns
                .iter()
                .find(|pattern| pattern.len() == 5)
                .unwrap()
                .chars()
                .for_each(|faulty_segment| {
                    if !segment_mapping.has_bad(faulty_segment) {
                        segment_mapping.add_bad_to_good(faulty_segment, 'g');
                    }
                });

            let bad_segments_to_digit = unique_patterns
                .into_iter()
                .fold(&mut HashMap::new(), |acc, pattern| {
                    let mut good_segments = pattern
                        .chars()
                        .map(|faulty_segment| {
                            *segment_mapping.bad_to_good.get(&faulty_segment).unwrap()
                        })
                        .collect::<Vec<_>>();
                    good_segments.sort_unstable();
                    let good_segments = good_segments.iter().collect::<String>();
                    let val = match good_segments.as_str() {
                        "abcefg" => 0,
                        "cf" => 1,
                        "acdeg" => 2,
                        "acdfg" => 3,
                        "bcdf" => 4,
                        "abdfg" => 5,
                        "abdefg" => 6,
                        "acf" => 7,
                        "abcdefg" => 8,
                        "abcdfg" => 9,
                        _ => panic!(
                            "Cannot find digit for good {}, bad {}",
                            good_segments, pattern
                        ),
                    };
                    acc.insert(pattern, val);
                    acc
                })
                .to_owned();

            outputs
                .into_iter()
                .map(|pattern| *bad_segments_to_digit.get(&pattern).unwrap())
                .fold(0, |acc, digit| acc * 10 + digit)
        })
        .sum::<i32>()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("./example.txt");

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "26");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "512");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "61229");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1091165");
    }
}
