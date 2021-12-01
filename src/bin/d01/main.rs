const ACTUAL_INPUT: &str = include_str!("input.txt");

struct AccumulatorP1 {
    prev_value: Option<i32>,
    count: i32,
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .fold(
            AccumulatorP1 {
                prev_value: None,
                count: 0,
            },
            |acc, current_value| AccumulatorP1 {
                prev_value: Some(current_value),
                count: match acc.prev_value {
                    None => 0,
                    Some(prev_value) => {
                        if let std::cmp::Ordering::Greater = current_value.cmp(&prev_value) {
                            acc.count + 1
                        } else {
                            acc.count
                        }
                    }
                },
            },
        )
        .count
        .to_string()
}

fn p2(input: &str) -> String {
    let numbers = input
        .trim()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut window_sum = vec![];

    for i in 0..numbers.len() - 2 {
        window_sum.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
    }

    let mut count = 0;

    for i in 0..window_sum.len() - 1 {
        if let std::cmp::Ordering::Greater = window_sum[i + 1].cmp(&window_sum[i]) {
            count += 1;
        }
    }

    count.to_string()
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
