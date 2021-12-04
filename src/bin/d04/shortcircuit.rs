use std::{collections::HashMap, iter};

const ACTUAL_INPUT: &str = include_str!("input.txt");
const BOARD_SIZE: i32 = 5;

fn parse_called_numbers(line: &str) -> Vec<i32> {
    line.split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}

struct Board {
    remaining_numbers: HashMap<i32, (i32, i32)>,
    remaining_numbers_sum: i32,

    rows_remaining: Vec<i32>,
    cols_remaining: Vec<i32>,
    won: bool,
}

impl Board {
    fn parse_from_lines(lines: Vec<&str>) -> Self {
        if lines.len() != BOARD_SIZE as usize {
            panic!("Expected 5x5 board, found {} rows", lines.len());
        }

        let mut coords = HashMap::new();

        lines
            .into_iter()
            .map(|line| {
                line.split(' ')
                    .filter(|x| !x.is_empty()) // input contains multiple spaces, these will show up as "" unless we remove them
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>()
            })
            .enumerate()
            .for_each(|(y, rows)| {
                if rows.len() != BOARD_SIZE as usize {
                    panic!("Expected 5x5 board, found {} cols in row {}", rows.len(), y);
                }

                rows.into_iter().enumerate().for_each(|(x, number)| {
                    coords.insert(number, (y as i32, x as i32));
                });
            });

        let sum = coords.keys().sum();
        let rows_remaining = iter::repeat(BOARD_SIZE)
            .take(BOARD_SIZE as usize)
            .collect::<Vec<_>>();
        let cols_remaining = iter::repeat(BOARD_SIZE)
            .take(BOARD_SIZE as usize)
            .collect::<Vec<_>>();

        Self {
            remaining_numbers: coords,
            remaining_numbers_sum: sum,
            rows_remaining,
            cols_remaining,
            won: false,
        }
    }

    fn update(&mut self, called_number: i32) -> bool {
        if self.won {
            panic!("We already won but still cancelling numbers");
        }

        if let Some(number_coord) = self.remaining_numbers.remove(&called_number) {
            self.remaining_numbers_sum -= called_number;

            {
                let y = number_coord.0 as usize;
                self.rows_remaining[y] -= 1;

                if self.rows_remaining[y] == 0 {
                    self.won = true;
                    return true;
                }
            }

            {
                let x = number_coord.1 as usize;
                self.cols_remaining[x] -= 1;
                if self.cols_remaining[x] == 0 {
                    self.won = true;
                    return true;
                }
            }
        }

        false
    }

    fn get_remaining_sum(&self) -> i32 {
        self.remaining_numbers_sum
    }

    fn get_is_won(&self) -> bool {
        self.won
    }
}

struct Input {
    called_numbers: Vec<i32>,
    boards: Vec<Board>,
}

impl Input {
    fn parse_input(input: &str) -> Self {
        let mut lines = input.trim().lines();

        let called_numbers = parse_called_numbers(lines.next().unwrap());

        let mut boards = vec![];

        loop {
            lines.next();

            let current_board = (0..BOARD_SIZE)
                .map(|_| lines.next())
                .flatten()
                .collect::<Vec<_>>();

            if current_board.is_empty() {
                // we finish parsing
                break;
            }

            boards.push(Board::parse_from_lines(current_board));
        }

        Self {
            called_numbers,
            boards,
        }
    }
}

fn p1(input: &str) -> String {
    let mut input = Input::parse_input(input);

    input
        .called_numbers
        .into_iter()
        .find_map(|called_number| {
            input.boards.iter_mut().find_map(|board| {
                if board.update(called_number) {
                    Some(called_number * board.get_remaining_sum())
                } else {
                    None
                }
            })
        })
        .unwrap()
        .to_string()
}

fn p2(input: &str) -> String {
    let input = Input::parse_input(input);

    let (board, calling_number) =
        input
            .called_numbers
            .into_iter()
            .fold((input.boards, None), |result, called_number| {
                if result.1.is_some() {
                    result
                } else if result.0.len() == 1 {
                    let boards = result
                        .0
                        .into_iter()
                        .map(|mut board| {
                            board.update(called_number);
                            board
                        })
                        .collect::<Vec<_>>();

                    let last_number = if boards[0].get_is_won() {
                        Some(called_number)
                    } else {
                        None
                    };

                    (boards, last_number)
                } else {
                    (
                        result
                            .0
                            .into_iter()
                            .map(|mut board| {
                                board.update(called_number);
                                board
                            })
                            .filter(|board| !board.get_is_won())
                            .collect::<Vec<_>>(),
                        None,
                    )
                }
            });

    (calling_number.unwrap() * board[0].get_remaining_sum()).to_string()
}

pub fn main_shortcircuit() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "4512");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "50008");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "1924");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "17408");
    }
}
