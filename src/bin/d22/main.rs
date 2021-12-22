use std::collections::HashSet;
use std::ops::RangeInclusive;

use regex::Regex;

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
enum CommandType {
    On,
    Off,
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
}

fn create_range(start: &str, end_inclusive: &str) -> (i32, i32) {
    (
        start.parse::<i32>().unwrap(),
        end_inclusive.parse::<i32>().unwrap(),
    )
}

fn parse_commands(input: &str) -> Vec<Command> {
    let re = Regex::new(
        r"(on|off) x=(-*[\d]+)\.\.(-*[\d]+),y=(-*[\d]+)\.\.(-*[\d]+),z=(-*[\d]+)\.\.(-*[\d]+)",
    )
    .unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();

            Command {
                command_type: if &cap[1] == "on" {
                    CommandType::On
                } else {
                    CommandType::Off
                },
                x_range: create_range(&cap[2], &cap[3]),
                y_range: create_range(&cap[4], &cap[5]),
                z_range: create_range(&cap[6], &cap[7]),
            }
        })
        .collect()
}

fn restrict_range_to_p1_50(commands: &[Command]) -> Vec<Command> {
    fn restrict_range(range: &(i32, i32)) -> (i32, i32) {
        (range.0.max(-50), range.1.min(50))
    }

    commands
        .iter()
        .map(|command| Command {
            command_type: command.command_type,
            x_range: restrict_range(&command.x_range),
            y_range: restrict_range(&command.y_range),
            z_range: restrict_range(&command.z_range),
        })
        .collect()
}

fn get_range_inclusive(range: &(i32, i32)) -> RangeInclusive<i32> {
    (range.0)..=(range.1)
}

fn p1(input: &str) -> String {
    let mut on = HashSet::new();

    restrict_range_to_p1_50(&parse_commands(input))
        .into_iter()
        .for_each(|command| {
            for x in get_range_inclusive(&command.x_range) {
                for y in get_range_inclusive(&command.y_range) {
                    for z in get_range_inclusive(&command.z_range) {
                        let coord = (x, y, z);

                        match command.command_type {
                            CommandType::On => {
                                on.insert(coord);
                            }
                            CommandType::Off => {
                                on.remove(&coord);
                            }
                        }
                    }
                }
            }
        });

    on.len().to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(
            p1(r"
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
    "),
            "39"
        );
        assert_eq!(
            p1(r"
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
    "),
            "590784"
        );
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "581108");
    }

    #[test]
    #[ignore = "not implemented yet"]
    fn test_p2_sample() {
        assert_eq!(
            p2(r"
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
"),
            "2758514936282235"
        );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
