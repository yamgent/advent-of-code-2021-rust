const ACTUAL_INPUT: &str = include_str!("input.txt");

enum SfFlatModified {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SfFlatToken {
    Open,
    Number(i32),
    Close,
}

impl SfFlatToken {
    fn number(&self) -> Option<i32> {
        match self {
            SfFlatToken::Number(num) => Some(*num),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SfFlat {
    tokens: Vec<SfFlatToken>,
}

impl SfFlat {
    fn from_line(line: &str) -> Self {
        let mut tokens = vec![];
        let mut current_number = None;

        for c in line.chars() {
            match c {
                '[' => tokens.push(SfFlatToken::Open),
                '0'..='9' => {
                    let digit = c as i32 - '0' as i32;
                    current_number = Some(match current_number {
                        None => digit,
                        Some(num) => num * 10 + digit,
                    });
                }
                ']' => {
                    if let Some(number) = current_number {
                        tokens.push(SfFlatToken::Number(number));
                        current_number = None;
                    }
                    tokens.push(SfFlatToken::Close);
                }
                ',' => {
                    if let Some(number) = current_number {
                        tokens.push(SfFlatToken::Number(number));
                        current_number = None;
                    }
                }
                _ => panic!("Illegal char '{}'", c),
            }
        }

        if current_number.is_some() {
            panic!("Numbers outside a pair");
        }

        Self { tokens }
    }

    fn explode_once(tokens: &mut Vec<SfFlatToken>) -> SfFlatModified {
        let mut offending_pair_indices = None;

        {
            let mut level = 0;
            for (i, t) in tokens.iter().enumerate() {
                match *t {
                    SfFlatToken::Open => level += 1,
                    SfFlatToken::Close => level -= 1,
                    _ => {}
                }

                if level == 5 {
                    offending_pair_indices = Some((i + 1, i + 2));
                    break;
                }
            }
        }

        match offending_pair_indices {
            None => SfFlatModified::No,
            Some((pair_left_index, pair_right_index)) => {
                let pair_left = tokens[pair_left_index].number().unwrap();
                let pair_right = tokens[pair_right_index].number().unwrap();

                for token in tokens.iter_mut().take(pair_left_index).rev() {
                    if let SfFlatToken::Number(num) = token {
                        *num += pair_left;
                        break;
                    }
                }

                for token in tokens.iter_mut().skip(pair_right_index + 1) {
                    if let SfFlatToken::Number(num) = token {
                        *num += pair_right;
                        break;
                    }
                }

                tokens[pair_left_index - 1] = SfFlatToken::Number(0);

                // delete 'pair_left', 'pair_right', ']'
                for _ in 0..3 {
                    tokens.remove(pair_left_index);
                }

                SfFlatModified::Yes
            }
        }
    }

    fn split_once(tokens: &mut Vec<SfFlatToken>) -> SfFlatModified {
        let mut offending_number_index = None;

        for (i, t) in tokens.iter().enumerate() {
            if let SfFlatToken::Number(num) = t {
                if *num >= 10 {
                    offending_number_index = Some(i);
                    break;
                }
            }
        }

        match offending_number_index {
            None => SfFlatModified::No,
            Some(index) => {
                let num = tokens[index].number().unwrap();
                tokens.remove(index);

                let left = num / 2;
                let right = num / 2 + num % 2;

                // insert the new pair in reverse, because we only have the
                // start index to work with
                tokens.insert(index, SfFlatToken::Close);
                tokens.insert(index, SfFlatToken::Number(right));
                tokens.insert(index, SfFlatToken::Number(left));
                tokens.insert(index, SfFlatToken::Open);

                SfFlatModified::Yes
            }
        }
    }

    fn add(mut self, mut another: SfFlat) -> SfFlat {
        let mut tokens = vec![SfFlatToken::Open];
        tokens.append(&mut self.tokens);
        tokens.append(&mut another.tokens);
        tokens.push(SfFlatToken::Close);

        loop {
            if let SfFlatModified::Yes = SfFlat::explode_once(&mut tokens) {
                continue;
            }

            if let SfFlatModified::Yes = SfFlat::split_once(&mut tokens) {
                continue;
            }

            return SfFlat { tokens };
        }
    }

    fn get_magnitude(&self) -> i32 {
        let mut tokens = self.tokens.clone();

        while tokens.len() != 1 {
            let (pair_start_index, _) = tokens
                .iter()
                .zip(tokens.iter().skip(1))
                .enumerate()
                .find(|(_, (a, b))| a.number().is_some() && b.number().is_some())
                .unwrap();

            let pair_left = tokens[pair_start_index].number().unwrap();
            let pair_right = tokens[pair_start_index + 1].number().unwrap();

            let mag = 3 * pair_left + 2 * pair_right;

            // delete the entire pair ('[', 'left', 'right', ']')
            for _ in 0..4 {
                tokens.remove(pair_start_index - 1);
            }

            // insert mag
            tokens.insert(pair_start_index - 1, SfFlatToken::Number(mag));
        }

        tokens[0].number().unwrap()
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(SfFlat::from_line)
        .reduce(|acc, current| acc.add(current))
        .unwrap()
        .get_magnitude()
        .to_string()
}

fn p2(input: &str) -> String {
    let numbers = input
        .trim()
        .lines()
        .map(SfFlat::from_line)
        .collect::<Vec<_>>();

    numbers
        .iter()
        .map(|x| {
            numbers
                .iter()
                .map(|y| {
                    if x == y {
                        0
                    } else {
                        let x = SfFlat {
                            tokens: x.tokens.clone(),
                        };
                        let y = SfFlat {
                            tokens: y.tokens.clone(),
                        };
                        x.add(y).get_magnitude()
                    }
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_naive() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sfflattoken_number() {
        assert_eq!(SfFlatToken::Open.number(), None);
        assert_eq!(SfFlatToken::Number(42).number(), Some(42));
        assert_eq!(SfFlatToken::Close.number(), None);
    }

    #[test]
    fn test_sfflat_from_line() {
        assert_eq!(
            SfFlat::from_line("[1,2]"),
            SfFlat {
                tokens: vec![
                    SfFlatToken::Open,
                    SfFlatToken::Number(1),
                    SfFlatToken::Number(2),
                    SfFlatToken::Close
                ]
            }
        );

        assert_eq!(
            SfFlat::from_line("[9,[8,7]]"),
            SfFlat {
                tokens: vec![
                    SfFlatToken::Open,
                    SfFlatToken::Number(9),
                    SfFlatToken::Open,
                    SfFlatToken::Number(8),
                    SfFlatToken::Number(7),
                    SfFlatToken::Close,
                    SfFlatToken::Close,
                ]
            }
        );

        assert_eq!(
            SfFlat::from_line("[[1,2],3]"),
            SfFlat {
                tokens: vec![
                    SfFlatToken::Open,
                    SfFlatToken::Open,
                    SfFlatToken::Number(1),
                    SfFlatToken::Number(2),
                    SfFlatToken::Close,
                    SfFlatToken::Number(3),
                    SfFlatToken::Close,
                ]
            }
        );

        assert_eq!(
            SfFlat::from_line("[[1,9],[8,5]]"),
            SfFlat {
                tokens: vec![
                    SfFlatToken::Open,
                    SfFlatToken::Open,
                    SfFlatToken::Number(1),
                    SfFlatToken::Number(9),
                    SfFlatToken::Close,
                    SfFlatToken::Open,
                    SfFlatToken::Number(8),
                    SfFlatToken::Number(5),
                    SfFlatToken::Close,
                    SfFlatToken::Close,
                ]
            }
        );

        // while input won't have double-digit numbers,
        // our test cases does have such numbers
        assert_eq!(
            SfFlat::from_line("[12,345]"),
            SfFlat {
                tokens: vec![
                    SfFlatToken::Open,
                    SfFlatToken::Number(12),
                    SfFlatToken::Number(345),
                    SfFlatToken::Close,
                ]
            }
        )
    }

    #[test]
    fn test_sfflat_explode_once() {
        // this test assumes that SfFlat::from_line() is working correctly
        [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ]
        .into_iter()
        .for_each(|(input_line, expected_output_line)| {
            let mut input = SfFlat::from_line(input_line);
            let expected_output = SfFlat::from_line(expected_output_line);

            let result = SfFlat::explode_once(&mut input.tokens);

            assert!(
                matches!(result, SfFlatModified::Yes),
                "{} did not explode",
                input_line
            );
            assert_eq!(
                input.tokens, expected_output.tokens,
                "{} exploded wrongly",
                input_line
            );
        });
    }

    #[test]
    fn test_sfflat_split_once() {
        // this test assumes that SfFlat::from_line() is working correctly
        [
            ("[10,1]", "[[5,5],1]"),
            ("[11,1]", "[[5,6],1]"),
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ]
        .into_iter()
        .for_each(|(input_line, expected_output_line)| {
            let mut input = SfFlat::from_line(input_line);
            let expected_output = SfFlat::from_line(expected_output_line);

            let result = SfFlat::split_once(&mut input.tokens);

            assert!(
                matches!(result, SfFlatModified::Yes),
                "{} did not split",
                input_line
            );
            assert_eq!(
                input.tokens, expected_output.tokens,
                "{} split wrongly",
                input_line
            );
        });
    }

    #[test]
    fn test_sfflat_add_no_reduce_needed() {
        // this test assumes that SfFlat::from_line() is working correctly
        assert_eq!(
            SfFlat::from_line("[1,2]").add(SfFlat::from_line("[[3,4],5]")),
            SfFlat::from_line("[[1,2],[[3,4],5]]")
        );
    }

    #[test]
    fn test_sfflat_add_with_reduce() {
        // this test assumes that SfFlat::from_line() is working correctly
        assert_eq!(
            SfFlat::from_line("[[[[4,3],4],4],[7,[[8,4],9]]]").add(SfFlat::from_line("[1,1]")),
            SfFlat::from_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_sfflat_add_multiple() {
        // this test assumes that SfFlat::from_line() is working correctly
        [
            (
                r"
[1,1]
[2,2]
[3,3]
[4,4]
",
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            ),
            (
                r"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
",
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            ),
            (
                r"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
",
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            ),
            (
                r"
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ]
        .into_iter()
        .for_each(|(inputs, expected_output_line)| {
            let expected_output = SfFlat::from_line(expected_output_line);

            let result = inputs
                .trim()
                .lines()
                .map(SfFlat::from_line)
                .reduce(|acc, sf| acc.add(sf))
                .unwrap();

            assert_eq!(
                result, expected_output,
                "did not get {}",
                expected_output_line
            );
        });
    }

    #[test]
    fn test_sfnumber_get_magnitude() {
        // this test assumes that SfFlat::from_line() is working correctly
        [
            ("[9,1]", 29),
            ("[1,9]", 21),
            ("[[9,1],[1,9]]", 129),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ]
        .into_iter()
        .for_each(|(input_line, expected_mag)| {
            assert_eq!(
                SfFlat::from_line(input_line).get_magnitude(),
                expected_mag,
                "{} magnitude is wrong",
                input_line
            );
        })
    }

    const SAMPLE_INPUT: &str = r"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "4140");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4008");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "3993");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "4667");
    }
}
