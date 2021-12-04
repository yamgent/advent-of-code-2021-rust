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
    coords: HashMap<i32, (i32, i32)>,
}

struct BoardWinState {
    called_index: i32,
    called_number: i32,
    unmarked_numbers_sum: i32,
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

        Self { coords }
    }

    fn get_win_state(&self, called_numbers: &[i32]) -> Option<BoardWinState> {
        let mut rows_remaining = iter::repeat(BOARD_SIZE)
            .take(BOARD_SIZE as usize)
            .collect::<Vec<_>>();
        let mut cols_remaining = iter::repeat(BOARD_SIZE)
            .take(BOARD_SIZE as usize)
            .collect::<Vec<_>>();

        let mut remaining_numbers = self.coords.clone();

        called_numbers
            .iter()
            .enumerate()
            .find_map(|(index, number)| match remaining_numbers.remove(number) {
                None => None,
                Some(number_coord) => {
                    {
                        let y = number_coord.0 as usize;
                        rows_remaining[y] -= 1;

                        if rows_remaining[y] == 0 {
                            return Some(BoardWinState {
                                called_index: index as i32,
                                called_number: *number,
                                unmarked_numbers_sum: remaining_numbers.keys().into_iter().sum(),
                            });
                        }
                    }

                    {
                        let x = number_coord.1 as usize;
                        cols_remaining[x] -= 1;
                        if cols_remaining[x] == 0 {
                            return Some(BoardWinState {
                                called_index: index as i32,
                                called_number: *number,
                                unmarked_numbers_sum: remaining_numbers.keys().into_iter().sum(),
                            });
                        }
                    }

                    None
                }
            })
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
    let input = Input::parse_input(input);

    let result = input
        .boards
        .into_iter()
        .flat_map(|board| board.get_win_state(&input.called_numbers))
        .min_by(|x, y| x.called_index.cmp(&y.called_index))
        .expect("Cannot find a winning board");

    (result.called_number * result.unmarked_numbers_sum).to_string()
}

fn p2(input: &str) -> String {
    let input = Input::parse_input(input);

    let result = input
        .boards
        .into_iter()
        .flat_map(|board| board.get_win_state(&input.called_numbers))
        .max_by(|x, y| x.called_index.cmp(&y.called_index))
        .expect("Cannot find a last losing board");

    (result.called_number * result.unmarked_numbers_sum).to_string()
}

fn main() {
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
