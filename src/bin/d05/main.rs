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
            let mut parts = part.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
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
        let direction = {
            let x_delta = self.end.0 - self.start.0;
            let y_delta = self.end.1 - self.start.1;
            (
                if x_delta == 0 {
                    0
                } else {
                    x_delta / x_delta.abs()
                },
                if y_delta == 0 {
                    0
                } else {
                    y_delta / y_delta.abs()
                },
            )
        };

        LineIterator {
            start_produced: false,
            current: self.start,
            end: self.end,
            direction,
        }
    }
}

struct LineIterator {
    start_produced: bool,
    current: (i32, i32),
    end: (i32, i32),
    direction: (i32, i32),
}

impl Iterator for LineIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.start_produced {
            self.start_produced = true;
            Some(self.current)
        } else if self.current != self.end {
            self.current.0 += self.direction.0;
            self.current.1 += self.direction.1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn p1(input: &str) -> String {
    let mut visited = HashMap::new();

    input
        .trim()
        .lines()
        .map(Line::parse_from_line)
        .filter(Line::is_horizontal_or_vertical)
        .for_each(|line| {
            line.iter().for_each(|coord| {
                *visited.entry(coord).or_insert(0) += 1;
            })
        });

    visited
        .values()
        .into_iter()
        .filter(|x| **x > 1)
        .count()
        .to_string()
}

fn p2(input: &str) -> String {
    let mut visited = HashMap::new();

    input
        .trim()
        .lines()
        .map(Line::parse_from_line)
        .for_each(|line| {
            line.iter().for_each(|coord| {
                *visited.entry(coord).or_insert(0) += 1;
            })
        });

    visited
        .values()
        .into_iter()
        .filter(|x| **x > 1)
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
