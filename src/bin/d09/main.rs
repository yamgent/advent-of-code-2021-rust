const ACTUAL_INPUT: &str = include_str!("input.txt");

fn read_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char as i32 - '0' as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_low_point_coords(grid: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    (0..grid.len() as i32).for_each(|r| {
        (0..grid[0].len() as i32).for_each(|c| {
            let adjacents_higher = vec![(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                .into_iter()
                .filter(|coord| {
                    coord.0 >= 0
                        && coord.0 < grid.len() as i32
                        && coord.1 >= 0
                        && coord.1 < grid[0].len() as i32
                })
                .all(|coord| {
                    grid[coord.0 as usize][coord.1 as usize] > grid[r as usize][c as usize]
                });

            if adjacents_higher {
                result.push((r as usize, c as usize));
            }
        });
    });

    result
}

fn p1(input: &str) -> String {
    let grid = read_input(input);

    get_low_point_coords(&grid)
        .into_iter()
        .map(|coord| grid[coord.0][coord.1])
        .map(|x| x + 1)
        .sum::<i32>()
        .to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
