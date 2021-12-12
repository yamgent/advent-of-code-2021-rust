use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("input.txt");

struct Graph {
    vertices: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn from_input(input: &str) -> Self {
        let lines = input
            .trim()
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .collect::<Vec<_>>();
        let mut vertices = HashMap::new();

        lines
            .iter()
            .flat_map(|line| [line.0, line.1])
            .for_each(|vertex| {
                if !vertices.contains_key(vertex) {
                    vertices.insert(vertex.to_string(), HashSet::new());
                }
            });

        lines.into_iter().for_each(|line| {
            vertices.get_mut(line.0).unwrap().insert(line.1.to_string());
            vertices.get_mut(line.1).unwrap().insert(line.0.to_string());
        });

        Self { vertices }
    }

    fn is_big_cave(vertex: &str) -> bool {
        vertex.chars().next().unwrap().is_ascii_uppercase()
    }

    fn p1_count_paths_to_end(&self, current: &str, visited_smalls: &mut HashSet<String>) -> usize {
        if current == "end" {
            1
        } else {
            self.vertices
                .get(current)
                .unwrap()
                .iter()
                .map(|neighbour| {
                    if Graph::is_big_cave(neighbour) {
                        self.p1_count_paths_to_end(neighbour, visited_smalls)
                    } else if neighbour != "start" && !visited_smalls.contains(neighbour) {
                        visited_smalls.insert(neighbour.to_owned());
                        let count = self.p1_count_paths_to_end(neighbour, visited_smalls);
                        visited_smalls.remove(neighbour);
                        count
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        }
    }

    fn p1_count_total_paths(&self) -> usize {
        self.p1_count_paths_to_end("start", &mut HashSet::new())
    }

    fn p2_count_paths_to_end(
        &self,
        current: &str,
        visited_once_smalls: &mut HashSet<String>,
        visited_twice_small: &Option<String>,
    ) -> usize {
        if current == "end" {
            1
        } else {
            self.vertices
                .get(current)
                .unwrap()
                .iter()
                .map(|neighbour| {
                    if Graph::is_big_cave(neighbour) {
                        self.p2_count_paths_to_end(
                            neighbour,
                            visited_once_smalls,
                            visited_twice_small,
                        )
                    } else if neighbour != "start" {
                        if visited_once_smalls.contains(neighbour) {
                            if visited_twice_small.is_none() {
                                self.p2_count_paths_to_end(
                                    neighbour,
                                    visited_once_smalls,
                                    &Some(neighbour.to_owned()),
                                )
                            } else {
                                0
                            }
                        } else {
                            visited_once_smalls.insert(neighbour.to_owned());
                            let count = self.p2_count_paths_to_end(
                                neighbour,
                                visited_once_smalls,
                                visited_twice_small,
                            );
                            visited_once_smalls.remove(neighbour);
                            count
                        }
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        }
    }

    fn p2_count_total_paths(&self) -> usize {
        self.p2_count_paths_to_end("start", &mut HashSet::new(), &None)
    }
}

fn p1(input: &str) -> String {
    Graph::from_input(input).p1_count_total_paths().to_string()
}

fn p2(input: &str) -> String {
    Graph::from_input(input).p2_count_total_paths().to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_SAMPLE: &str = r"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    const LARGE_SAMPLE: &str = r"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    const LARGEST_SAMPLE: &str = r"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SMALL_SAMPLE), "10");
        assert_eq!(p1(LARGE_SAMPLE), "19");
        assert_eq!(p1(LARGEST_SAMPLE), "226");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "3779");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SMALL_SAMPLE), "36");
        assert_eq!(p2(LARGE_SAMPLE), "103");
        assert_eq!(p2(LARGEST_SAMPLE), "3509");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "96988");
    }
}
