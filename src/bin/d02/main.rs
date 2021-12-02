const ACTUAL_INPUT: &str = include_str!("input.txt");

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let parts = line.split(' ').collect::<Vec<_>>();

        if parts.len() != 2 {
            panic!("Expected 2 parts, found '{}'", line);
        }

        let count = parts[1]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Expect value for second part, found '{}'", line));

        match parts[0] {
            "forward" => Instruction::Forward(count),
            "down" => Instruction::Down(count),
            "up" => Instruction::Up(count),
            _ => panic!("Expected action for first part, found '{}'", line),
        }
    }
}

#[derive(Default)]
struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn p1_do(&self, instruction: &Instruction) -> Self {
        match *instruction {
            Instruction::Forward(x) => Submarine {
                horizontal: self.horizontal + x,
                depth: self.depth,
                aim: self.aim,
            },
            Instruction::Down(x) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth + x,
                aim: self.aim,
            },
            Instruction::Up(x) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth - x,
                aim: self.aim,
            },
        }
    }

    fn p2_do(&self, instruction: &Instruction) -> Self {
        match *instruction {
            Instruction::Forward(x) => Submarine {
                horizontal: self.horizontal + x,
                depth: self.depth + self.aim * x,
                aim: self.aim,
            },
            Instruction::Down(x) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + x,
            },
            Instruction::Up(x) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - x,
            },
        }
    }

    fn get_answer(&self) -> String {
        (self.horizontal * self.depth).to_string()
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(Instruction::parse)
        .fold(Submarine::default(), |acc, instruction| {
            acc.p1_do(&instruction)
        })
        .get_answer()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(Instruction::parse)
        .fold(Submarine::default(), |acc, instruction| {
            acc.p2_do(&instruction)
        })
        .get_answer()
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
