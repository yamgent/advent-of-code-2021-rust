use std::{collections::VecDeque, iter};

const ACTUAL_INPUT: &str = include_str!("input.txt");

struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        Self {
            grid: input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|char| char as i32 - '0' as i32)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }

    fn get_neighbours(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let (r, c) = (coord.0 as i32, coord.1 as i32);
        [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
            .into_iter()
            .filter(|coord| {
                coord.0 >= 0
                    && coord.0 < self.grid.len() as i32
                    && coord.1 >= 0
                    && coord.1 < self.grid[0].len() as i32
            })
            .map(|(r, c)| (r as usize, c as usize))
            .collect::<Vec<_>>()
    }

    fn get_low_point_coords(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];

        (0..self.grid.len()).for_each(|r| {
            (0..self.grid[0].len()).for_each(|c| {
                let adjacents_higher = self.get_neighbours((r, c)).into_iter().all(|coord| {
                    self.grid[coord.0 as usize][coord.1 as usize]
                        > self.grid[r as usize][c as usize]
                });

                if adjacents_higher {
                    result.push((r as usize, c as usize));
                }
            });
        });

        result
    }

    fn get(&self, coord: &(usize, usize)) -> i32 {
        self.grid[coord.0][coord.1]
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }
}

fn p1(input: &str) -> String {
    let grid = Grid::from_input(input);

    grid.get_low_point_coords()
        .into_iter()
        .map(|coord| grid.get(&coord))
        .map(|x| x + 1)
        .sum::<i32>()
        .to_string()
}

fn p2(input: &str) -> String {
    let grid = Grid::from_input(input);

    let unassigned = 99999;

    let mut basin_assignment = iter::repeat(
        iter::repeat(unassigned)
            .take(grid.width())
            .collect::<Vec<_>>(),
    )
    .take(grid.height())
    .collect::<Vec<_>>();
    let low_point_coords = grid.get_low_point_coords();
    let total_low_points = low_point_coords.len();

    low_point_coords
        .into_iter()
        .enumerate()
        .for_each(|(index, coord)| {
            let mut to_visit = VecDeque::new();

            basin_assignment[coord.0][coord.1] = index;
            to_visit.push_back(coord);

            loop {
                match to_visit.pop_front() {
                    None => break,
                    Some(coord) => {
                        #[allow(clippy::needless_collect)]
                        // reason = "Collect is actually needed to avoid borrow checker errors"
                        let unvisited_valid_neighbours = grid
                            .get_neighbours(coord)
                            .into_iter()
                            .filter(|neighbour_coord| {
                                basin_assignment[neighbour_coord.0][neighbour_coord.1] == unassigned
                            })
                            .filter(|neighbour_coord| {
                                grid.get(neighbour_coord) != 9
                                    && grid.get(neighbour_coord) > grid.get(&coord)
                            })
                            .collect::<Vec<_>>();

                        unvisited_valid_neighbours
                            .into_iter()
                            .for_each(|neighbour_coord| {
                                basin_assignment[neighbour_coord.0][neighbour_coord.1] = index;
                                to_visit.push_back(neighbour_coord)
                            });
                    }
                }
            }
        });

    let mut basin_sizes = (0..total_low_points)
        .map(|index| {
            basin_assignment
                .iter()
                .map(|row| row.iter().filter(|&&cell| cell == index).count())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    basin_sizes.sort_unstable();

    basin_sizes
        .into_iter()
        .rev()
        .take(3)
        .product::<usize>()
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
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "15");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "504");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "1134");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1558722");
    }
}
