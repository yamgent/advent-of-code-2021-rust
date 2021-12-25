use std::ops::Index;

use regex::Regex;

const ACTUAL_INPUT: &str = include_str!("input.txt");

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
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn new_from_strs(x: &str, y: &str, z: &str) -> Self {
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }

    fn new_from_slice(val: &[i64]) -> Self {
        Self {
            x: val[0],
            y: val[1],
            z: val[2],
        }
    }

    fn clone_with_new_x(&self, x: i64) -> Self {
        Vec3::new(x, self.y, self.z)
    }

    fn clone_with_new_y(&self, y: i64) -> Self {
        Vec3::new(self.x, y, self.z)
    }

    fn clone_with_new_z(&self, z: i64) -> Self {
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
        !(0..2).any(|axis| self.max[axis] < other.min[axis] || other.max[axis] < self.min[axis])
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
                Cuboid::new(self.min, self.max.clone_with_new_x(x - 1)),
                Cuboid::new(self.min.clone_with_new_x(x), self.max),
            ]
        } else {
            vec![*self]
        }
    }

    fn cut_xz_plane(&self, plane: &Rectangle, y: i64) -> Vec<Self> {
        if self.xz_plane().intersects(plane) && ((self.min.y + 1)..(self.max.y)).contains(&y) {
            vec![
                Cuboid::new(self.min, self.max.clone_with_new_y(y - 1)),
                Cuboid::new(self.min.clone_with_new_y(y), self.max),
            ]
        } else {
            vec![*self]
        }
    }

    fn cut_xy_plane(&self, plane: &Rectangle, z: i64) -> Vec<Self> {
        if self.xy_plane().intersects(plane) && ((self.min.z + 1)..(self.max.z)).contains(&z) {
            vec![
                Cuboid::new(self.min, self.max.clone_with_new_z(z - 1)),
                Cuboid::new(self.min.clone_with_new_z(z), self.max),
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
enum CommandType {
    Off,
    On,
}

#[derive(Debug, Clone, Copy)]
struct Command {
    cmd_type: CommandType,
    region: Cuboid,
}

impl Command {
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

                Command {
                    cmd_type: if &cap[1] == "on" {
                        CommandType::On
                    } else {
                        CommandType::Off
                    },
                    region: Cuboid::new(
                        Vec3::new_from_strs(&cap[2], &cap[4], &cap[6]),
                        Vec3::new_from_strs(&cap[3], &cap[5], &cap[7]),
                    ),
                }
            })
            .collect()
    }

    fn restrict_to_neg50_50(self) -> Command {
        let min = Vec3::new_from_slice(
            &(0..3)
                .map(|axis| self.region.min[axis].max(-50))
                .collect::<Vec<_>>(),
        );
        let max = Vec3::new_from_slice(
            &(0..3)
                .map(|axis| self.region.max[axis].min(50))
                .collect::<Vec<_>>(),
        );
        Command {
            cmd_type: self.cmd_type,
            region: Cuboid::new(min, max),
        }
    }
}

fn p1(input: &str) -> String {
    Command::parse_input(input)
        .into_iter()
        .map(Command::restrict_to_neg50_50)
        .fold(vec![], |acc: Vec<Cuboid>, command| {
            let mut acc: Vec<Cuboid> = acc
                .into_iter()
                .flat_map(|cuboid| cuboid.subtract(&command.region))
                .collect();

            if let CommandType::On = command.cmd_type {
                acc.push(command.region);
            }

            acc
        })
        .into_iter()
        .map(|cuboid| cuboid.get_total_points())
        .sum::<i64>()
        .to_string()
}

fn p2(input: &str) -> String {
    "".to_string()
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
"),
            "27"
        );

        assert_eq!(
            p1(r"
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
    "),
            "46"
        );

        assert_eq!(
            p1(r"
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
    "),
            "38"
        );

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
