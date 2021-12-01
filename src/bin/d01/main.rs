const ACTUAL_INPUT: &str = include_str!("input.txt");

fn parse_numbers(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}

fn count_increments(numbers: &[i32]) -> i32 {
    let mut count = 0;

    for i in 0..numbers.len() - 1 {
        if numbers[i + 1] > numbers[i] {
            count += 1;
        }
    }

    count
}

fn p1(input: &str) -> String {
    let numbers = parse_numbers(input);
    count_increments(&numbers).to_string()
}

fn p2(input: &str) -> String {
    let numbers = parse_numbers(input);

    let mut window_sum = vec![];
    for i in 0..numbers.len() - 2 {
        window_sum.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
    }

    count_increments(&window_sum).to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "7");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1832");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "5");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1858");
    }
}
