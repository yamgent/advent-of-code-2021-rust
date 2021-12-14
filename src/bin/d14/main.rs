use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> (String, HashMap<String, char>) {
    let (template, rule_lines) = input.trim().split_once("\n\n").unwrap();

    let mut rules = HashMap::new();
    rule_lines.split('\n').for_each(|line| {
        let (input, output) = line.split_once(" -> ").unwrap();
        let output = output.chars().next().expect("Output is empty");

        rules.insert(input.to_owned(), output);
    });

    (template.to_owned(), rules)
}

fn solve(input: &str, steps: i32) -> String {
    let (template, rules) = parse_input(input);

    let mut counts: HashMap<char, i64> = HashMap::new();
    template.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1;
    });

    let mut pairs_seen: HashMap<String, i64> = HashMap::new();
    template
        .chars()
        .zip(template.chars().skip(1))
        .for_each(|pair| {
            let pair = format!("{}{}", pair.0, pair.1);
            *pairs_seen.entry(pair).or_insert(0) += 1;
        });

    let (counts, _) = (0..steps).fold((counts, pairs_seen), |(mut counts, pairs_seen), _| {
        let mut updated_seen = HashMap::new();

        pairs_seen.into_iter().for_each(|occurence| {
            let first_char = occurence.0.chars().next().unwrap();
            let third_char = occurence.0.chars().nth(1).unwrap();
            let second_char = rules.get(&occurence.0).unwrap();
            let total = occurence.1;

            let first_pair = format!("{}{}", first_char, second_char);
            let second_pair = format!("{}{}", second_char, third_char);

            *updated_seen.entry(first_pair).or_insert(0) += total;
            *updated_seen.entry(second_pair).or_insert(0) += total;
            *counts.entry(*second_char).or_insert(0) += total;
        });

        (counts, updated_seen)
    });

    let counts = counts.into_iter().map(|count| count.1).collect::<Vec<_>>();

    let max = *counts.iter().max().unwrap();
    let min = *counts.iter().min().unwrap();

    (max - min).to_string()
}

fn p1(input: &str) -> String {
    solve(input, 10)
}

fn p2(input: &str) -> String {
    solve(input, 40)
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "1588");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "2587");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "2188189693529");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "3318837563123");
    }
}
