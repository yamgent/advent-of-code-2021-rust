use std::collections::HashSet;
use std::ops::{Index, RangeInclusive};

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

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Index<usize> for Vec2 {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    min: Vec2,
    max: Vec2,
}

impl Rectangle {
    fn new(min: Vec2, max: Vec2) -> Self {
        (0..2).for_each(|axis| {
            if min[axis] > max[axis] {
                panic!(
                    "Illegal rectangle values for axis {}: min {:?}, max {:?}",
                    axis, min, max
                );
            }
        });

        Self { min, max }
    }

    fn intersects(&self, other: &Self) -> bool {
        !(0..2).all(|axis| self.max[axis] < other.min[axis] || other.max[axis] < self.min[axis])
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn new_from_str(x: &str, y: &str, z: &str) -> Self {
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }

    fn new_x(&self, x: i64) -> Self {
        Vec3::new(x, self.y, self.z)
    }

    fn new_y(&self, y: i64) -> Self {
        Vec3::new(self.x, y, self.z)
    }

    fn new_z(&self, z: i64) -> Self {
        Vec3::new(self.x, self.y, z)
    }

    // swizzle to xy
    fn xy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    // swizzle to xz
    fn xz(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }

    // swizzle to yz
    fn yz(&self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    min: Vec3,
    max: Vec3,
}

impl Cuboid {
    fn new(min: Vec3, max: Vec3) -> Self {
        (0..3).for_each(|axis| {
            if min[axis] > max[axis] {
                panic!(
                    "Illegal cuboid values for axis {}: min {:?}, max {:?}",
                    axis, min, max
                );
            }
        });

        Self { min, max }
    }

    fn yz_plane(&self) -> Rectangle {
        Rectangle::new(self.min.yz(), self.max.yz())
    }

    fn xz_plane(&self) -> Rectangle {
        Rectangle::new(self.min.xz(), self.max.xz())
    }

    fn xy_plane(&self) -> Rectangle {
        Rectangle::new(self.min.xy(), self.max.xy())
    }

    fn cut_yz_plane(&self, plane: &Rectangle, x: i64) -> Vec<Self> {
        if self.yz_plane().intersects(plane) && ((self.min.x + 1)..(self.max.x)).contains(&x) {
            vec![
                Cuboid::new(self.min, self.max.new_x(x - 1)),
                Cuboid::new(self.min.new_x(x), self.max),
            ]
        } else {
            vec![*self]
        }
    }

    fn cut_xz_plane(&self, plane: &Rectangle, y: i64) -> Vec<Self> {
        if self.xz_plane().intersects(plane) && ((self.min.y + 1)..(self.max.y)).contains(&y) {
            vec![
                Cuboid::new(self.min, self.max.new_y(y - 1)),
                Cuboid::new(self.min.new_y(y), self.max),
            ]
        } else {
            vec![*self]
        }
    }

    fn cut_xy_plane(&self, plane: &Rectangle, z: i64) -> Vec<Self> {
        if self.xy_plane().intersects(plane) && ((self.min.z + 1)..(self.max.z)).contains(&z) {
            vec![
                Cuboid::new(self.min, self.max.new_z(z - 1)),
                Cuboid::new(self.min.new_z(z), self.max),
            ]
        } else {
            vec![*self]
        }
    }

    fn contains(&self, other: &Self) -> bool {
        (0..3).all(|axis| other.min[axis] >= self.min[axis] && other.max[axis] <= self.max[axis])
    }

    fn subtract(&self, other: &Self) -> Vec<Self> {
        vec![*self]
            .into_iter()
            .flat_map(|part| part.cut_yz_plane(&other.yz_plane(), other.min.x))
            .flat_map(|part| part.cut_yz_plane(&other.yz_plane(), other.max.x))
            .flat_map(|part| part.cut_xz_plane(&other.xz_plane(), other.min.y))
            .flat_map(|part| part.cut_xz_plane(&other.xz_plane(), other.max.y))
            .flat_map(|part| part.cut_xy_plane(&other.xy_plane(), other.min.z))
            .flat_map(|part| part.cut_xy_plane(&other.xy_plane(), other.max.z))
            .filter(|part| !other.contains(part))
            .collect()
    }

    fn get_total_points(&self) -> i64 {
        (0..3)
            .map(|axis| self.max[axis] - self.min[axis] + 1)
            .product()
    }
}

#[derive(Debug, Clone, Copy)]
enum P2CommandType {
    Off,
    On,
}

#[derive(Debug, Clone, Copy)]
struct P2Command {
    cmd_type: P2CommandType,
    region: Cuboid,
}

impl P2Command {
    fn parse_input(input: &str) -> Vec<Self> {
        let re = Regex::new(
            r"(on|off) x=(-*[\d]+)\.\.(-*[\d]+),y=(-*[\d]+)\.\.(-*[\d]+),z=(-*[\d]+)\.\.(-*[\d]+)",
        )
        .unwrap();

        input
            .trim()
            .lines()
            .map(|line| {
                let cap = re.captures(line).unwrap();

                P2Command {
                    cmd_type: if &cap[1] == "on" {
                        P2CommandType::On
                    } else {
                        P2CommandType::Off
                    },
                    region: Cuboid::new(
                        Vec3::new_from_str(&cap[2], &cap[4], &cap[6]),
                        Vec3::new_from_str(&cap[3], &cap[5], &cap[7]),
                    ),
                }
            })
            .collect()
    }
}

fn p2(input: &str) -> String {
    P2Command::parse_input(input)
        .into_iter()
        .fold(vec![], |acc: Vec<Cuboid>, command| {
            let mut new_cuboids = acc
                .into_iter()
                .flat_map(|cuboid| cuboid.subtract(&command.region))
                .collect::<Vec<_>>();

            if let P2CommandType::On = command.cmd_type {
                new_cuboids.push(command.region);
            }

            new_cuboids
        })
        .into_iter()
        .fold(0, |acc, cuboid| acc + cuboid.get_total_points())
        .to_string()
}

pub fn solve_second_attempt() {
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
    #[ignore = "not yet implemented"]
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
