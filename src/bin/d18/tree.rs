use std::{iter::Peekable, str::Chars};

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Debug)]
enum SfTreeReduced {
    Yes,
    No,
}

#[derive(PartialEq, Eq, Debug)]
enum SfNode {
    Number(i32),
    Pair {
        left: Box<SfNode>,
        right: Box<SfNode>,
    },
}

impl SfNode {
    fn get_size(&self) -> i32 {
        match self {
            SfNode::Number(_) => 1,
            SfNode::Pair { left, right } => left.get_size() + right.get_size(),
        }
    }

    fn is_number(&self) -> bool {
        matches!(self, SfNode::Number(_))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct SfTree {
    root: SfNode,
}

impl SfTree {
    fn from_line(line: &str) -> Self {
        fn parse_number(iter: &mut Peekable<Chars>) -> SfNode {
            let mut number = 0;

            while matches!(iter.peek(), Some('0'..='9')) {
                number = number * 10 + (iter.next().unwrap() as i32 - '0' as i32);
            }

            SfNode::Number(number)
        }

        fn parse_pair(iter: &mut Peekable<Chars>) -> SfNode {
            fn expect(iter: &mut Peekable<Chars>, expected: char) {
                let char = iter.next().unwrap();
                if char != expected {
                    panic!("Expected '{}', found '{}'", expected, char);
                }
            }

            fn parse_part(iter: &mut Peekable<Chars>) -> SfNode {
                match iter.peek() {
                    Some('[') => parse_pair(iter),
                    Some('0'..='9') => parse_number(iter),
                    Some(c) => panic!("Expected '[' or digit, found '{}'", c),
                    None => panic!("Expected part, found EOF"),
                }
            }

            expect(iter, '[');
            let left = Box::new(parse_part(iter));
            expect(iter, ',');
            let right = Box::new(parse_part(iter));
            expect(iter, ']');

            SfNode::Pair { left, right }
        }

        SfTree {
            root: parse_pair(&mut line.chars().peekable()),
        }
    }

    fn get_nth_mut(&mut self, nth: i32) -> Option<&mut SfNode> {
        fn get_nth_inner(node: &mut SfNode, nth: i32) -> Option<&mut SfNode> {
            match node {
                SfNode::Number(_) => {
                    if nth == 0 {
                        Some(node)
                    } else {
                        None
                    }
                }
                SfNode::Pair { left, right } => {
                    let left_size = left.get_size();
                    if nth < left_size {
                        get_nth_inner(left, nth)
                    } else {
                        get_nth_inner(right, nth - left_size)
                    }
                }
            }
        }

        get_nth_inner(&mut self.root, nth)
    }

    fn make_nth_parent_zero(&mut self, nth: i32) -> SfTreeReduced {
        fn zero_inner(node: &mut SfNode, nth: i32) -> SfTreeReduced {
            match node {
                SfNode::Number(_) => SfTreeReduced::No,
                SfNode::Pair { left, right } => {
                    let left_size = left.get_size();

                    let left_is_number = left.is_number();
                    let right_is_number = right.is_number();

                    if nth == 0 && left_is_number || nth == left_size && right_is_number {
                        *node = SfNode::Number(0);
                        SfTreeReduced::Yes
                    } else if nth < left_size {
                        zero_inner(left, nth)
                    } else {
                        zero_inner(right, nth - left_size)
                    }
                }
            }
        }

        zero_inner(&mut self.root, nth)
    }

    fn explode_once(&mut self) -> SfTreeReduced {
        fn get_offending_pair_first_index(
            node: &SfNode,
            left_most_index: i32,
            level: i32,
        ) -> Option<i32> {
            if level == 5 {
                Some(left_most_index)
            } else {
                match node {
                    SfNode::Number(_) => None,
                    SfNode::Pair { left, right } => {
                        let left_size = left.get_size();
                        let left_result =
                            get_offending_pair_first_index(left, left_most_index, level + 1);

                        if left_result.is_some() {
                            left_result
                        } else {
                            get_offending_pair_first_index(
                                right,
                                left_most_index + left_size,
                                level + 1,
                            )
                        }
                    }
                }
            }
        }

        match get_offending_pair_first_index(&self.root, 0, 0) {
            None => SfTreeReduced::No,
            Some(index) => {
                // TODO: Refactor?
                let pair_left = match self.get_nth_mut(index).unwrap() {
                    SfNode::Pair { .. } => panic!("Expected number"),
                    SfNode::Number(num) => *num,
                };
                let pair_right = match self.get_nth_mut(index + 1).unwrap() {
                    SfNode::Pair { .. } => panic!("Expected number"),
                    SfNode::Number(num) => *num,
                };

                match self.get_nth_mut(index - 1) {
                    None => {}
                    Some(SfNode::Number(num)) => *num += pair_left,
                    Some(SfNode::Pair { .. }) => panic!("Expected number"),
                }
                match self.get_nth_mut(index + 2) {
                    None => {}
                    Some(SfNode::Number(num)) => *num += pair_right,
                    Some(SfNode::Pair { .. }) => panic!("Expected number"),
                }

                self.make_nth_parent_zero(index);
                SfTreeReduced::Yes
            }
        }
    }

    fn split_once(&mut self) -> SfTreeReduced {
        fn handle_pair(node: &mut SfNode) -> SfTreeReduced {
            match node {
                SfNode::Number(num) => {
                    if *num >= 10 {
                        let left = *num / 2;
                        let right = *num / 2 + *num % 2;

                        *node = SfNode::Pair {
                            left: Box::new(SfNode::Number(left)),
                            right: Box::new(SfNode::Number(right)),
                        };

                        SfTreeReduced::Yes
                    } else {
                        SfTreeReduced::No
                    }
                }
                SfNode::Pair { left, right } => {
                    if let SfTreeReduced::Yes = handle_pair(left) {
                        return SfTreeReduced::Yes;
                    }

                    handle_pair(right)
                }
            }
        }

        handle_pair(&mut self.root)
    }

    fn add(self, another: SfTree) -> SfTree {
        let mut new_tree = SfTree {
            root: SfNode::Pair {
                left: Box::new(self.root),
                right: Box::new(another.root),
            },
        };

        loop {
            if let SfTreeReduced::Yes = new_tree.explode_once() {
                continue;
            }

            if let SfTreeReduced::Yes = new_tree.split_once() {
                continue;
            }

            return new_tree;
        }
    }

    fn get_magnitude(&self) -> i32 {
        fn get_inner(node: &SfNode, multiplier: i32) -> i32 {
            multiplier
                * match node {
                    SfNode::Number(num) => *num,
                    SfNode::Pair { left, right } => get_inner(left, 3) + get_inner(right, 2),
                }
        }

        get_inner(&self.root, 1)
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(SfTree::from_line)
        .reduce(|acc, current| acc.add(current))
        .unwrap()
        .get_magnitude()
        .to_string()
}

fn p2(input: &str) -> String {
    let numbers = input.trim().lines().collect::<Vec<_>>();

    numbers
        .iter()
        .map(|x| {
            numbers
                .iter()
                .map(|y| {
                    if x == y {
                        0
                    } else {
                        let x = SfTree::from_line(x);
                        let y = SfTree::from_line(y);
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

pub fn solve_tree() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sfnode_get_size() {
        assert_eq!(SfNode::Number(3).get_size(), 1);
        assert_eq!(
            SfNode::Pair {
                left: Box::new(SfNode::Number(2)),
                right: Box::new(SfNode::Pair {
                    left: Box::new(SfNode::Number(1)),
                    right: Box::new(SfNode::Number(3))
                })
            }
            .get_size(),
            3
        );
    }

    #[test]
    fn test_sftree_from_line() {
        assert_eq!(
            SfTree::from_line("[1,2]"),
            SfTree {
                root: SfNode::Pair {
                    left: Box::new(SfNode::Number(1)),
                    right: Box::new(SfNode::Number(2)),
                }
            }
        );

        assert_eq!(
            SfTree::from_line("[9,[8,7]]"),
            SfTree {
                root: SfNode::Pair {
                    left: Box::new(SfNode::Number(9)),
                    right: Box::new(SfNode::Pair {
                        left: Box::new(SfNode::Number(8)),
                        right: Box::new(SfNode::Number(7)),
                    })
                }
            }
        );

        assert_eq!(
            SfTree::from_line("[[1,2],3]"),
            SfTree {
                root: SfNode::Pair {
                    left: Box::new(SfNode::Pair {
                        left: Box::new(SfNode::Number(1)),
                        right: Box::new(SfNode::Number(2)),
                    }),
                    right: Box::new(SfNode::Number(3))
                }
            }
        );

        assert_eq!(
            SfTree::from_line("[[1,9],[8,5]]"),
            SfTree {
                root: SfNode::Pair {
                    left: Box::new(SfNode::Pair {
                        left: Box::new(SfNode::Number(1)),
                        right: Box::new(SfNode::Number(9))
                    }),
                    right: Box::new(SfNode::Pair {
                        left: Box::new(SfNode::Number(8)),
                        right: Box::new(SfNode::Number(5))
                    })
                }
            }
        );

        // while input won't have double-digit numbers,
        // our test cases does have such numbers
        assert_eq!(
            SfTree::from_line("[12,345]"),
            SfTree {
                root: SfNode::Pair {
                    left: Box::new(SfNode::Number(12)),
                    right: Box::new(SfNode::Number(345))
                }
            }
        )
    }

    #[test]
    fn test_sftree_get_nth_mut() {
        // this test assumes that SfTree::from_line() is working correctly
        let mut tree = SfTree::from_line("[[1,[2,3]],[[4,5],6]]");
        assert_eq!(tree.get_nth_mut(-1), None);
        assert_eq!(tree.get_nth_mut(0), Some(&mut SfNode::Number(1)));
        assert_eq!(tree.get_nth_mut(1), Some(&mut SfNode::Number(2)));
        assert_eq!(tree.get_nth_mut(2), Some(&mut SfNode::Number(3)));
        assert_eq!(tree.get_nth_mut(3), Some(&mut SfNode::Number(4)));
        assert_eq!(tree.get_nth_mut(4), Some(&mut SfNode::Number(5)));
        assert_eq!(tree.get_nth_mut(5), Some(&mut SfNode::Number(6)));
        assert_eq!(tree.get_nth_mut(6), None);
    }

    #[test]
    fn test_sftree_make_nth_parent_zero() {
        let initial_tree_line = "[[1,[2,3]],[[4,5],6]]";
        // this test assumes that SfTree::from_line() is working correctly
        [
            (-1, "[[1,[2,3]],[[4,5],6]]"),
            (0, "[0,[[4,5],6]]"),
            (1, "[[1,0],[[4,5],6]]"),
            (2, "[[1,0],[[4,5],6]]"),
            (3, "[[1,[2,3]],[0,6]]"),
            (4, "[[1,[2,3]],[0,6]]"),
            (5, "[[1,[2,3]],0]"),
            (6, "[[1,[2,3]],[[4,5],6]]"),
        ]
        .into_iter()
        .for_each(|(input_nth, output_tree_line)| {
            let mut initial_tree = SfTree::from_line(initial_tree_line);
            let output_tree = SfTree::from_line(output_tree_line);

            assert_eq!(
                initial_tree.make_nth_parent_zero(input_nth),
                if initial_tree_line == output_tree_line {
                    SfTreeReduced::No
                } else {
                    SfTreeReduced::Yes
                }
            );
            assert_eq!(initial_tree, output_tree);
        });
    }

    #[test]
    fn test_sftree_explode_once() {
        // this test assumes that SfTree::from_line() is working correctly
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
            let mut input = SfTree::from_line(input_line);
            let expected_output = SfTree::from_line(expected_output_line);

            let result = input.explode_once();

            assert!(
                matches!(result, SfTreeReduced::Yes),
                "{} did not explode",
                input_line
            );
            assert_eq!(input, expected_output, "{} exploded wrongly", input_line);
        });
    }

    #[test]
    fn test_sftree_split_once() {
        // this test assumes that SfTree::from_line() is working correctly
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
            let mut input = SfTree::from_line(input_line);
            let expected_output = SfTree::from_line(expected_output_line);

            let result = input.split_once();

            assert!(
                matches!(result, SfTreeReduced::Yes),
                "{} did not split",
                input_line
            );
            assert_eq!(input, expected_output, "{} split wrongly", input_line);
        });
    }

    #[test]
    fn test_sftree_get_magnitude() {
        // this test assumes that SfTree::from_line() is working correctly
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
                SfTree::from_line(input_line).get_magnitude(),
                expected_mag,
                "{} magnitude is wrong",
                input_line
            );
        })
    }

    #[test]
    fn test_sftree_add_no_reduce_needed() {
        // this test assumes that SfTree::from_line() is working correctly
        assert_eq!(
            SfTree::from_line("[1,2]").add(SfTree::from_line("[[3,4],5]")),
            SfTree::from_line("[[1,2],[[3,4],5]]")
        );
    }

    #[test]
    fn test_sftree_add_with_reduce() {
        // this test assumes that SfTree::from_line() is working correctly
        assert_eq!(
            SfTree::from_line("[[[[4,3],4],4],[7,[[8,4],9]]]").add(SfTree::from_line("[1,1]")),
            SfTree::from_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_sftree_add_multiple() {
        // this test assumes that SfTree::from_line() is working correctly
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
            let expected_output = SfTree::from_line(expected_output_line);

            let result = inputs
                .trim()
                .lines()
                .map(SfTree::from_line)
                .reduce(|acc, sf| acc.add(sf))
                .unwrap();

            assert_eq!(
                result, expected_output,
                "did not get {}",
                expected_output_line
            );
        });
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
