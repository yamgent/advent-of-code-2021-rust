const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    let mut numbers = input
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    numbers.sort_unstable();

    let median = numbers[numbers.len() / 2];

    numbers
        .into_iter()
        .map(|x| (median - x).abs())
        .sum::<i32>()
        .to_string()
}

fn p2(input: &str) -> String {
    let mut numbers = input
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    numbers.sort_unstable();

    let min = numbers[0];
    let max = numbers[numbers.len() - 1];

    (min..max)
        .map(|point| {
            numbers
                .iter()
                .map(|x| {
                    let steps = (point - x).abs();
                    (steps * (steps + 1)) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "37");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "343605");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "168");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "96744904");
    }
}
