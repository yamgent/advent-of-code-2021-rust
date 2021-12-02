const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    let result = input.trim().lines().fold((0, 0), |acc, instruction| {
        let parts = instruction.split(' ').collect::<Vec<_>>();

        if parts.len() != 2 {
            panic!("Expected 2 parts, found {}", instruction);
        }

        let count = parts[1].parse::<i32>().unwrap();

        match parts[0] {
            "forward" => (acc.0 + count, acc.1),
            "down" => (acc.0, acc.1 + count),
            "up" => (acc.0, acc.1 - count),
            _ => panic!("Expected action, found {}", parts[0]),
        }
    });

    (result.0 * result.1).to_string()
}

struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn p2(input: &str) -> String {
    let result = input.trim().lines().fold(
        Submarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |acc, instruction| {
            let parts = instruction.split(' ').collect::<Vec<_>>();

            if parts.len() != 2 {
                panic!("Expected 2 parts, found {}", instruction);
            }

            let count = parts[1].parse::<i32>().unwrap();

            match parts[0] {
                "forward" => Submarine {
                    horizontal: acc.horizontal + count,
                    depth: acc.depth + acc.aim * count,
                    aim: acc.aim,
                },
                "down" => Submarine {
                    horizontal: acc.horizontal,
                    depth: acc.depth,
                    aim: acc.aim + count,
                },
                "up" => Submarine {
                    horizontal: acc.horizontal,
                    depth: acc.depth,
                    aim: acc.aim - count,
                },
                _ => panic!("Expected action, found {}", parts[0]),
            }
        },
    );

    (result.horizontal * result.depth).to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "150");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1698735");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "900");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1594785890");
    }
}
