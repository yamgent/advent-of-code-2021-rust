use std::{collections::HashMap, iter};
const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    let mut fishes_days_left = input
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    (0..80).for_each(|_| {
        let mut new_fishes_count = 0;
        fishes_days_left.iter_mut().for_each(|fish_days_left| {
            if *fish_days_left == 0 {
                *fish_days_left = 6;
                new_fishes_count += 1;
            } else {
                *fish_days_left -= 1;
            }
        });
        fishes_days_left.append(&mut iter::repeat(8).take(new_fishes_count).collect::<Vec<i32>>());
    });

    fishes_days_left.len().to_string()
}

#[allow(dead_code)] // reason = "Showing an alternative method"
fn solve_efficient_topdown(input: &str, total_days: i64) -> String {
    // using dynamic programming for "compute()"
    let mut compute_cache: HashMap<i64, i64> = HashMap::new();

    // compute how many fishes the current fish and its children, grandchildren,
    // etc can produce, plus it will also count itself in the final output
    //
    // we assume that the fish starts at the beginning of the 6-cycle
    // on the first day (if it isn't, callers must compensate by modifying
    // days_left accordingly)
    fn compute(compute_cache: &mut HashMap<i64, i64>, days_left: i64) -> i64 {
        match compute_cache.get(&days_left) {
            Some(&value) => value,
            None => {
                let answer = if days_left < 7 {
                    1
                } else {
                    (0..(days_left / 7))
                        // -2 to account for the fact that new fishes have a longer cycle of 9,
                        // for the first birth, so they lose two days
                        .map(|i| compute(compute_cache, days_left - ((i + 1) * 7) - 2))
                        .sum::<i64>()
                        + 1
                };
                compute_cache.insert(days_left, answer);
                answer
            }
        }
    }

    input
        .trim()
        .split(',')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .map(|fish_days_left| {
            // our algorithm assumes that in the initial day, everyone
            // is at the start of the cycle (in initial fishes, that is 6).
            // If the current fish isn't so, we lengthen
            // their total days, so that it is as if they started as '6'
            // [hence the (7 - fish_days_left - 1) part], so that
            // their "headstart" is accounted for
            total_days + (7 - fish_days_left - 1)
        })
        .map(|fish_days_left| compute(&mut compute_cache, fish_days_left))
        .sum::<i64>()
        .to_string()
}

// same as the solve_efficient_topdown() version, except
// that the "compute()" (now called answers) is bottom up (i.e. in reverse)
fn solve_efficient_bottomup(input: &str, total_days: i64) -> String {
    let mut answers = HashMap::new();

    (7..(total_days + 7)).for_each(|days_left| {
        answers.insert(
            days_left,
            (0..(days_left / 7))
                .map(|i| {
                    let child_days_left = days_left - ((i + 1) * 7) - 2;
                    if child_days_left < 7 {
                        1
                    } else {
                        *answers.get(&child_days_left).unwrap()
                    }
                })
                .sum::<i64>()
                + 1,
        );
    });

    input
        .trim()
        .split(',')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .map(|fish_days_left| total_days + (7 - fish_days_left - 1))
        .map(|fish_days_left| answers.get(&fish_days_left).unwrap())
        .sum::<i64>()
        .to_string()
}

fn p2(input: &str) -> String {
    solve_efficient_bottomup(input, 256)
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
        assert_eq!(solve_efficient_topdown(SAMPLE_INPUT, 80), p1(SAMPLE_INPUT));
        assert_eq!(solve_efficient_bottomup(SAMPLE_INPUT, 80), p1(SAMPLE_INPUT));

        assert_eq!(p2(SAMPLE_INPUT), "26984457539");
    }

    #[test]
    fn test_p2_actual() {
        // p1 is inefficient version with 80 days
        assert_eq!(solve_efficient_topdown(ACTUAL_INPUT, 80), p1(ACTUAL_INPUT));
        assert_eq!(solve_efficient_bottomup(ACTUAL_INPUT, 80), p1(ACTUAL_INPUT));

        assert_eq!(p2(ACTUAL_INPUT), "1686252324092");
    }
}
