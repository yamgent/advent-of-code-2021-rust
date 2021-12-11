use std::collections::VecDeque;

const ACTUAL_INPUT: &str = include_str!("input.txt");

struct Cavern {
    grid: Vec<Vec<i32>>,
}

impl Cavern {
    fn from_input(input: &str) -> Self {
        Self {
            grid: input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c as i32 - '0' as i32)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }

    fn get_height(&self) -> usize {
        self.grid.len()
    }

    fn get_width(&self) -> usize {
        self.grid[0].len()
    }

    fn get_neighbour_coords(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let coord = (coord.0 as i64, coord.1 as i64);
        let height = self.get_height() as i64;
        let width = self.get_width() as i64;

        ((coord.0 - 1)..=(coord.0 + 1))
            .flat_map(|r| {
                ((coord.1 - 1)..=(coord.1 + 1))
                    .map(|c| (r, c))
                    .collect::<Vec<_>>()
            })
            .filter(|(r, c)| {
                !(*r == coord.0 && *c == coord.1) && *r >= 0 && *r < height && *c >= 0 && *c < width
            })
            .map(|(r, c)| (r as usize, c as usize))
            .collect::<Vec<_>>()
    }

    fn step(&mut self) -> i32 {
        let mut counter = 0;
        let mut to_visit_for_flash = VecDeque::new();

        self.grid.iter_mut().enumerate().for_each(|(r, row)| {
            row.iter_mut().enumerate().for_each(|(c, cell)| {
                *cell += 1;

                if *cell > 9 {
                    to_visit_for_flash.push_back((r, c));
                }
            });
        });

        while let Some(coord) = to_visit_for_flash.pop_front() {
            if self.grid[coord.0][coord.1] == 0 {
                // already flashed once
                continue;
            }

            counter += 1;
            self.grid[coord.0][coord.1] = 0;

            self.get_neighbour_coords(coord).into_iter().for_each(|c| {
                if self.grid[c.0][c.1] != 0 {
                    self.grid[c.0][c.1] += 1;

                    if self.grid[c.0][c.1] > 9 {
                        to_visit_for_flash.push_back(c);
                    }
                }
            });
        }

        counter
    }
}

fn p1(input: &str) -> String {
    let mut counter = 0;
    let mut cavern = Cavern::from_input(input);

    (0..100).for_each(|_| counter += cavern.step());

    counter.to_string()
}

fn p2(input: &str) -> String {
    let mut cavern = Cavern::from_input(input);

    let mut step = 1;
    let total = (cavern.get_height() * cavern.get_width()) as i32;

    while cavern.step() != total {
        step += 1;
    }

    step.to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_neighbour_coords() {
        let cavern = Cavern::from_input("000\n000\n000\n");

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((0, 0))),
            [(0, 1), (1, 0), (1, 1)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((0, 1))),
            [(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((0, 2))),
            [(0, 1), (1, 1), (1, 2)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((1, 0))),
            [(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((1, 1))),
            [
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
            .into_iter()
            .collect::<HashSet<(usize, usize)>>()
        );

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((1, 2))),
            [(0, 1), (0, 2), (1, 1), (2, 1), (2, 2)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((2, 0))),
            [(1, 0), (1, 1), (2, 1)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((2, 1))),
            [(1, 0), (1, 1), (1, 2), (2, 0), (2, 2)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );

        assert_eq!(
            HashSet::from_iter(cavern.get_neighbour_coords((2, 2))),
            [(1, 1), (1, 2), (2, 1)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );
    }

    const SAMPLE_INPUT: &str = r"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "1656");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1755");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "195");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "212");
    }
}
