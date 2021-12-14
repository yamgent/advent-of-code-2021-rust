use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("input.txt");

fn p1(input: &str) -> String {
    let (template, rules_string) = input.trim().split_once("\n\n").unwrap();

    let template = template.to_owned();
    let mut rules = HashMap::new();

    rules_string.split('\n').for_each(|rule| {
        let (input, output) = rule.split_once(" -> ").unwrap();
        rules.insert(input.to_owned(), output.to_owned());
    });

    let result = (0..10).fold(template, |acc, _| {
        let mut updated = String::new();

        let chars = acc.chars().collect::<Vec<_>>();
        chars.windows(2).into_iter().for_each(|pair| {
            let input = pair.iter().collect::<String>();
            updated.push(pair[0]);
            updated.push_str(rules.get(&input).unwrap());
        });

        updated.push(chars[chars.len() - 1]);

        updated
    });

    let mut count = HashMap::new();

    result.chars().for_each(|c| {
        *count.entry(c).or_insert(0) += 1;
    });

    let max = *count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let min = *count.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;

    (max - min).to_string()
}

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

fn p2(input: &str) -> String {
    let (template, rules) = parse_input(input);

    let mut counts: HashMap<char, i128> = HashMap::new();
    template.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1;
    });

    let mut pairs_seen: HashMap<String, i128> = HashMap::new();
    template
        .chars()
        .zip(template.chars().skip(1))
        .for_each(|pair| {
            let pair = format!("{}{}", pair.0, pair.1);
            *pairs_seen.entry(pair).or_insert(0) += 1;
        });

    let steps = 40;

    let (counts, _) = (0..steps).fold((counts, pairs_seen), |(mut counts, pairs_seen), _| {
        let mut updated_seen = HashMap::new();

        pairs_seen.into_iter().for_each(|occurence| {
            let first_char = occurence.0.chars().next().unwrap();
            let second_char = rules.get(&occurence.0).unwrap();
            let third_char = occurence.0.chars().nth(1).unwrap();

            let first_pair = format!("{}{}", first_char, second_char);
            let second_pair = format!("{}{}", second_char, third_char);

            *updated_seen.entry(first_pair).or_insert(0) += occurence.1;
            *updated_seen.entry(second_pair).or_insert(0) += occurence.1;
            *counts.entry(*second_char).or_insert(0) += occurence.1;
        });

        (counts, updated_seen)
    });

    let max = *counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let min = *counts.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;

    (max - min).to_string()
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
