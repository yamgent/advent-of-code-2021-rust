use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Vertex {
    Start,
    End,
    Big(i32),
    Small(i32),
}

struct Graph {
    vertices: HashMap<Vertex, HashSet<Vertex>>,
}

impl Graph {
    fn from_input(input: &str) -> Self {
        let lines = input
            .trim()
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .collect::<Vec<_>>();

        let mut name_to_vertex_mapping = HashMap::new();
        let mut vertices = HashMap::new();

        lines
            .iter()
            .flat_map(|line| [line.0, line.1])
            .for_each(|name| {
                if !name_to_vertex_mapping.contains_key(name) {
                    let next_index = name_to_vertex_mapping.len() as i32;

                    let vertex = if name == "start" {
                        Vertex::Start
                    } else if name == "end" {
                        Vertex::End
                    } else if Self::is_big_cave(name) {
                        Vertex::Big(next_index)
                    } else {
                        Vertex::Small(next_index)
                    };

                    name_to_vertex_mapping.insert(name, vertex);
                    vertices.insert(vertex, HashSet::new());
                }
            });

        lines.into_iter().for_each(|line| {
            let a = name_to_vertex_mapping.get(line.0).unwrap();
            let b = name_to_vertex_mapping.get(line.1).unwrap();
            vertices.get_mut(a).unwrap().insert(b.to_owned());
            vertices.get_mut(b).unwrap().insert(a.to_owned());
        });

        Self { vertices }
    }

    fn is_big_cave(vertex: &str) -> bool {
        vertex.chars().next().unwrap().is_ascii_uppercase()
    }

    fn p1_count_paths_to_end(&self, current: &Vertex, visited_smalls: &mut HashSet<i32>) -> usize {
        if matches!(current, Vertex::End) {
            1
        } else {
            self.vertices
                .get(current)
                .unwrap()
                .iter()
                .map(|neighbour| match neighbour {
                    Vertex::End | Vertex::Big(_) => {
                        self.p1_count_paths_to_end(neighbour, visited_smalls)
                    }
                    Vertex::Start => 0,
                    Vertex::Small(index) => {
                        if !visited_smalls.contains(index) {
                            visited_smalls.insert(*index);
                            let count = self.p1_count_paths_to_end(neighbour, visited_smalls);
                            visited_smalls.remove(index);
                            count
                        } else {
                            0
                        }
                    }
                })
                .sum::<usize>()
        }
    }

    fn p1_count_total_paths(&self) -> usize {
        self.p1_count_paths_to_end(&Vertex::Start, &mut HashSet::new())
    }

    // changing visited_twice_small from Option<String> to &Option<String>
    // (no reference -> reference) improved the performance of the p2() method
    // for actual input.
    //
    // on test_p2_actual(), the timings before were:
    //      1.33s, 1.34s, 1.33s, 1.33s, 1.36s
    // and after the change:
    //      1.26s, 1.26s, 1.28s, 1.27s, 1.27s
    //
    // then by optimizing further, we replace String -> i32, to avoid needless
    // copy of string
    //
    // on test_p2_actual(), the new timings are:
    //      0.75s, 0.73s, 0.73s, 0.75s, 0.75s
    //
    // all timings are in debug profile
    fn p2_count_paths_to_end(
        &self,
        current: &Vertex,
        visited_once_smalls: &mut HashSet<i32>,
        visited_twice_small: &Option<i32>,
    ) -> usize {
        if matches!(current, Vertex::End) {
            1
        } else {
            self.vertices
                .get(current)
                .unwrap()
                .iter()
                .map(|neighbour| match neighbour {
                    Vertex::End | Vertex::Big(_) => self.p2_count_paths_to_end(
                        neighbour,
                        visited_once_smalls,
                        visited_twice_small,
                    ),
                    Vertex::Start => 0,
                    Vertex::Small(index) => {
                        if !visited_once_smalls.contains(index) {
                            visited_once_smalls.insert(*index);
                            let count = self.p2_count_paths_to_end(
                                neighbour,
                                visited_once_smalls,
                                visited_twice_small,
                            );
                            visited_once_smalls.remove(index);
                            count
                        } else if visited_twice_small.is_none() {
                            self.p2_count_paths_to_end(
                                neighbour,
                                visited_once_smalls,
                                &Some(*index),
                            )
                        } else {
                            0
                        }
                    }
                })
                .sum::<usize>()
        }
    }

    fn p2_count_total_paths(&self) -> usize {
        self.p2_count_paths_to_end(&Vertex::Start, &mut HashSet::new(), &None)
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
