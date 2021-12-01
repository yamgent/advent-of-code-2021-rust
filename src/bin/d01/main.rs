const ACTUAL_INPUT: &str = include_str!("input.txt");

fn parse_numbers(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect()
}

fn p1(input: &str) -> String {
    let numbers = parse_numbers(input);
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .filter(|(x, y)| y > x)
        .count()
        .to_string()
}

fn p2(input: &str) -> String {
    let numbers = parse_numbers(input);
    numbers
        .iter()
        .zip(numbers.iter().skip(3))
        .filter(|(x, y)| y > x)
        .count()
        .to_string()
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
