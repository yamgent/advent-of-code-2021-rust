const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut stack = vec![];

            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    _ => {
                        let expected_closing = stack.pop().unwrap();
                        if c != expected_closing {
                            return match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => panic!("Unknown character {}", c),
                            };
                        }
                    }
                }
            }

            0
        })
        .sum::<i32>()
        .to_string()
}

fn p2(input: &str) -> String {
    let illegal = -1;

    let mut scores = input
        .trim()
        .lines()
        .map(|line| {
            let mut stack = vec![];

            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    _ => {
                        let expected_closing = stack.pop().unwrap();
                        if c != expected_closing {
                            return illegal;
                        }
                    }
                }
            }

            stack.into_iter().rev().fold(0i64, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Unknown character {}", c),
                    }
            })
        })
        .filter(|&score| score != illegal)
        .collect::<Vec<_>>();

    scores.sort_unstable();

    scores[scores.len() / 2].to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "26397");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "389589");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "288957");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1190420163");
    }
}
