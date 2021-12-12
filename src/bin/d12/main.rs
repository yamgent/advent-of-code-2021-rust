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

    fn is_big(vertex: &str) -> bool {
        (('A' as i32)..=('Z' as i32)).contains(&(vertex.chars().next().unwrap() as i32))
    }

    fn is_small(vertex: &str) -> bool {
        (('a' as i32)..=('z' as i32)).contains(&(vertex.chars().next().unwrap() as i32))
    }

    fn count_paths_to_end(&self, current: &str, visited_smalls: &mut HashSet<String>) -> usize {
        if current == "end" {
            1
        } else {
            self.vertices
                .get(current)
                .unwrap()
                .iter()
                .map(|neighbour| {
                    if Graph::is_big(neighbour) {
                        self.count_paths_to_end(neighbour, visited_smalls)
                    } else if !visited_smalls.contains(neighbour) {
                        visited_smalls.insert(neighbour.to_owned());
                        let count = self.count_paths_to_end(neighbour, visited_smalls);
                        visited_smalls.remove(neighbour);
                        count
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        }
    }

    fn count_total_paths(&self) -> usize {
        let mut visited_smalls = HashSet::new();
        visited_smalls.insert("start".to_owned());
        self.count_paths_to_end("start", &mut visited_smalls)
    }
}

fn p1(input: &str) -> String {
    Graph::from_input(input).count_total_paths().to_string()
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

    const SAMPLE_INPUT: &str = "";

    #[test]
    fn test_p1_sample() {
        assert_eq!(
            p1(r"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"),
            "10"
        );

        assert_eq!(
            p1(r"
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
"),
            "19"
        );

        assert_eq!(
            p1(r"
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
"),
            "226"
        );
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "3779");
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
