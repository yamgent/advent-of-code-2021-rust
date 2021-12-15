use std::collections::{BinaryHeap, HashMap};

const ACTUAL_INPUT: &str = include_str!("input.txt");

trait Grid {
    fn get(&self, pos: (usize, usize)) -> i32;
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn get_neighbours(&self, current: (usize, usize)) -> Vec<(usize, usize)> {
        let (r, c) = (current.0 as i32, current.1 as i32);

        [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
            .into_iter()
            .filter(|pos| {
                pos.0 >= 0
                    && pos.1 >= 0
                    && pos.0 < self.height() as i32
                    && pos.1 < self.width() as i32
            })
            .map(|pos| (pos.0 as usize, pos.1 as usize))
            .collect::<Vec<_>>()
    }
}

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

fn dijkstra_shortest(grid: &impl Grid) -> String {
    let target = (grid.height() - 1, grid.width() - 1);

    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    let mut heap = BinaryHeap::new();

    dist.insert((0, 0), 0);
    heap.push(DijkstraCost {
        position: (0, 0),
        cost: 0,
    });

    while !heap.is_empty() {
        let selected = heap.pop().unwrap();
        let selected_dist = *dist.get(&selected.position).unwrap();

        if selected_dist < selected.cost {
            // found a better path already
            continue;
        }

        if selected.position == target {
            // we can halt the loop, a path is already found
            break;
        }

        grid.get_neighbours(selected.position)
            .into_iter()
            .for_each(|neighbour_coord| {
                let alt = selected_dist + grid.get(neighbour_coord);

                let is_better = match dist.get(&neighbour_coord) {
                    None => true,
                    Some(&cost) => alt < cost,
                };

                if is_better {
                    dist.insert(neighbour_coord, alt);
                    prev.insert(neighbour_coord, selected.position);
                    heap.push(DijkstraCost {
                        position: neighbour_coord,
                        cost: alt,
                    });
                }
            });
    }

    let mut current_pos = target;
    let mut shortest_cost = 0;
    while current_pos != (0, 0) {
        shortest_cost += grid.get(current_pos);
        current_pos = *prev.get(&current_pos).unwrap();
    }

    shortest_cost.to_string()
}

struct P1Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid for P1Grid {
    fn get(&self, pos: (usize, usize)) -> i32 {
        self.grid[pos.0][pos.1]
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn p1(input: &str) -> String {
    dijkstra_shortest(&P1Grid {
        grid: parse_input(input),
    })
}

struct P2Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid for P2Grid {
    fn get(&self, pos: (usize, usize)) -> i32 {
        let actual_r = pos.0 % self.grid.len();
        let actual_c = pos.1 % self.grid[0].len();
        let actual_value = self.grid[actual_r][actual_c];

        let increment_r = (pos.0 / self.grid.len()) as i32;
        let increment_c = (pos.1 / self.grid[0].len()) as i32;

        ((actual_value - 1 + increment_r + increment_c) % 9) + 1
    }

    fn width(&self) -> usize {
        self.grid[0].len() * 5
    }

    fn height(&self) -> usize {
        self.grid.len() * 5
    }
}

fn p2(input: &str) -> String {
    dijkstra_shortest(&P2Grid {
        grid: parse_input(input),
    })
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
        assert_eq!(p2(SAMPLE_INPUT), "315");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "2995");
    }
}
