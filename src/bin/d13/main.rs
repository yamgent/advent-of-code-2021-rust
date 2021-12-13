use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("input.txt");

fn get_points_display(points: &HashSet<(i32, i32)>) -> String {
    if points.is_empty() {
        return "Empty".to_owned();
    }

    // String native type can be used as a buffer efficiently
    let mut string_buffer = String::new();
    let max_x = points.iter().map(|point| point.0).max().unwrap();
    let max_y = points.iter().map(|point| point.1).max().unwrap();

    string_buffer.push('\n'); // for easier testing, add \n at the start
    for y in 0..=max_y {
        for x in 0..=max_x {
            string_buffer.push(if points.contains(&(x, y)) { '#' } else { '.' });
        }
        string_buffer.push('\n');
    }
    string_buffer
}

fn solve(input: &str) -> (usize, String) {
    let mut points = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("fold"))
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<HashSet<_>>();

    let mut first_fold_dots_count = None;

    input
        .trim()
        .lines()
        .filter(|line| line.starts_with("fold"))
        .for_each(|line| {
            let line = line.strip_prefix("fold along ").unwrap();
            let (direction, position) = line.split_once('=').unwrap();
            let position = position.parse::<i32>().unwrap();

            #[allow(clippy::needless_collect)] // reason = "collect is actually needed"
            let points_to_remove = points
                .iter()
                .filter(|point| match direction {
                    "y" => point.1 > position,
                    "x" => point.0 > position,
                    _ => panic!("Unknown direction {}", direction),
                })
                .copied()
                .collect::<Vec<_>>();

            points_to_remove.into_iter().for_each(|point| {
                let new_point = match direction {
                    "y" => (point.0, 2 * position - point.1),
                    "x" => (2 * position - point.0, point.1),
                    _ => panic!("Unknown direction {}", direction),
                };

                points.remove(&point);
                points.insert(new_point);
            });

            if first_fold_dots_count.is_none() {
                first_fold_dots_count = Some(points.len());
            }
        });

    (first_fold_dots_count.unwrap(), get_points_display(&points))
}

fn p1(input: &str) -> String {
    solve(input).0.to_string()
}

fn p2(input: &str) -> String {
    solve(input).1
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "17");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "770");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(
            p2(SAMPLE_INPUT),
            r"
#####
#...#
#...#
#...#
#####
"
        );
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(
            p2(ACTUAL_INPUT),
            r"
####.###..#..#.####.#....###..###..###.
#....#..#.#..#.#....#....#..#.#..#.#..#
###..#..#.#..#.###..#....#..#.###..#..#
#....###..#..#.#....#....###..#..#.###.
#....#....#..#.#....#....#....#..#.#.#.
####.#.....##..####.####.#....###..#..#
"
        );
    }
}
