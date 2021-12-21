use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(' ')
                .rev()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                - 1
        })
        .collect()
}

struct Die {
    value: i32,
    roll_count: i32,
}

impl Die {
    fn new() -> Self {
        Self {
            value: 0,
            roll_count: 0,
        }
    }
    fn roll(&mut self) -> i32 {
        let result = self.value + 1;
        self.value = (self.value + 1) % 100;
        self.roll_count += 1;
        result
    }
}

struct Player {
    pos: i32,
    score: i32,
}

impl Player {
    fn new(pos: i32) -> Self {
        Self { pos, score: 0 }
    }

    fn roll_and_move(&mut self, die: &mut Die) -> bool {
        let total_moves = (0..3).map(|_| die.roll()).sum::<i32>();
        self.pos = (self.pos + total_moves) % 10;
        self.score += self.pos + 1;
        self.score >= 1000
    }
}

fn p1(input: &str) -> String {
    let input = parse_input(input);

    let mut player = [Player::new(input[0]), Player::new(input[1])];
    let mut die = Die::new();

    loop {
        if player[0].roll_and_move(&mut die) {
            return (player[1].score * die.roll_count).to_string();
        }

        if player[1].roll_and_move(&mut die) {
            return (player[0].score * die.roll_count).to_string();
        }
    }
}

fn p2(input: &str) -> String {
    let input = parse_input(input);

    let mut roll_distribution = HashMap::new();
    (1..=3).for_each(|die1| {
        (1..=3).for_each(|die2| {
            (1..=3).for_each(|die3| {
                let total = die1 + die2 + die3;
                *roll_distribution.entry(total).or_insert(0u64) += 1;
            });
        });
    });

    let mut scores = HashMap::new();
    let mut currently_player1 = true;
    let mut wins = [0u64, 0];

    scores.insert((input[0], input[1], 0, 0), 1u64);

    while !scores.is_empty() {
        let mut new_scores = HashMap::new();
        scores
            .into_iter()
            .for_each(|((pos1, pos2, score1, score2), count)| {
                roll_distribution.iter().for_each(|(roll, roll_count)| {
                    let mut new_score = (pos1, pos2, score1, score2);

                    if currently_player1 {
                        new_score.0 = (new_score.0 + roll) % 10;
                        new_score.2 += new_score.0 + 1;
                    } else {
                        new_score.1 = (new_score.1 + roll) % 10;
                        new_score.3 += new_score.1 + 1;
                    }

                    let new_count = count * roll_count;
                    if new_score.2 >= 21 {
                        wins[0] += new_count;
                    } else if new_score.3 >= 21 {
                        wins[1] += new_count;
                    } else {
                        *new_scores.entry(new_score).or_insert(0u64) += new_count;
                    }
                });
            });
        scores = new_scores;
        currently_player1 = !currently_player1;
    }

    wins.into_iter().max().unwrap().to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "739785");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "597600");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "444356092776315");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "634769613696613");
    }
}
