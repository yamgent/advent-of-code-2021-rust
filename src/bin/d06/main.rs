use std::{collections::HashMap, iter};
const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    let mut fishes = input
        .trim()
        .split(',')
        .flat_map(str::parse::<i32>)
        .collect::<Vec<_>>();

    (0..80).for_each(|_| {
        let mut new_fishes = 0;
        fishes.iter_mut().for_each(|fish| {
            if *fish == 0 {
                *fish = 6;
                new_fishes += 1;
            } else {
                *fish -= 1;
            }
        });
        fishes.append(&mut iter::repeat(8).take(new_fishes).collect::<Vec<i32>>());
    });

    fishes.len().to_string()
}

fn solve_efficient(input: &str, days: i64) -> String {
    let mut dp_store: HashMap<i64, i64> = HashMap::new();
    fn dp(days_left: i64, dp_store: &mut HashMap<i64, i64>) -> i64 {
        match dp_store.get(&days_left) {
            Some(value) => *value,
            None => {
                let answer = if days_left < 7 {
                    1
                } else {
                    (0..(days_left / 7))
                        .map(|i| dp(days_left - ((i + 1) * 7) - 2, dp_store))
                        .sum::<i64>()
                        + 1
                };
                dp_store.insert(days_left, answer);
                answer
            }
        }
    }

    input
        .trim()
        .split(',')
        .flat_map(str::parse::<i64>)
        .map(|x| days + (7 - x - 1))
        .map(|x| dp(x, &mut dp_store))
        .sum::<i64>()
        .to_string()
}

fn p2(input: &str) -> String {
    solve_efficient(input, 256)
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "5934");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "374994");
    }

    #[test]
    fn test_p2_sample() {
        // p1 is inefficient version with 80 days
        assert_eq!(solve_efficient(SAMPLE_INPUT, 80), p1(SAMPLE_INPUT));

        assert_eq!(p2(SAMPLE_INPUT), "26984457539");
    }

    #[test]
    fn test_p2_actual() {
        // p1 is inefficient version with 80 days
        assert_eq!(solve_efficient(ACTUAL_INPUT, 80), p1(ACTUAL_INPUT));

        assert_eq!(p2(ACTUAL_INPUT), "1686252324092");
    }
}
