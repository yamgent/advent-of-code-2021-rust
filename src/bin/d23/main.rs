use std::collections::{BinaryHeap, HashMap};

const ACTUAL_INPUT: &str = include_str!("input.txt");

fn manhatten(start: (i32, i32), dest: (i32, i32)) -> i32 {
    (dest.0 - start.0).abs() + (dest.1 - start.1).abs()
}

fn amp_move_cost(amp_type: &char) -> i32 {
    match *amp_type {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Illegal amp type {}", amp_type),
    }
}

fn amp_home_x(amp_type: &char) -> i32 {
    match *amp_type {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!("Unknown amp_type {}", amp_type),
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Amp(char),
}

impl Cell {
    fn get_cell(ch: char) -> Option<Cell> {
        match ch {
            '.' => Some(Cell::Empty),
            'A' => Some(Cell::Amp('A')),
            'B' => Some(Cell::Amp('B')),
            'C' => Some(Cell::Amp('C')),
            'D' => Some(Cell::Amp('D')),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct World {
    cells: HashMap<(i32, i32), Cell>,
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl World {
    fn from_input(input: &str) -> World {
        World {
            cells: HashMap::from_iter(input.trim().lines().enumerate().flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, ch)| {
                    Cell::get_cell(ch).map(|cell| ((x as i32, y as i32), cell))
                })
            })),
        }
    }

    fn amp_is_at_correct_home(&self, amp_pos: &(i32, i32), amp_type: &char) -> bool {
        let x = amp_home_x(amp_type);

        self.cells
            .iter()
            .filter(|(pos, _)| pos.0 == x && pos.1 > amp_pos.1)
            .all(|(_, cell)| matches!(cell, Cell::Empty))
    }

    fn find_amp_home_pos_from_hallway(&self, amp_type: &char) -> Option<(i32, i32)> {
        let x = amp_home_x(amp_type);

        let mut room = self
            .cells
            .iter()
            .filter(
                |(pos, _)| pos.0 == x && pos.1 != 1, // filter hallway (y = 1)
            )
            .collect::<Vec<_>>();

        if room.iter().any(|(_, cell)| match cell {
            Cell::Empty => false,
            Cell::Amp(another_amp) => another_amp != amp_type,
        }) {
            return None;
        }

        room.sort_unstable_by_key(|(pos, _)| pos.1);

        room.into_iter()
            .rev()
            .find(|(_, cell)| matches!(cell, Cell::Empty))
            .map(|(pos, _)| (x, pos.1))
    }

    fn is_empty(&self, pos: &(i32, i32)) -> bool {
        matches!(self.cells.get(pos), Some(Cell::Empty))
    }

    fn has_no_obstruction(&self, start: (i32, i32), end: (i32, i32)) -> bool {
        let dir_norm = (
            if end.0 == start.0 {
                0
            } else {
                end.0 - start.0 / (end.0 - start.0)
            },
            if end.1 == start.1 {
                0
            } else {
                end.1 - start.1 / (end.1 - start.1)
            },
        );

        if start.0 == end.0 {
            ((start.1 + dir_norm.1)..=(end.1))
                .map(|y| (end.0, y))
                .all(|pos| self.is_empty(&pos))
        } else if start.1 == end.1 {
            ((start.0 + dir_norm.0)..=(end.0))
                .map(|x| (x, end.1))
                .all(|pos| self.is_empty(&pos))
        } else {
            // try going x first
            ((start.0 + dir_norm.0)..=(end.0))
            .map(|x| (x, start.1))
            .chain(((start.1)..=(end.1)).map(|y| (end.0, y)))
            .all(|pos| self.is_empty(&pos))
        ||
        // if doesn't work with x, try y first
        ((start.1 + dir_norm.1)..=(end.1))
            .map(|y| (start.0, y))
            .chain(((start.0)..=(end.0)).map(|x| (x, end.1)))
            .all(|pos| self.is_empty(&pos))
        }
    }

    fn swap(&self, pos1: &(i32, i32), pos2: &(i32, i32)) -> World {
        let mut cells = self.cells.clone();

        let cell1 = *cells.get(pos1).unwrap();
        let cell2 = *cells.get(pos2).unwrap();
        cells.insert(*pos1, cell2);
        cells.insert(*pos2, cell1);

        World { cells }
    }

    fn get_next_worlds(&self) -> Vec<(World, usize)> {
        self.cells
            .iter()
            .flat_map(|(pos, cell)| match cell {
                Cell::Empty => vec![],
                Cell::Amp(amp_type) => {
                    let is_hallway = pos.1 == 1;

                    if is_hallway {
                        if let Some(end_pos) = self.find_amp_home_pos_from_hallway(amp_type) {
                            if self.has_no_obstruction(*pos, end_pos) {
                                let cost =
                                    (manhatten(*pos, end_pos) * amp_move_cost(amp_type)) as usize;
                                return vec![(self.swap(pos, &end_pos), cost)];
                            }
                        }
                        vec![]
                    } else if self.amp_is_at_correct_home(pos, amp_type) {
                        vec![]
                    } else {
                        self.cells
                            .iter()
                            .map(|(end_pos, _)| end_pos)
                            .filter(|end_pos| end_pos.1 == 1 && end_pos.0 != pos.0)
                            .filter(|end_pos| self.has_no_obstruction(*pos, **end_pos))
                            .map(|end_pos| {
                                let cost =
                                    (manhatten(*pos, *end_pos) * amp_move_cost(amp_type)) as usize;
                                (self.swap(pos, end_pos), cost)
                            })
                            .collect()
                    }
                }
            })
            .collect()
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xs = self
            .cells
            .iter()
            .map(|(coord, _)| coord.0)
            .collect::<Vec<_>>();

        let ys = self
            .cells
            .iter()
            .map(|(coord, _)| coord.1)
            .collect::<Vec<_>>();

        let min = (*xs.iter().min().unwrap(), *ys.iter().min().unwrap());
        let max = (xs.into_iter().max().unwrap(), ys.into_iter().max().unwrap());

        for y in (min.1)..=(max.1) {
            for x in (min.0)..=(max.0) {
                write!(
                    f,
                    "{}",
                    match self.cells.get(&(x, y)) {
                        None => ' ',
                        Some(cell) => match cell {
                            Cell::Empty => '.',
                            Cell::Amp(amp_type) => *amp_type,
                        },
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    world: World,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // flip the cost to make heap a minheap
        other.cost.cmp(&self.cost).then_with(|| {
            // needed for `PartialEq` to be consistent
            // with `Ord`
            self.world.to_string().cmp(&other.world.to_string())
        })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: World) -> Option<usize> {
    let goal = World::from_input(
        r"
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
",
    );
    let mut dist: HashMap<String, usize> = HashMap::from_iter([(start.to_string(), 0)]);
    let mut heap = BinaryHeap::from_iter([(State {
        cost: 0,
        world: start,
    })]);

    while let Some(State { cost, world }) = heap.pop() {
        if world == goal {
            return Some(cost);
        }

        if cost > *dist.get(&world.to_string()).unwrap_or(&0) {
            continue;
        }

        for (next_world, delta_cost) in world.get_next_worlds() {
            let next_state = State {
                cost: cost + delta_cost,
                world: next_world,
            };

            if next_state.cost
                < *dist
                    .get(&next_state.world.to_string())
                    .unwrap_or(&usize::MAX)
            {
                dist.insert(next_state.world.to_string(), next_state.cost);
                heap.push(next_state);
            }
        }
    }

    None
}

fn p1(input: &str) -> String {
    dijkstra(World::from_input(input)).unwrap().to_string()
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
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "12521");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "");
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
