const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    let lines = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '0' { 0 } else { 1 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let total_bits = lines[0].len();

    let counters = (0..total_bits)
        .map(|i| lines.iter().map(|line| line[i]).sum())
        .collect::<Vec<u32>>();

    let half_way_point = (lines.len() / 2) as u32;

    let gamma_rate = counters
        .iter()
        .map(|x| if *x > half_way_point { '1' } else { '0' })
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

fn filter_by_bit_criteria(
    list: Vec<&str>,
    pos: usize,
    criteria_type: BitCriteriaType,
) -> Vec<&str> {
    let chars = list
        .iter()
        .map(|line| line.chars().nth(pos).unwrap())
        .collect::<Vec<_>>();
    let zeroes = chars.iter().filter(|char| **char == '0').count();
    let ones = list.len() - zeroes;

    let filt = match criteria_type {
        BitCriteriaType::Oxygen => {
            if ones >= zeroes {
                '1'
            } else {
                '0'
            }
        }
        BitCriteriaType::CO2 => {
            if zeroes <= ones {
                '0'
            } else {
                '1'
            }
        }
    };

    list.iter()
        .filter(|line| line.chars().nth(pos).unwrap() == filt)
        .copied()
        .collect::<Vec<_>>()
}

fn p2(input: &str) -> String {
    let lines = input.trim().lines().collect::<Vec<_>>();

    let oxy = (0..lines[0].len())
        .into_iter()
        .fold(lines.clone(), |acc, i| {
            if acc.len() == 1 {
                acc
            } else {
                filter_by_bit_criteria(acc, i, BitCriteriaType::Oxygen)
            }
        })[0]
        .to_string();

    let co2 = (0..lines[0].len())
        .into_iter()
        .fold(lines.clone(), |acc, i| {
            if acc.len() == 1 {
                acc
            } else {
                filter_by_bit_criteria(acc, i, BitCriteriaType::CO2)
            }
        })[0]
        .to_string();

    let oxy = u32::from_str_radix(&oxy, 2).unwrap();
    let co2 = u32::from_str_radix(&co2, 2).unwrap();

    (oxy * co2).to_string()
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
