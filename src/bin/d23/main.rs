use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    ops::RangeInclusive,
};

const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1_get_end_world() -> World {
    World::from_input(
        r"
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
",
    )
}

fn get_move_cost(amp_type: &char, start: &(i32, i32), end: &(i32, i32)) -> usize {
    fn manhatten(start: &(i32, i32), end: &(i32, i32)) -> i32 {
        (end.0 - start.0).abs() + (end.1 - start.1).abs()
    }

    fn get_unit_cost(amp_type: &char) -> i32 {
        match *amp_type {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("Illegal amp type {}", amp_type),
        }
    }

    (manhatten(start, end) * get_unit_cost(amp_type)) as usize
}

fn get_amp_home_x(amp_type: &char) -> i32 {
    match *amp_type {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!("Unknown amp_type {}", amp_type),
    }
}

fn inclusive_range(a: i32, b: i32) -> RangeInclusive<i32> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Amp(char),
}

impl Cell {
    fn get_cell(ch: &char) -> Option<Cell> {
        match ch {
            '.' => Some(Cell::Empty),
            'A' | 'B' | 'C' | 'D' => Some(Cell::Amp(*ch)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct World {
    cells: HashMap<(i32, i32), Cell>,
    string_repr_cache: String,
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.string_repr_cache == other.string_repr_cache
    }
}

impl World {
    fn get_string_repr(cells: &HashMap<(i32, i32), Cell>) -> String {
        let mut result = String::new();

        let max = (
            cells.iter().map(|(coord, _)| coord.0).max().unwrap(),
            cells.iter().map(|(coord, _)| coord.1).max().unwrap(),
        );

        for y in 0..=(max.1) {
            for x in 0..=(max.0) {
                result.push(match cells.get(&(x, y)) {
                    None => ' ',
                    Some(cell) => match cell {
                        Cell::Empty => '.',
                        Cell::Amp(amp_type) => *amp_type,
                    },
                });
            }
            result.push('\n');
        }

        result
    }

    fn from_input(input: &str) -> World {
        let cells = HashMap::from_iter(input.trim().lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, ch)| {
                Cell::get_cell(&ch).map(|cell| ((x as i32, y as i32), cell))
            })
        }));
        let string_repr = World::get_string_repr(&cells);

