use std::{collections::BinaryHeap, iter};

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq)]
struct DijkstraCost {
    position: (usize, usize),
    cost: i32,
}

impl PartialOrd for DijkstraCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // compare in reverse, since the binary heap
        // is a max heap, not a min heap
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for DijkstraCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare in reverse, since the binary heap
        // is a max heap, not a min heap
        other.cost.cmp(&self.cost)
    }
}

fn get_neighbours(current: (usize, usize), grid: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let r = current.0 as i32;
    let c = current.1 as i32;

    [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
        .into_iter()
        .filter(|coord| {
            coord.0 >= 0
                && coord.1 >= 0
                && coord.0 < grid.len() as i32
                && coord.1 < grid[0].len() as i32
        })
        .map(|coord| (coord.0 as usize, coord.1 as usize))
        .collect::<Vec<_>>()
}

fn p1(input: &str) -> String {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // use dijkstra to find shortest
    let target = (grid.len() - 1, grid[0].len() - 1);
    let mut heap = BinaryHeap::new();

    let infinity = 999999;

    let mut dist = iter::repeat(
        iter::repeat(infinity)
            .take(grid[0].len())
            .collect::<Vec<_>>(),
    )
    .take(grid.len())
    .collect::<Vec<_>>();
    let mut prev = iter::repeat(iter::repeat(None).take(grid[0].len()).collect::<Vec<_>>())
        .take(grid.len())
        .collect::<Vec<_>>();

    dist[0][0] = 0;
    heap.push(DijkstraCost {
        position: (0, 0),
        cost: 0,
    });

    while !heap.is_empty() {
        let info = heap.pop().unwrap();

        if dist[info.position.0][info.position.1] < info.cost {
            // already found a better one
            continue;
        }

        if info.position == target {
            // found our answer
            break;
        }

        get_neighbours(info.position, &grid)
            .into_iter()
            .for_each(|coord| {
                let alt = dist[info.position.0][info.position.1] + grid[coord.0][coord.1];

                if alt < dist[coord.0][coord.1] {
                    dist[coord.0][coord.1] = alt;
                    prev[coord.0][coord.1] = Some(info.position);
                    heap.push(DijkstraCost {
                        position: coord,
                        cost: alt,
                    });
                }
            });
    }

    let mut current_pos = target;
    let mut shortest_cost = 0;
    while current_pos != (0, 0) {
        shortest_cost += grid[current_pos.0][current_pos.1];
        current_pos = prev[current_pos.0][current_pos.1].unwrap();
    }

    shortest_cost.to_string()
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
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "40");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "685");
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
