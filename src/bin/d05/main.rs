use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn parse_from_line(line: &str) -> Self {
        let (start, end) = line.split_once(" -> ").unwrap();

        fn parse_coord_from_part(part: &str) -> (i32, i32) {
            let mut components = part.split(',');
            let x = components.next().unwrap().parse().unwrap();
            let y = components.next().unwrap().parse().unwrap();
            (x, y)
        }

        let start = parse_coord_from_part(start);
        let end = parse_coord_from_part(end);

        Self { start, end }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn iter(&self) -> LineIterator {
        fn calculate_normalized_direction(start: i32, end: i32) -> i32 {
            if start == end {
                0
            } else {
                let delta = end - start;
                delta / delta.abs()
            }
        }

        let direction = (
            calculate_normalized_direction(self.start.0, self.end.0),
            calculate_normalized_direction(self.start.1, self.end.1),
        );

        LineIterator {
            current: (self.start.0 - direction.0, self.start.1 - direction.1),
            end: self.end,
            direction,
        }
    }
}

struct LineIterator {
    current: (i32, i32),
    end: (i32, i32),
    direction: (i32, i32),
}

impl Iterator for LineIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current != self.end {
            self.current.0 += self.direction.0;
            self.current.1 += self.direction.1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn solve(input: &str, include_diagonals: bool) -> String {
    input
        .trim()
        .lines()
        .map(Line::parse_from_line)
        .filter(|line| include_diagonals || line.is_horizontal_or_vertical())
        .fold(&mut HashMap::new(), |visited, current_line| {
            current_line.iter().for_each(|coord| {
                *visited.entry(coord).or_insert(0) += 1;
            });
            visited
        })
        .values()
        .into_iter()
        .filter(|x| **x > 1)
        .count()
        .to_string()
}

fn p1(input: &str) -> String {
    solve(input, false)
}

fn p2(input: &str) -> String {
    solve(input, true)
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "5");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "5442");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "12");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "19571");
    }
}
