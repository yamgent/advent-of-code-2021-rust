use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Bounds {
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

impl Bounds {
    fn is_outside_bounds(&self, coord: &(i32, i32)) -> bool {
        coord.0 < self.top_left.0
            || coord.1 < self.top_left.1
            || coord.0 > self.bottom_right.0
            || coord.1 > self.bottom_right.1
    }
}

fn get_kernel(coord: &(i32, i32)) -> Vec<(i32, i32)> {
    ((coord.1 - 1)..=(coord.1 + 1))
        .flat_map(|y| {
            ((coord.0 - 1)..=(coord.0 + 1))
                .map(|x| (x, y))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_bounds(list: &HashSet<(i32, i32)>) -> Bounds {
    let min_x = list.iter().map(|c| c.0).min().unwrap();
    let min_y = list.iter().map(|c| c.1).min().unwrap();

    let max_x = list.iter().map(|c| c.0).max().unwrap();
    let max_y = list.iter().map(|c| c.1).max().unwrap();

    Bounds {
        top_left: (min_x, min_y),
        bottom_right: (max_x, max_y),
    }
}

fn enhance(
    image: &HashSet<(i32, i32)>,
    algorithm: &HashSet<i32>,
    iteration: usize,
) -> HashSet<(i32, i32)> {
    let mut new_image = HashSet::new();
    let bounds = get_bounds(image);

    ((bounds.top_left.0 - 1)..=(bounds.bottom_right.0 + 1)).for_each(|x| {
        ((bounds.top_left.1 - 1)..=(bounds.bottom_right.1 + 1)).for_each(|y| {
            let id = u32::from_str_radix(
                &get_kernel(&(x, y))
                    .into_iter()
                    .map(|coord| {
                        if bounds.is_outside_bounds(&coord) {
                            if algorithm.contains(&0) && iteration % 2 == 1 {
                                "1"
                            } else {
                                "0"
                            }
                        } else if image.contains(&coord) {
                            "1"
                        } else {
                            "0"
                        }
                    })
                    .collect::<String>(),
                2,
            )
            .unwrap() as i32;

            if algorithm.contains(&id) {
                new_image.insert((x, y));
            }
        });
    });

    new_image
}

#[allow(dead_code)] // reason = "is debug code"
fn print_image(image: &HashSet<(i32, i32)>) {
    let bounds = get_bounds(image);

    (bounds.top_left.1..=bounds.bottom_right.1).for_each(|y| {
        (bounds.top_left.0..=bounds.bottom_right.0).for_each(|x| {
            print!("{}", if image.contains(&(x, y)) { '#' } else { '.' });
        });
        println!();
    });
}

fn solve(input: &str, total_iterations: usize) -> String {
    let (algorithm, image) = input.trim().split_once("\n\n").unwrap();

    let algorithm: HashSet<i32> = HashSet::from_iter(
        algorithm
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i as i32),
    );

    let mut image: HashSet<(i32, i32)> =
        HashSet::from_iter(image.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| (i as i32, y as i32))
                .collect::<Vec<_>>()
        }));

    (0..total_iterations).for_each(|i| {
        image = enhance(&image, &algorithm, i);
    });

    image.len().to_string()
}

fn p1(input: &str) -> String {
    solve(input, 2)
}

fn p2(input: &str) -> String {
    solve(input, 50)
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_kernel() {
        assert_eq!(
            get_kernel(&(1, 1)),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2),
            ]
        )
    }

    #[test]
    fn test_get_bounds() {
        assert_eq!(
            get_bounds(&HashSet::from_iter([(-1, -6), (-2, -4), (9, 10), (18, 5)])),
            Bounds {
                top_left: (-2, -6),
                bottom_right: (18, 10)
            }
        );
    }

    const SAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "35");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4917");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "3351");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "16389");
    }
}
