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
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
