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

struct GoodBadMapping {
    good_to_bad: HashMap<char, char>,
    bad_to_good: HashMap<char, char>,
}

impl GoodBadMapping {
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

    fn get_good(&self, bad: char) -> char {
        *self.bad_to_good.get(&bad).unwrap()
    }

    fn get_pattern_to_digit(&self, unique_patterns: Vec<Vec<char>>) -> HashMap<String, i32> {
        unique_patterns
            .into_iter()
            .fold(&mut HashMap::new(), |acc, pattern| {
                let mut good_segments = pattern
                    .iter()
                    .map(|&bad_segment| self.get_good(bad_segment))
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
                        good_segments,
                        pattern.iter().collect::<String>()
                    ),
                };

                acc.insert(pattern.iter().collect::<String>(), val);
                acc
            })
            .to_owned()
    }
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|part| {
                part.trim()
                    .split(' ')
                    .map(|pattern| {
                        let mut pattern = pattern.chars().collect::<Vec<_>>();
                        pattern.sort_unstable();
                        pattern
                    })
                    .collect::<Vec<_>>()
            });

            let unique_patterns = parts.next().unwrap();
            let outputs = parts.next().unwrap();
            if parts.next().is_some() {
                panic!("Residue parts found after splitting by |");
            }

            let mut good_bad_mapping = GoodBadMapping::new();

            unique_patterns
                .iter()
                .fold(&mut HashMap::new(), |acc, pattern| {
                    pattern.iter().for_each(|&bad_segment| {
                        let counter = acc.entry(bad_segment).or_insert(0);
                        *counter += 1;
                    });
                    acc
                })
                .iter()
                .for_each(|(&bad_segment, &count)| match count {
                    6 => {
                        good_bad_mapping.add_bad_to_good(bad_segment, 'b');
                    }
                    4 => {
                        good_bad_mapping.add_bad_to_good(bad_segment, 'e');
                    }
                    9 => {
                        good_bad_mapping.add_bad_to_good(bad_segment, 'f');
                    }
                    _ => (), // cannot determine
                });

            fn assign_unique_unresolved_bad_segment(
                unique_patterns: &[Vec<char>],
                good_bad_mapping: &mut GoodBadMapping,
                pattern_length: usize,
                good_segment: char,
            ) {
                let unresolved = unique_patterns
                    .iter()
                    .find(|pattern| pattern.len() == pattern_length)
                    .unwrap()
                    .iter()
                    .filter(|&&bad_segment| !good_bad_mapping.has_bad(bad_segment))
                    .collect::<Vec<_>>();

                if unresolved.len() != 1 {
                    panic!("There are {} unresolved segments", unresolved.len());
                }

                good_bad_mapping.add_bad_to_good(*unresolved[0], good_segment);
            }

            assign_unique_unresolved_bad_segment(&unique_patterns, &mut good_bad_mapping, 2, 'c');
            assign_unique_unresolved_bad_segment(&unique_patterns, &mut good_bad_mapping, 3, 'a');
            assign_unique_unresolved_bad_segment(&unique_patterns, &mut good_bad_mapping, 4, 'd');
            assign_unique_unresolved_bad_segment(&unique_patterns, &mut good_bad_mapping, 5, 'g');

            let pattern_to_digit = good_bad_mapping.get_pattern_to_digit(unique_patterns);

            outputs
                .into_iter()
                .map(|pattern| {
                    *pattern_to_digit
                        .get(&pattern.iter().collect::<String>())
                        .unwrap()
                })
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
