const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Input {
    min: (i32, i32),
    max: (i32, i32),
}

impl Input {
    fn parse_input(input: &str) -> Self {
        let (x, y) = input
            .trim()
            .strip_prefix("target area: ")
            .unwrap()
            .split_once(", ")
            .unwrap();

        let x = x.strip_prefix("x=").unwrap().split_once("..").unwrap();
        let y = y.strip_prefix("y=").unwrap().split_once("..").unwrap();

        Self {
            min: (x.0.parse().unwrap(), y.0.parse().unwrap()),
            max: (x.1.parse().unwrap(), y.1.parse().unwrap()),
        }
    }

    fn in_range(&self, coord: &(i32, i32)) -> bool {
        coord.0 >= self.min.0
            && coord.0 <= self.max.0
            && coord.1 >= self.min.1
            && coord.1 <= self.max.1
    }
}

fn simulate(input: &Input, mut vel: (i32, i32)) -> (i32, bool) {
    assert!(vel.0 >= 0, "This method cannot handle negative");

    let mut highest_y = 0;
    let mut pos = (0, 0);

    while pos.0 <= input.max.0 && pos.1 >= input.min.1 {
        pos.0 += vel.0;
        pos.1 += vel.1;

        vel.0 = if vel.0 == 0 { 0 } else { vel.0 - 1 };
        vel.1 -= 1;

        highest_y = highest_y.max(pos.1);

        if input.in_range(&pos) {
            return (highest_y, true);
        }
    }

    (0, false)
}

fn p1(input: &str) -> String {
    let input = Input::parse_input(input);

    (0..1000)
        .map(|x_vel| {
            (-1000..1000)
                .map(|y_vel| simulate(&input, (x_vel, y_vel)).0)
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .to_string()
}

fn p2(input: &str) -> String {
    let input = Input::parse_input(input);

    (0..1000)
        .map(|x_vel| {
            (-1000..1000)
                .map(|y_vel| {
                    if simulate(&input, (x_vel, y_vel)).1 {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "45");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "6903");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "112");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "2351");
    }
}
