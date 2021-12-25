use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy)]
enum Dir {
    Right,
    Down,
}

#[derive(Clone)]
struct SeaMap {
    animals: HashMap<(i32, i32), Dir>,
    max: (i32, i32),
}

impl SeaMap {
    fn parse_input(input: &str) -> SeaMap {
        let input = input
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut animals = HashMap::new();
        let max = (input[0].len() as i32, input.len() as i32);

        input.into_iter().enumerate().for_each(|(y, line)| {
            line.into_iter().enumerate().for_each(|(x, ch)| {
                let coord = (x as i32, y as i32);
                match ch {
                    '>' => {
                        animals.insert(coord, Dir::Right);
                    }
                    'v' => {
                        animals.insert(coord, Dir::Down);
                    }
                    '.' => {}
                    _ => panic!("Unknown character {}", ch),
                }
            });
        });

        SeaMap { animals, max }
    }

    fn get_right_coord(&self, coord: &(i32, i32)) -> (i32, i32) {
        ((coord.0 + 1) % self.max.0, coord.1)
    }

    fn get_down_coord(&self, coord: &(i32, i32)) -> (i32, i32) {
        (coord.0, (coord.1 + 1) % self.max.1)
    }

    fn simulate_one_step(&self) -> (SeaMap, i32) {
        let max = self.max;

        let mut animals = HashMap::new();
        let mut moved = 0;

        self.animals
            .iter()
            .filter(|(_, dir)| matches!(dir, Dir::Right))
            .for_each(|(coord, _)| {
                let new_pos = self.get_right_coord(coord);

                if self.animals.get(&new_pos).is_some() {
                    animals.insert(*coord, Dir::Right);
                } else {
                    moved += 1;
                    animals.insert(new_pos, Dir::Right);
                }
            });

        self.animals
            .iter()
            .filter(|(_, dir)| matches!(dir, Dir::Down))
            .for_each(|(coord, _)| {
                let new_pos = self.get_down_coord(coord);

                if matches!(self.animals.get(&new_pos), Some(Dir::Down))
                    || animals.get(&new_pos).is_some()
                {
                    animals.insert(*coord, Dir::Down);
                } else {
                    moved += 1;
                    animals.insert(new_pos, Dir::Down);
                }
            });

        (SeaMap { animals, max }, moved)
    }
}

fn p1(input: &str) -> String {
    let mut sea_map = SeaMap::parse_input(input);
    let mut steps = 0;

    loop {
        let (new_map, moved) = sea_map.simulate_one_step();
        steps += 1;

        if moved == 0 {
            return steps.to_string();
        }

        sea_map = new_map;
    }
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
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "58");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "308");
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
