const ACTUAL_INPUT: &str = include_str!("input.txt");

fn get_matrix_from_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '0' { 0 } else { 1 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn p1(input: &str) -> String {
    let matrix = get_matrix_from_input(input);
    let total_bits = matrix[0].len();
    let total_lines = matrix.len() as i32;

    let counters = (0..total_bits)
        .map(|i| matrix.iter().map(|line| line[i]).sum::<i32>())
        .collect::<Vec<_>>();

    let gamma_rate = counters
        .iter()
        .map(|x| if *x > total_lines / 2 { '1' } else { '0' })
        .collect::<String>();
    let epsilon_rate = gamma_rate
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect::<String>();

    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();

    (gamma_rate * epsilon_rate).to_string()
}

enum BitCriteriaType {
    Oxygen,
    CO2,
}

fn find_entry_by_bit_criteria(mut matrix: Vec<Vec<i32>>, criteria_type: BitCriteriaType) -> u32 {
    let total_bits = matrix[0].len();

    for i in 0..total_bits {
        let ones = matrix.iter().map(|line| line[i]).sum::<i32>();
        let zeroes = matrix.len() as i32 - ones;
        let selected = match criteria_type {
            BitCriteriaType::Oxygen => {
                if ones >= zeroes {
                    1
                } else {
                    0
                }
            }
            BitCriteriaType::CO2 => {
                if zeroes <= ones {
                    0
                } else {
                    1
                }
            }
        };

        matrix.retain(|x| x[i] == selected);

        if matrix.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(
        &matrix[0].iter().map(|x| x.to_string()).collect::<String>(),
        2,
    )
    .unwrap()
}

fn p2(input: &str) -> String {
    let matrix = get_matrix_from_input(input);
    let oxygen = find_entry_by_bit_criteria(matrix.clone(), BitCriteriaType::Oxygen);
    let co2 = find_entry_by_bit_criteria(matrix, BitCriteriaType::CO2);
    (oxygen * co2).to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "198");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1082324");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "230");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1353024");
    }
}
