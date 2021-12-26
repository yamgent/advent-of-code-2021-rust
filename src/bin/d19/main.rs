use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

fn sin(degree: i32) -> i32 {
    match degree {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!("This method doesn't accept non-90 degrees"),
    }
}

fn cos(degree: i32) -> i32 {
    match degree {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!("This method doesn't accept non-90 degrees"),
    }
}

impl Vec3 {
    fn new_from_array(arr: &[i32]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }

    fn parse_line(line: &str) -> Self {
        Self::new_from_array(
            &line
                .trim()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>(),
        )
    }

    fn new(x: i32, y: i32, z: i32) -> Self {
        Self::new_from_array(&[x, y, z])
    }

    // anti-clockwise direction, right-hand coordinate system
    fn rotate_x(&self, degree: i32) -> Self {
        Self {
            x: self.x,
            y: self.y * cos(degree) - self.z * sin(degree),
            z: self.y * sin(degree) + self.z * cos(degree),
        }
    }

    // anti-clockwise direction, right-hand coordinate system
    fn rotate_y(&self, degree: i32) -> Self {
        Self {
            x: self.x * cos(degree) + self.z * sin(degree),
            y: self.y,
            z: -self.x * sin(degree) + self.z * cos(degree),
        }
    }

    // anti-clockwise direction, right-hand coordinate system
    fn rotate_z(&self, degree: i32) -> Self {
        Self {
            x: self.x * cos(degree) - self.y * sin(degree),
            y: self.x * sin(degree) + self.y * cos(degree),
            z: self.z,
        }
    }