        World {
            cells,
            string_repr_cache: string_repr,
        }
    }

    fn new(cells: HashMap<(i32, i32), Cell>) -> World {
        let string_repr = World::get_string_repr(&cells);
        World {
            cells,
            string_repr_cache: string_repr,
        }
    }

    fn is_amp_at_correct_home(&self, amp_pos: &(i32, i32), amp_type: &char) -> bool {
        let x = get_amp_home_x(amp_type);

        if amp_pos.0 != x {
            false
        } else {
            self.cells
                .iter()
                .filter(|(pos, _)| pos.0 == x && pos.1 > amp_pos.1)
                .all(|(_, cell)| match cell {
                    Cell::Amp(amp) => amp == amp_type,
                    _ => false,
                })
        }
    }

    fn find_amp_home_pos_from_hallway(&self, amp_type: &char) -> Option<(i32, i32)> {
        let x = get_amp_home_x(amp_type);

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

        room.iter()
            .rev()
            .find(|(_, cell)| matches!(cell, Cell::Empty))
            .map(|(pos, _)| (x, pos.1))
            .map(|empty_pos| {
                if room
                    .iter()
                    .filter(|(pos, _)| pos.1 < empty_pos.1)
                    .all(|(_, cell)| matches!(cell, Cell::Empty))
                {
                    Some(empty_pos)
                } else {
                    None
                }
            })
            .flatten()
    }

    fn is_empty(&self, pos: &(i32, i32)) -> bool {
        matches!(self.cells.get(pos), Some(Cell::Empty))
    }

    fn has_no_obstruction(&self, start: &(i32, i32), end: &(i32, i32)) -> bool {
        let dir_norm = (
            match end.0.cmp(&start.0) {
                Ordering::Equal => 0,
                Ordering::Greater => 1,
                Ordering::Less => -1,
            },
            match end.1.cmp(&start.1) {
                Ordering::Equal => 0,
                Ordering::Greater => 1,
                Ordering::Less => -1,
            },
        );

        if start.0 == end.0 {
            inclusive_range(start.1 + dir_norm.1, end.1)
                .map(|y| (end.0, y))
                .all(|pos| self.is_empty(&pos))
        } else if start.1 == end.1 {
            inclusive_range(start.0 + dir_norm.0, end.0)
                .map(|x| (x, end.1))
                .all(|pos| self.is_empty(&pos))
        } else {
            // try going x first
            inclusive_range(start.0 + dir_norm.0, end.0)
            .map(|x| (x, start.1))
            .chain((inclusive_range(start.1, end.1)).map(|y| (end.0, y)))
            .all(|pos| self.is_empty(&pos))
        ||
        // if doesn't work with x, try y first
        inclusive_range(start.1 + dir_norm.1, end.1)
            .map(|y| (start.0, y))
            .chain(inclusive_range(start.0, end.0).map(|x| (x, end.1)))
            .all(|pos| self.is_empty(&pos))
        }
    }

    fn swap(&self, pos1: &(i32, i32), pos2: &(i32, i32)) -> World {
        let mut cells = self.cells.clone();

        let cell1 = *cells.get(pos1).unwrap();
        let cell2 = *cells.get(pos2).unwrap();
        cells.insert(*pos1, cell2);
        cells.insert(*pos2, cell1);

        World::new(cells)
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
                            if self.has_no_obstruction(pos, &end_pos) {
                                let cost = get_move_cost(amp_type, pos, &end_pos);
                                return vec![(self.swap(pos, &end_pos), cost)];
                            }
                        }
                        vec![]
                    } else if self.is_amp_at_correct_home(pos, amp_type) {
                        vec![]
                    } else {
                        self.cells
                            .iter()
                            .map(|(end_pos, _)| end_pos)
                            .filter(|end_pos| end_pos.1 == 1 && end_pos.0 != pos.0)
                            .filter(|end_pos| self.has_no_obstruction(pos, *end_pos))
                            .map(|end_pos| {
                                let cost = get_move_cost(amp_type, pos, end_pos);
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
        write!(f, "{}", self.string_repr_cache)
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

fn dijkstra(start: World, goal: &World) -> Option<usize> {
    let mut dist: HashMap<String, usize> = HashMap::from_iter([(start.to_string(), 0)]);
    let mut heap = BinaryHeap::from_iter([(State {
        cost: 0,
        world: start,
    })]);

    while let Some(State { cost, world }) = heap.pop() {
        if world == *goal {
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
    dijkstra(World::from_input(input), &p1_get_end_world())
        .unwrap()
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
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn test_get_move_cost() {
        [
            (('A', (0, 0), (5, 0)), 5),
            (('A', (0, 0), (0, 5)), 5),
            (('A', (0, 0), (2, 3)), 5),
            (('B', (0, 0), (5, 0)), 50),
            (('B', (0, 0), (0, 5)), 50),
            (('B', (0, 0), (2, 3)), 50),
            (('C', (0, 0), (5, 0)), 500),
            (('C', (0, 0), (0, 5)), 500),
            (('C', (0, 0), (2, 3)), 500),
            (('D', (0, 0), (5, 0)), 5000),
            (('D', (0, 0), (0, 5)), 5000),
            (('D', (0, 0), (2, 3)), 5000),
        ]
        .into_iter()
        .for_each(|(input, output)| {
            assert_eq!(
                get_move_cost(&input.0, &input.1, &input.2),
                output,
                "Input: {}, {:?}, {:?}",
                input.0,
                input.1,
                input.2
            );

            assert_eq!(
                get_move_cost(&input.0, &input.2, &input.1),
                output,
                "Reverse Input: {}, {:?}, {:?}",
                input.0,
                input.1,
                input.2
            );
        });
    }

    #[test]
    fn test_get_amp_home_x() {
        p1_get_end_world().cells.iter().for_each(|(pos, cell)| {
            if let Cell::Amp(amp_type) = cell {
                assert_eq!(get_amp_home_x(amp_type), pos.0, "Input: {}", amp_type);
            }
        });
    }

    #[test]
    fn test_inclusive_range() {
        assert_eq!(inclusive_range(1, 2), 1..=2);
        assert_eq!(inclusive_range(2, 1), 1..=2);
    }

    #[test]
    fn test_world_eq() {
        let end1 = p1_get_end_world();
        let end2 = p1_get_end_world();
        let sample = World::from_input(SAMPLE_INPUT);

        assert!(end1 == end2);
        assert!(end1 != sample);
        assert!(end2 != sample);
    }

    #[test]
    fn test_get_string_repr() {
        let expected = ["    ", " .AB", "   C", "   D", ""].join("\n");
        let actual_input = HashMap::from_iter([
            ((1, 1), Cell::Empty),
            ((2, 1), Cell::Amp('A')),
            ((3, 1), Cell::Amp('B')),
            ((3, 2), Cell::Amp('C')),
            ((3, 3), Cell::Amp('D')),
        ]);

        assert_eq!(World::get_string_repr(&actual_input), expected);
    }

    #[test]
    fn test_world_from_input() {
        let expected_cells = HashMap::from_iter([
            ((1, 1), Cell::Empty),
            ((2, 1), Cell::Amp('A')),
            ((3, 1), Cell::Amp('B')),
            ((3, 2), Cell::Amp('C')),
            ((3, 3), Cell::Amp('D')),
        ]);
        let actual = World::from_input(
            r"
#####
#.AB#
###C#
  #D#
  ###
",
        );

        assert_eq!(actual.cells, expected_cells);
    }

    #[test]
    fn test_world_is_amp_at_correct_home() {
        // wrong x: false
        assert!(!World::new(HashMap::from_iter([((5, 3), Cell::Amp('A'))]))
            .is_amp_at_correct_home(&(5, 3), &'A'));

        // correct x, at bottom, no matter what is above: true
        assert!(World::new(HashMap::from_iter([((3, 4), Cell::Amp('A'))]))
            .is_amp_at_correct_home(&(3, 4), &'A'));
        assert!(World::new(HashMap::from_iter([
            ((3, 3), Cell::Empty),
            ((3, 4), Cell::Amp('A'))
        ]))
        .is_amp_at_correct_home(&(3, 4), &'A'));
        assert!(World::new(HashMap::from_iter([
            ((3, 3), Cell::Amp('A')),
            ((3, 4), Cell::Amp('A'))
        ]))
        .is_amp_at_correct_home(&(3, 4), &'A'));
        assert!(World::new(HashMap::from_iter([
            ((3, 3), Cell::Amp('B')),
            ((3, 4), Cell::Amp('A'))
        ]))
        .is_amp_at_correct_home(&(3, 4), &'A'));

        // correct x, below is empty or non-A: false
        assert!(!World::new(HashMap::from_iter([
            ((3, 4), Cell::Amp('A')),
            ((3, 5), Cell::Empty)
        ]))
        .is_amp_at_correct_home(&(3, 4), &'A'));
        assert!(!World::new(HashMap::from_iter([
            ((3, 4), Cell::Amp('A')),
            ((3, 5), Cell::Amp('B'))
        ]))
        .is_amp_at_correct_home(&(3, 4), &'A'));
        assert!(!World::new(HashMap::from_iter([
            ((3, 4), Cell::Amp('A')),
            ((3, 5), Cell::Amp('A')),
            ((3, 6), Cell::Amp('D'))
        ]))
        .is_amp_at_correct_home(&(3, 4), &'A'));
        assert!(!World::new(HashMap::from_iter([
            ((3, 4), Cell::Amp('A')),
            ((3, 5), Cell::Amp('A')),
            ((3, 6), Cell::Empty)
        ]))
        .is_amp_at_correct_home(&(3, 4), &'A'));

        // correct x, below is A: true
        assert!(World::new(HashMap::from_iter([
            ((3, 4), Cell::Amp('A')),
            ((3, 5), Cell::Amp('A')),
            ((3, 6), Cell::Amp('A'))
        ]))
        .is_amp_at_correct_home(&(3, 4), &'A'));
    }

    #[test]
    fn test_find_amp_home_pos_from_hallway() {
        // cannot go home if have foreign amp
        assert_eq!(
            World::new(HashMap::from_iter([((3, 3), Cell::Amp('B'))]))
                .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Empty),
                ((3, 4), Cell::Amp('B'))
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Empty),
                ((3, 4), Cell::Amp('B')),
                ((3, 5), Cell::Empty),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Amp('B')),
                ((3, 4), Cell::Empty),
                ((3, 5), Cell::Empty),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Empty),
                ((3, 4), Cell::Empty),
                ((3, 5), Cell::Amp('B')),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Empty),
                ((3, 4), Cell::Amp('B')),
                ((3, 5), Cell::Amp('B')),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Amp('B')),
                ((3, 4), Cell::Amp('B')),
                ((3, 5), Cell::Empty),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );

        // cannot go home if ally is obstructing
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Amp('A')),
                ((3, 4), Cell::Amp('A')),
                ((3, 5), Cell::Empty),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Amp('A')),
                ((3, 4), Cell::Empty),
                ((3, 5), Cell::Amp('A')),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Amp('A')),
                ((3, 4), Cell::Empty),
                ((3, 5), Cell::Empty),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Empty),
                ((3, 4), Cell::Amp('A')),
                ((3, 5), Cell::Empty),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            None
        );

        // can go home if no obstruction
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Empty),
                ((3, 4), Cell::Amp('A')),
                ((3, 5), Cell::Amp('A')),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            Some((3, 3))
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Empty),
                ((3, 4), Cell::Empty),
                ((3, 5), Cell::Amp('A')),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            Some((3, 4))
        );
        assert_eq!(
            World::new(HashMap::from_iter([
                ((3, 3), Cell::Empty),
                ((3, 4), Cell::Empty),
                ((3, 5), Cell::Empty),
            ]))
            .find_amp_home_pos_from_hallway(&'A'),
            Some((3, 5))
        );
        assert_eq!(
            World::new(HashMap::from_iter([((3, 3), Cell::Empty),]))
                .find_amp_home_pos_from_hallway(&'A'),
            Some((3, 3))
        );
    }

    #[test]
    fn test_world_is_empty() {
        let world = World::from_input(
            r"
#####
#.AB#
#####
",
        );

        (0..5).for_each(|x| {
            (0..3).for_each(|y| {
                let coord = (x, y);
                assert_eq!(world.is_empty(&coord), coord == (1, 1), "Coord {:?}", coord);
            });
        })
    }

    #[test]
    fn test_world_has_no_obstruction() {
        let world = World::from_input(
            r"
  #######
  #...B.#
#####A#####
#.# #B# #.#
#B###.###.#
#.AB.A.BA.#
#.###.###B#
#.# #B# #.#
#####A#####
  #.B...#
  #######
",
        );

        // from center A - always have obstruction
        assert!(!world.has_no_obstruction(&(5, 5), &(9, 5)));
        assert!(!world.has_no_obstruction(&(5, 5), &(9, 7)));
        assert!(!world.has_no_obstruction(&(5, 5), &(9, 3)));
        assert!(!world.has_no_obstruction(&(5, 5), &(1, 5)));
        assert!(!world.has_no_obstruction(&(5, 5), &(1, 7)));
        assert!(!world.has_no_obstruction(&(5, 5), &(1, 3)));
        assert!(!world.has_no_obstruction(&(5, 5), &(5, 1)));
        assert!(!world.has_no_obstruction(&(5, 5), &(3, 1)));
        assert!(!world.has_no_obstruction(&(5, 5), &(7, 1)));
        assert!(!world.has_no_obstruction(&(5, 5), &(5, 9)));
        assert!(!world.has_no_obstruction(&(5, 5), &(3, 9)));
        assert!(!world.has_no_obstruction(&(5, 5), &(7, 9)));

        // from tunnel B - straight have A obstruction
        assert!(!world.has_no_obstruction(&(7, 5), &(9, 5)));
        assert!(!world.has_no_obstruction(&(3, 5), &(1, 5)));
        assert!(!world.has_no_obstruction(&(5, 3), &(5, 1)));
        assert!(!world.has_no_obstruction(&(5, 7), &(5, 9)));

        // from tunnel A - turn right obstructed by B
        assert!(!world.has_no_obstruction(&(8, 5), &(9, 7)));
        assert!(!world.has_no_obstruction(&(2, 5), &(1, 3)));
        assert!(!world.has_no_obstruction(&(5, 2), &(7, 1)));
        assert!(!world.has_no_obstruction(&(5, 8), &(3, 9)));

        // from tunnel A - turn left ok
        assert!(world.has_no_obstruction(&(8, 5), &(9, 3)));
        assert!(world.has_no_obstruction(&(2, 5), &(1, 7)));
        assert!(world.has_no_obstruction(&(5, 2), &(3, 1)));
        assert!(world.has_no_obstruction(&(5, 8), &(7, 9)));

        // from tunnel A - forward ok
        assert!(world.has_no_obstruction(&(8, 5), &(9, 5)));
        assert!(world.has_no_obstruction(&(2, 5), &(1, 5)));
        assert!(world.has_no_obstruction(&(5, 2), &(5, 1)));
        assert!(world.has_no_obstruction(&(5, 8), &(5, 9)));
    }

    #[test]
    #[ignore = "expensive"]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "12521");
    }

    #[test]
    #[ignore = "expensive"]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "11516");
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
