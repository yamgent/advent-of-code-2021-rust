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

struct P1Die {
    value: i32,
    roll_count: i32,
}

impl P1Die {
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

struct P1Player {
    pos: i32,
    score: i32,
}

impl P1Player {
    fn new(pos: i32) -> Self {
        Self { pos, score: 0 }
    }

    fn roll_and_move(&mut self, die: &mut P1Die) -> bool {
        let total_moves = (0..3).map(|_| die.roll()).sum::<i32>();
        self.pos = (self.pos + total_moves) % 10;
        self.score += self.pos + 1;
        self.score >= 1000
    }
}

fn p1(input: &str) -> String {
    let input = parse_input(input);

    let mut player = [P1Player::new(input[0]), P1Player::new(input[1])];
    let mut die = P1Die::new();

    loop {
        if player[0].roll_and_move(&mut die) {
            return (player[1].score * die.roll_count).to_string();
        }

        if player[1].roll_and_move(&mut die) {
            return (player[0].score * die.roll_count).to_string();
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct P2State {
    pos: [i32; 2],
    scores: [i32; 2],
}

impl P2State {
    fn advance(&self, roll: i32, is_player_1_turn: bool) -> Self {
        let player_index = if is_player_1_turn { 0 } else { 1 };
        let mut pos = self.pos.clone();
        let mut scores = self.scores.clone();

        pos[player_index] = (pos[player_index] + roll) % 10;
        scores[player_index] += pos[player_index] + 1;

        Self { pos, scores }
    }

    fn player_1_won(&self) -> bool {
        self.scores[0] >= 21
    }

    fn player_2_won(&self) -> bool {
        self.scores[1] >= 21
    }
}

fn p2(input: &str) -> String {
    let input = parse_input(input);

    let mut roll_distribution = HashMap::new();
    (1..=3).for_each(|die1| {
        (1..=3).for_each(|die2| {
            (1..=3).for_each(|die3| {
                let total = die1 + die2 + die3;
                *roll_distribution.entry(total).or_insert(0) += 1;
            });
        });
    });

    let mut states = HashMap::new();
    let mut currently_player_1 = true;
    let mut wins = [0u64, 0];

    states.insert(
        P2State {
            pos: [input[0], input[1]],
            scores: [0, 0],
        },
        1u64,
    );

    while !states.is_empty() {
        let mut new_states = HashMap::new();

        states.into_iter().for_each(|(state, count)| {
            roll_distribution.iter().for_each(|(roll, roll_count)| {
                let new_state = state.advance(*roll, currently_player_1);
                let new_count = count * roll_count;

                if new_state.player_1_won() {
                    wins[0] += new_count;
                } else if new_state.player_2_won() {
                    wins[1] += new_count;
                } else {
                    *new_states.entry(new_state).or_insert(0u64) += new_count;
                }
            });
        });

        states = new_states;
        currently_player_1 = !currently_player_1;
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
