use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|pattern| matches!(pattern.len(), 2 | 3 | 4 | 7))
                .count() as i32
        })
        .sum::<i32>()
        .to_string()
}

fn p2(input: &str) -> String {
    // logic adapted from https://github.com/Fadi88/AoC/blob/master/2021/day08/main.rs
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ").map(|part| {
                part.split(' ')
                    .map(|pattern| HashSet::from_iter(pattern.chars()))
                    .collect::<Vec<_>>()
            });

            let unique_patterns = parts.next().unwrap();
            let outputs = parts.next().unwrap();
            if parts.next().is_some() {
                panic!("Residue parts found after splitting by |");
            }

            let mut mappings: [HashSet<char>; 10] = Default::default();

            let mut remaining_5 = vec![];
            let mut remaining_6 = vec![];

            unique_patterns
                .into_iter()
                .for_each(|pattern| match pattern.len() {
                    2 => mappings[1] = pattern,
                    3 => mappings[7] = pattern,
                    4 => mappings[4] = pattern,
                    7 => mappings[8] = pattern,
                    5 => remaining_5.push(pattern),
                    6 => remaining_6.push(pattern),
                    _ => panic!("Unusual length"),
                });

            remaining_5.into_iter().for_each(|pattern| {
                if mappings[1].difference(&pattern).count() == 0 {
                    mappings[3] = pattern;
                } else if pattern.difference(&mappings[4]).count() == 2 {
                    mappings[5] = pattern;
                } else {
                    mappings[2] = pattern;
                }
            });

            remaining_6.into_iter().for_each(|pattern| {
                if mappings[1].difference(&pattern).count() == 1 {
                    mappings[6] = pattern;
                } else if mappings[5].difference(&pattern).count() == 0 {
                    mappings[9] = pattern;
                } else {
                    mappings[0] = pattern;
                }
            });

            outputs
                .into_iter()
                .map(|x| {
                    for (i, item) in mappings.iter().enumerate() {
                        if *item == x {
                            return i as i32;
                        }
                    }
                    panic!("Cannot find correct mapping for {:?}", x);
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