    fn orient_iter(&self) -> Vec3OrientIterator {
        Vec3OrientIterator {
            vec: *self,
            index: 0,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

struct Vec3OrientIterator {
    vec: Vec3,
    index: usize,
}

impl Iterator for Vec3OrientIterator {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 24 {
            None
        } else {
            let result = match self.index / 4 {
                0 => self.vec,
                1 => self.vec.rotate_x(90),
                2 => self.vec.rotate_x(180),
                3 => self.vec.rotate_x(270),
                4 => self.vec.rotate_y(90),
                5 => self.vec.rotate_y(270),
                _ => unreachable!(),
            };
            let result = result.rotate_z((self.index as i32 % 4) * 90);
            self.index += 1;
            Some(result)
        }
    }
}

fn get_deltas(all_points: &[Vec3], point: &Vec3) -> Vec<Vec3> {
    all_points
        .iter()
        .filter(|p| **p != *point)
        .map(|p| *p - *point)
        .collect()
}

impl Vec3CollectionOrientIterator {
    fn get_orient_iters(points: &[Vec3]) -> Self {
        Self {
            iters: points.iter().map(|p| p.orient_iter()).collect(),
            index: 0,
        }
    }
}

struct Vec3CollectionOrientIterator {
    iters: Vec<Vec3OrientIterator>,
    index: usize,
}

impl Iterator for Vec3CollectionOrientIterator {
    type Item = (Vec<Vec3>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 24 {
            None
        } else {
            let result = self
                .iters
                .iter_mut()
                .map(|iter| iter.next())
                .map(Option::unwrap)
                .collect();

            self.index += 1;
            Some((result, self.index - 1))
        }
    }
}

fn get_equivalent_count(a: &[Vec3], b: &[Vec3]) -> i32 {
    // we cannot use hashset, because there might be multiple
    // copies of the same coord. So we must use hashmap
    // instead
    let mut remaining: HashMap<Vec3, usize> = HashMap::new();
    let mut result = 0;

    a.iter().for_each(|a_vec| {
        *remaining.entry(*a_vec).or_insert(0) += 1;
    });

    b.iter().for_each(|b_vec| {
        let b_count = remaining.entry(*b_vec).or_insert(0);
        if *b_count > 0 {
            result += 1;
            *b_count -= 1;
        }
    });

    result
}

#[derive(Debug, PartialEq)]
struct ViewDiff {
    common: Vec<Vec3>,
    left_remaining: Vec<Vec3>,
    right_remaining: Vec<Vec3>,
}

#[derive(Debug, Clone)]
struct View {
    beacons: Vec<Vec3>,
}

impl PartialEq for View {
    fn eq(&self, other: &Self) -> bool {
        if self.beacons.len() != other.beacons.len() {
            false
        } else {
            get_equivalent_count(&self.beacons, &other.beacons) == self.beacons.len() as i32
        }
    }
}

impl View {
    fn parse_input(input: &str) -> Vec<Self> {
        input
            .trim()
            .split("\n\n")
            .map(|view| Self {
                beacons: view.trim().lines().skip(1).map(Vec3::parse_line).collect(),
            })
            .collect()
    }

    fn combine_both_views(&self, other: &Self, threshold: i32) -> Option<Self> {
        self.beacons
            .iter()
            .find_map(|self_b1| {
                let self_deltas = get_deltas(&self.beacons, self_b1);

                other.beacons.iter().find_map(|other_b1| {
                    let other_deltas = get_deltas(&other.beacons, other_b1);

                    Vec3CollectionOrientIterator::get_orient_iters(&other_deltas).find_map(
                        |(deltas, index)| {
                            if get_equivalent_count(&self_deltas, &deltas) >= threshold - 1 {
                                Some((*self_b1, *other_b1, index))
                            } else {
                                None
                            }
                        },
                    )
                })
            })
            .map(|(self_b1, other_b1, index)| {
                let other_b1 = other_b1.orient_iter().nth(index).unwrap();

                Self {
                    beacons: self
                        .beacons
                        .iter()
                        .copied()
                        .chain(
                            Vec3CollectionOrientIterator::get_orient_iters(&other.beacons)
                                .nth(index)
                                .unwrap()
                                .0
                                .iter()
                                .map(|point| *point + (self_b1 - other_b1)),
                        )
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .collect(),
                }
            })
    }

    fn combine_all_views(views: &[View], threshold: i32) -> View {
        let total = views.len();
        let mut views = Vec::from(views);

        while views.len() > 1 {
            let (i, j, new_view) = views
                .iter()
                .enumerate()
                .find_map(|(i, view_i)| {
                    views
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| i != *j)
                        .find_map(|(j, view_j)| {
                            view_i
                                .combine_both_views(view_j, threshold)
                                .map(|new_view| (i, j, new_view))
                        })
                })
                .unwrap();

            views.swap_remove(i.max(j));
            views.swap_remove(i.min(j));
            views.push(new_view);
            println!(
                "Progress {}/{} (Combining: {} and {})",
                total - views.len(),
                total - 1,
                i,
                j
            );
        }

        views.remove(0)
    }

    fn debug_get_diff(&self, right: &Self) -> ViewDiff {
        let mut left_remaining: HashMap<Vec3, usize> = HashMap::new();
        let mut common = vec![];
        let mut right_remaining = vec![];

        self.beacons.iter().for_each(|a_vec| {
            *left_remaining.entry(*a_vec).or_insert(0) += 1;
        });

        right.beacons.iter().for_each(|b_vec| {
            let b_count = left_remaining.entry(*b_vec).or_insert(0);
            if *b_count > 0 {
                common.push(*b_vec);
                *b_count -= 1;
            } else {
                right_remaining.push(*b_vec);
            }
        });

        let left_remaining = left_remaining
            .iter()
            .filter(|val| *val.1 > 0)
            .map(|val| *val.0)
            .collect();

        ViewDiff {
            common,
            left_remaining,
            right_remaining,
        }
    }
}

fn p1(input: &str) -> String {
    View::combine_all_views(&View::parse_input(input), 12)
        .beacons
        .len()
        .to_string()
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

    const EXAMPLE_2D_INPUT: &str = r"
--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0
";
    const SAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn test_sin() {
        assert_eq!(sin(0), (0f64).sin() as i32);
        assert_eq!(sin(90), (std::f64::consts::PI / 2f64).sin() as i32);
        assert_eq!(sin(180), std::f64::consts::PI.sin() as i32);
        assert_eq!(sin(270), (std::f64::consts::PI * 1.5f64).sin() as i32);
    }

    #[test]
    fn test_cos() {
        assert_eq!(cos(0), (0f64).cos() as i32);
        assert_eq!(cos(90), (std::f64::consts::PI / 2f64).cos() as i32);
        assert_eq!(cos(180), std::f64::consts::PI.cos() as i32);
        assert_eq!(cos(270), (std::f64::consts::PI * 1.5f64).cos() as i32);
    }

    #[test]
    fn test_vec3_new_from_array() {
        assert_eq!(Vec3::new_from_array(&[1, 2, 3]), Vec3 { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn test_vec3_parse_line() {
        assert_eq!(Vec3::parse_line("1,2,3"), Vec3 { x: 1, y: 2, z: 3 });
        assert_eq!(
            Vec3::parse_line("-1,-2,-3"),
            Vec3 {
                x: -1,
                y: -2,
                z: -3
            }
        );
    }

    #[test]
    fn test_vec3_new() {
        assert_eq!(Vec3::new(1, 2, 3), Vec3 { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn test_vec3_rotate_x() {
        assert_eq!(Vec3::new(1, 0, 0).rotate_x(0), Vec3::new(1, 0, 0));
        assert_eq!(Vec3::new(1, 0, 0).rotate_x(90), Vec3::new(1, 0, 0));
        assert_eq!(Vec3::new(1, 0, 0).rotate_x(180), Vec3::new(1, 0, 0));
        assert_eq!(Vec3::new(1, 0, 0).rotate_x(270), Vec3::new(1, 0, 0));

        assert_eq!(Vec3::new(0, 1, 0).rotate_x(0), Vec3::new(0, 1, 0));
        assert_eq!(Vec3::new(0, 1, 0).rotate_x(90), Vec3::new(0, 0, 1));
        assert_eq!(Vec3::new(0, 1, 0).rotate_x(180), Vec3::new(0, -1, 0));
        assert_eq!(Vec3::new(0, 1, 0).rotate_x(270), Vec3::new(0, 0, -1));

        assert_eq!(Vec3::new(0, 0, 1).rotate_x(0), Vec3::new(0, 0, 1));
        assert_eq!(Vec3::new(0, 0, 1).rotate_x(90), Vec3::new(0, -1, 0));
        assert_eq!(Vec3::new(0, 0, 1).rotate_x(180), Vec3::new(0, 0, -1));
        assert_eq!(Vec3::new(0, 0, 1).rotate_x(270), Vec3::new(0, 1, 0));
    }

    #[test]
    fn test_vec3_rotate_y() {
        assert_eq!(Vec3::new(1, 0, 0).rotate_y(0), Vec3::new(1, 0, 0));
        assert_eq!(Vec3::new(1, 0, 0).rotate_y(90), Vec3::new(0, 0, -1));
        assert_eq!(Vec3::new(1, 0, 0).rotate_y(180), Vec3::new(-1, 0, 0));
        assert_eq!(Vec3::new(1, 0, 0).rotate_y(270), Vec3::new(0, 0, 1));

        assert_eq!(Vec3::new(0, 1, 0).rotate_y(0), Vec3::new(0, 1, 0));
        assert_eq!(Vec3::new(0, 1, 0).rotate_y(90), Vec3::new(0, 1, 0));
        assert_eq!(Vec3::new(0, 1, 0).rotate_y(180), Vec3::new(0, 1, 0));
        assert_eq!(Vec3::new(0, 1, 0).rotate_y(270), Vec3::new(0, 1, 0));

        assert_eq!(Vec3::new(0, 0, 1).rotate_y(0), Vec3::new(0, 0, 1));
        assert_eq!(Vec3::new(0, 0, 1).rotate_y(90), Vec3::new(1, 0, 0));
        assert_eq!(Vec3::new(0, 0, 1).rotate_y(180), Vec3::new(0, 0, -1));
        assert_eq!(Vec3::new(0, 0, 1).rotate_y(270), Vec3::new(-1, 0, 0));
    }

    #[test]
    fn test_vec3_rotate_z() {
        assert_eq!(Vec3::new(1, 0, 0).rotate_z(0), Vec3::new(1, 0, 0));
        assert_eq!(Vec3::new(1, 0, 0).rotate_z(90), Vec3::new(0, 1, 0));
        assert_eq!(Vec3::new(1, 0, 0).rotate_z(180), Vec3::new(-1, 0, 0));
        assert_eq!(Vec3::new(1, 0, 0).rotate_z(270), Vec3::new(0, -1, 0));

        assert_eq!(Vec3::new(0, 1, 0).rotate_z(0), Vec3::new(0, 1, 0));
        assert_eq!(Vec3::new(0, 1, 0).rotate_z(90), Vec3::new(-1, 0, 0));
        assert_eq!(Vec3::new(0, 1, 0).rotate_z(180), Vec3::new(0, -1, 0));
        assert_eq!(Vec3::new(0, 1, 0).rotate_z(270), Vec3::new(1, 0, 0));

        assert_eq!(Vec3::new(0, 0, 1).rotate_z(0), Vec3::new(0, 0, 1));
        assert_eq!(Vec3::new(0, 0, 1).rotate_z(90), Vec3::new(0, 0, 1));
        assert_eq!(Vec3::new(0, 0, 1).rotate_z(180), Vec3::new(0, 0, 1));
        assert_eq!(Vec3::new(0, 0, 1).rotate_z(270), Vec3::new(0, 0, 1));
    }

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(1000, 100, 10) + Vec3::new(1, 2, 3),
            Vec3::new(1001, 102, 13)
        );
    }

    #[test]
    fn test_vec3_sub() {
        assert_eq!(
            Vec3::new(1000, 100, 10) - Vec3::new(1, 2, 3),
            Vec3::new(999, 98, 7)
        );
    }

    #[test]
    fn test_vec3_orient_iterator_next() {
        let mut right_iter = Vec3::new(1, 0, 0).orient_iter();

        assert_eq!(right_iter.next(), Some(Vec3::new(1, 0, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, -1, 0)));

        assert_eq!(right_iter.next(), Some(Vec3::new(1, 0, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, -1, 0)));

        assert_eq!(right_iter.next(), Some(Vec3::new(1, 0, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, -1, 0)));

        assert_eq!(right_iter.next(), Some(Vec3::new(1, 0, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, -1, 0)));

        assert_eq!(right_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 0, -1)));

        assert_eq!(right_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(right_iter.next(), Some(Vec3::new(0, 0, 1)));

        assert_eq!(right_iter.next(), None);

        let mut up_iter = Vec3::new(0, 1, 0).orient_iter();

        assert_eq!(up_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, -1, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(1, 0, 0)));

        assert_eq!(up_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, 0, 1)));

        assert_eq!(up_iter.next(), Some(Vec3::new(0, -1, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(1, 0, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(-1, 0, 0)));

        assert_eq!(up_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, 0, -1)));

        assert_eq!(up_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, -1, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(1, 0, 0)));

        assert_eq!(up_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(0, -1, 0)));
        assert_eq!(up_iter.next(), Some(Vec3::new(1, 0, 0)));

        assert_eq!(up_iter.next(), None);

        let mut front_iter = Vec3::new(0, 0, 1).orient_iter();

        assert_eq!(front_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 0, 1)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 0, 1)));

        assert_eq!(front_iter.next(), Some(Vec3::new(0, -1, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(1, 0, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(-1, 0, 0)));

        assert_eq!(front_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 0, -1)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 0, -1)));

        assert_eq!(front_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, -1, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(1, 0, 0)));

        assert_eq!(front_iter.next(), Some(Vec3::new(1, 0, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 1, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, -1, 0)));

        assert_eq!(front_iter.next(), Some(Vec3::new(-1, 0, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, -1, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(1, 0, 0)));
        assert_eq!(front_iter.next(), Some(Vec3::new(0, 1, 0)));

        assert_eq!(front_iter.next(), None);
    }

    #[test]
    fn test_get_deltas() {
        assert_eq!(
            get_deltas(
                &[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)],
                &Vec3::new(1, 2, 3)
            ),
            [Vec3::new(3, 3, 3), Vec3::new(6, 6, 6)]
        );

        assert_eq!(
            get_deltas(
                &[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)],
                &Vec3::new(4, 5, 6)
            ),
            [Vec3::new(-3, -3, -3), Vec3::new(3, 3, 3)]
        );

        assert_eq!(
            get_deltas(
                &[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)],
                &Vec3::new(7, 8, 9)
            ),
            [Vec3::new(-6, -6, -6), Vec3::new(-3, -3, -3)]
        );
    }

    #[test]
    fn test_vec3_collection_orient_iterator_next() {
        let mut iter = Vec3CollectionOrientIterator::get_orient_iters(&[
            Vec3::new(1, 0, 0),
            Vec3::new(0, 1, 0),
            Vec3::new(0, 0, 1),
        ]);

        Vec3::new(1, 0, 0)
            .orient_iter()
            .zip(Vec3::new(0, 1, 0).orient_iter())
            .zip(Vec3::new(0, 0, 1).orient_iter())
            .map(|((a, b), c)| (a, b, c))
            .enumerate()
            .for_each(|(i, (a, b, c))| {
                assert_eq!(iter.next(), Some((vec![a, b, c], i)));
            });

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_get_equivalent_count() {
        assert_eq!(
            get_equivalent_count(&[Vec3::new(0, 0, 0)], &[Vec3::new(1, 1, 1)]),
            0
        );

        assert_eq!(
            get_equivalent_count(
                &[Vec3::new(0, 0, 0), Vec3::new(1, 1, 1)],
                &[Vec3::new(1, 1, 1)]
            ),
            1
        );

        assert_eq!(
            get_equivalent_count(
                &[Vec3::new(1, 1, 1)],
                &[Vec3::new(0, 0, 0), Vec3::new(1, 1, 1)],
            ),
            1
        );

        assert_eq!(
            get_equivalent_count(
                &[Vec3::new(0, 0, 0), Vec3::new(1, 1, 1)],
                &[Vec3::new(1, 1, 1), Vec3::new(2, 2, 2)]
            ),
            1
        );

        assert_eq!(
            get_equivalent_count(
                &[Vec3::new(0, 0, 0), Vec3::new(2, 2, 2), Vec3::new(1, 1, 1)],
                &[Vec3::new(1, 1, 1), Vec3::new(2, 2, 2), Vec3::new(4, 4, 4)]
            ),
            2
        );

        // repeats are allowed
        assert_eq!(
            get_equivalent_count(
                &[Vec3::new(0, 0, 0), Vec3::new(0, 0, 0)],
                &[Vec3::new(0, 0, 0), Vec3::new(0, 0, 0)]
            ),
            2
        );
    }

    #[test]
    fn test_view_partial_eq() {
        assert_eq!(View { beacons: vec![] }, View { beacons: vec![] },);
        assert_eq!(
            View {
                beacons: vec![Vec3::new(1, 2, 3)]
            },
            View {
                beacons: vec![Vec3::new(1, 2, 3)]
            },
        );
        // order doesn't matter
        assert_eq!(
            View {
                beacons: vec![Vec3::new(1, 2, 3), Vec3::new(4, 5, 6)]
            },
            View {
                beacons: vec![Vec3::new(4, 5, 6), Vec3::new(1, 2, 3)]
            },
        );
        // duplication doesn't matter
        assert_eq!(
            View {
                beacons: vec![Vec3::new(1, 2, 3), Vec3::new(1, 2, 3)]
            },
            View {
                beacons: vec![Vec3::new(1, 2, 3), Vec3::new(1, 2, 3)]
            },
        );
        assert_ne!(
            View {
                beacons: vec![Vec3::new(1, 2, 3)]
            },
            View {
                beacons: vec![Vec3::new(0, 0, 0)]
            },
        );
        // count does matter
        assert_ne!(
            View {
                beacons: vec![Vec3::new(1, 2, 3)]
            },
            View {
                beacons: vec![Vec3::new(1, 2, 3), Vec3::new(1, 2, 3)]
            },
        );
    }

    #[test]
    fn test_view_parse_input() {
        assert_eq!(
            View::parse_input(EXAMPLE_2D_INPUT),
            vec![
                View {
                    beacons: vec![Vec3::new(0, 2, 0), Vec3::new(4, 1, 0), Vec3::new(3, 3, 0),]
                },
                View {
                    beacons: vec![
                        Vec3::new(-1, -1, 0),
                        Vec3::new(-5, 0, 0),
                        Vec3::new(-2, 1, 0),
                    ]
                }
            ]
        );
    }

    #[test]
    fn test_view_combine_both_views() {
        // equivalent
        let views = View::parse_input(
            r"
--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
0,2,0
4,1,0
3,3,0
        ",
        );
        assert_eq!(
            views[0].combine_both_views(&views[1], 3),
            Some(views[0].clone())
        );

        // equivalent but different order
        let views = View::parse_input(
            r"
--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
3,3,0
0,2,0
4,1,0
        ",
        );
        assert_eq!(
            views[0].combine_both_views(&views[1], 3),
            Some(views[0].clone())
        );

        // same deltas
        let views = View::parse_input(
            r"
--- scanner 0 ---
0,0,0
1,1,1
2,2,2

--- scanner 1 ---
0,0,0
1,1,1
2,2,2
        ",
        );
        assert_eq!(
            views[0].combine_both_views(&views[1], 3),
            Some(views[0].clone())
        );

        // try different rotations
        let views = View::parse_input(
            r"
--- scanner 0 ---
0,0,0
1,1,0
1,0,0
0,1,0

--- scanner 1 ---
0,0,0
0,1,-1
0,1,0
0,0,-1
        ",
        );
        assert_eq!(
            views[0].combine_both_views(&views[1], 3),
            Some(views[0].clone())
        );

        // remember to translate
        let views = View::parse_input(
            r"
--- scanner 0 ---
0,0,0
1,1,0
1,0,0
0,1,0

--- scanner 1 ---
3,5,-3
3,6,-4
3,6,-3
3,6,-4
        ",
        );
        assert_eq!(
            views[0].combine_both_views(&views[1], 3),
            Some(views[0].clone())
        );

        // extras are not forgetten
        let views = View::parse_input(
            r"
--- scanner 0 ---
0,0,0
1,1,0
1,0,0
0,1,0
12,12,12

--- scanner 1 ---
3,3,-3
3,4,-4
3,4,-3
3,4,-4
15,-9,9
        ",
        );
        assert_eq!(
            views[0].combine_both_views(&views[1], 3),
            Some(View {
                beacons: views[0]
                    .beacons
                    .iter()
                    .copied()
                    .chain([Vec3::new(-12, -12, -12)])
                    .collect()
            })
        );

        // example from question
        let views = View::parse_input(EXAMPLE_2D_INPUT);
        assert_eq!(
            views[0].combine_both_views(&views[1], 3),
            Some(views[0].clone())
        );
    }

    #[test]
    fn test_view_combine_both_views_2() {
        // another example from question, with different orientations
        let views = View::parse_input(
            r"
--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8
",
        );

        fn assert_view_eq(left: &View, right: &View) {
            assert_eq!(left, right, "Difference:\n{:?}", left.debug_get_diff(right));
        }

        assert_view_eq(
            &views[0].combine_both_views(&views[1], 6).unwrap(),
            &views[0],
        );
        assert_view_eq(
            &views[0].combine_both_views(&views[2], 6).unwrap(),
            &views[0],
        );
        assert_view_eq(
            &views[0].combine_both_views(&views[3], 6).unwrap(),
            &views[0],
        );
        assert_view_eq(
            &views[0].combine_both_views(&views[4], 6).unwrap(),
            &views[0],
        );
    }

    #[test]
    fn test_view_combine_all_views() {
        // input should have unique deltas, otherwise
        // each run may produce different results
        let views = View::parse_input(
            r"
--- scanner 0 ---
0,0,0
1,1,0
15,15,0
26,26,0

--- scanner 1 ---
0,0,0
1,1,0
15,15,0
317,317,0

--- scanner 2 ---
0,0,0
1,1,0
15,15,0
317,317,0
485,485,0

--- scanner 3 ---
1,1,0
15,15,0
26,26,0
317,317,0
485,485,0
",
        );

        assert_eq!(
            View::combine_all_views(&views, 4),
            View {
                beacons: vec![
                    Vec3::new(0, 0, 0),
                    Vec3::new(1, 1, 0),
                    Vec3::new(15, 15, 0),
                    Vec3::new(26, 26, 0),
                    Vec3::new(317, 317, 0),
                    Vec3::new(485, 485, 0),
                ]
            }
        );
    }

    #[test]
    fn test_view_debug_get_diff() {
        assert_eq!(
            View {
                beacons: vec![Vec3::new(0, 0, 0)]
            }
            .debug_get_diff(&View {
                beacons: vec![Vec3::new(1, 1, 1)]
            }),
            ViewDiff {
                common: vec![],
                left_remaining: vec![Vec3::new(0, 0, 0)],
                right_remaining: vec![Vec3::new(1, 1, 1)]
            }
        );

        assert_eq!(
            View {
                beacons: vec![Vec3::new(1, 1, 1)]
            }
            .debug_get_diff(&View {
                beacons: vec![Vec3::new(0, 0, 0)]
            }),
            ViewDiff {
                common: vec![],
                left_remaining: vec![Vec3::new(1, 1, 1)],
                right_remaining: vec![Vec3::new(0, 0, 0)],
            }
        );

        assert_eq!(
            View {
                beacons: vec![Vec3::new(1, 1, 1), Vec3::new(0, 0, 0)]
            }
            .debug_get_diff(&View {
                beacons: vec![Vec3::new(0, 0, 0)]
            }),
            ViewDiff {
                common: vec![Vec3::new(0, 0, 0)],
                left_remaining: vec![Vec3::new(1, 1, 1)],
                right_remaining: vec![],
            }
        );

        assert_eq!(
            View {
                beacons: vec![Vec3::new(0, 0, 0)]
            }
            .debug_get_diff(&View {
                beacons: vec![Vec3::new(1, 1, 1), Vec3::new(0, 0, 0)]
            }),
            ViewDiff {
                common: vec![Vec3::new(0, 0, 0)],
                left_remaining: vec![],
                right_remaining: vec![Vec3::new(1, 1, 1)],
            }
        );

        assert_eq!(
            View {
                beacons: vec![Vec3::new(0, 0, 0), Vec3::new(2, 2, 2), Vec3::new(1, 1, 1)]
            }
            .debug_get_diff(&View {
                beacons: vec![Vec3::new(1, 1, 1), Vec3::new(2, 2, 2), Vec3::new(4, 4, 4)]
            }),
            ViewDiff {
                common: vec![Vec3::new(1, 1, 1), Vec3::new(2, 2, 2)],
                left_remaining: vec![Vec3::new(0, 0, 0)],
                right_remaining: vec![Vec3::new(4, 4, 4)],
            }
        );

        // repeats are allowed
        assert_eq!(
            View {
                beacons: vec![Vec3::new(0, 0, 0), Vec3::new(0, 0, 0), Vec3::new(1, 1, 1)]
            }
            .debug_get_diff(&View {
                beacons: vec![Vec3::new(0, 0, 0), Vec3::new(0, 0, 0), Vec3::new(0, 0, 0)]
            }),
            ViewDiff {
                common: vec![Vec3::new(0, 0, 0), Vec3::new(0, 0, 0)],
                left_remaining: vec![Vec3::new(1, 1, 1)],
                right_remaining: vec![Vec3::new(0, 0, 0)],
            }
        );
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "79");
    }

    #[test]
    #[ignore = "expensive to run"]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "390");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
