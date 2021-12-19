use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn from_str(str: &str) -> Self {
        let mut iter = str.trim().split(',');

        Self {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
            z: iter.next().unwrap().parse().unwrap(),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn sin_cos(angle: i32) -> (i32, i32) {
        match angle {
            0 => (0, 1),
            90 => (1, 0),
            180 => (0, -1),
            270 => (-1, 0),
            _ => panic!("Cannot use non-90-degrees angles, given: {}", angle),
        }
    }

    fn rot_x(&self, angle: i32) -> Self {
        let (sin, cos) = Self::sin_cos(angle);
        Self {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }

    fn rot_y(&self, angle: i32) -> Self {
        let (sin, cos) = Self::sin_cos(angle);
        Self {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }

    fn rot_z(&self, angle: i32) -> Self {
        let (sin, cos) = Self::sin_cos(angle);
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
    }
}

fn get_deltas(source: &Vec3, list: &HashSet<Vec3>) -> HashSet<Vec3> {
    list.iter()
        .filter(|&p| p != source)
        .map(|p| p.sub(source))
        .collect()
}

struct Vec3OrientationIterator<'a> {
    list: &'a HashSet<Vec3>,
    current_orient: i32,
}

fn get_all_orientations(list: &HashSet<Vec3>) -> Vec3OrientationIterator {
    Vec3OrientationIterator {
        list,
        current_orient: 0,
    }
}

impl<'a> Iterator for Vec3OrientationIterator<'a> {
    type Item = HashSet<Vec3>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_orient >= 24 {
            None
        } else {
            let result = Some(
                self.list
                    .iter()
                    .map(|p| {
                        let rot_count = self.current_orient % 4 * 90;

                        match self.current_orient / 4 {
                            0 => p.rot_x(rot_count),            // rot = +ve x
                            1 => p.rot_z(180).rot_x(rot_count), // rot = -ve x
                            2 => p.rot_y(rot_count),            // rot = +ve y
                            3 => p.rot_x(180).rot_y(rot_count), // rot = -ve y
                            4 => p.rot_z(rot_count),            // rot = +ve z
                            5 => p.rot_y(180).rot_z(rot_count), // rot = -ve z
                            _ => unreachable!(),
                        }
                    })
                    .collect(),
            );

            self.current_orient += 1;

            result
        }
    }
}

fn adjust_points_to_basis(
    basis_points: &HashSet<Vec3>,
    points: &HashSet<Vec3>,
    tolerance: usize,
) -> Option<HashSet<Vec3>> {
    for basis_point in basis_points.iter() {
        let basis_deltas = get_deltas(basis_point, basis_points);

        for points_oriented in get_all_orientations(points) {
            for selected_point in points_oriented.iter() {
                let points_deltas = get_deltas(selected_point, points);

                if points_deltas.intersection(&basis_deltas).count() >= tolerance - 1 {
                    let diff_to_basis = selected_point.sub(basis_point);

                    return Some(
                        points
                            .iter()
                            .map(|p| p.sub(&diff_to_basis))
                            .collect::<HashSet<_>>(),
                    );
                }
            }
        }
    }

    None
}

fn p1(input: &str) -> String {
    let mut scanners = input
        .trim()
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(Vec3::from_str)
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    let mut known_beacons = scanners.swap_remove(0);

    while !scanners.is_empty() {
        let mut found = None;

        for (i, scanner) in scanners.iter().enumerate() {
            let new_points = adjust_points_to_basis(&known_beacons, scanner, 12);

            if let Some(new_points) = new_points {
                found = Some((i, new_points));
                break;
            }
        }

        let (scanner_index, new_beacons) = found.unwrap();
        known_beacons.extend(new_beacons);
        scanners.swap_remove(scanner_index);
    }

    known_beacons.len().to_string()
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
    fn test_vec3_from_str() {
        assert_eq!(Vec3::from_str("-1,2,3"), Vec3 { x: -1, y: 2, z: 3 });
    }

    #[test]
    fn test_vec3_sub() {
        assert_eq!(
            Vec3 {
                x: 100,
                y: 2000,
                z: 30000
            }
            .sub(&Vec3 { x: 8, y: 12, z: 17 }),
            Vec3 {
                x: 92,
                y: 1988,
                z: 29983
            }
        );
    }

    #[test]
    fn test_vec3_rot_x() {
        assert_eq!(
            Vec3 { x: 0, y: 1, z: 0 }.rot_x(0),
            Vec3 { x: 0, y: 1, z: 0 }
        );
        assert_eq!(
            Vec3 { x: 0, y: 1, z: 0 }.rot_x(90),
            Vec3 { x: 0, y: 0, z: 1 }
        );
        assert_eq!(
            Vec3 { x: 0, y: 1, z: 0 }.rot_x(180),
            Vec3 { x: 0, y: -1, z: 0 }
        );
        assert_eq!(
            Vec3 { x: 0, y: 1, z: 0 }.rot_x(270),
            Vec3 { x: 0, y: 0, z: -1 }
        );
    }

    #[test]
    fn test_get_deltas() {
        assert_eq!(
            get_deltas(
                &Vec3 { x: 1, y: 1, z: 1 },
                &HashSet::from_iter([
                    Vec3 { x: 4, y: 5, z: 6 },
                    Vec3 { x: 1, y: 1, z: 1 },
                    Vec3 {
                        x: -10,
                        y: 30,
                        z: 2
                    }
                ])
            ),
            HashSet::from_iter([
                Vec3 { x: 3, y: 4, z: 5 },
                Vec3 {
                    x: -11,
                    y: 29,
                    z: 1
                }
            ])
        );
    }

    #[test]
    fn test_get_all_orientations() {
        let list = HashSet::from_iter([Vec3 { x: 1, y: 1, z: 1 }]);
        let mut iter = get_all_orientations(&list);

        // axis = +ve x
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: 1, z: 1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: -1, z: 1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: -1, z: -1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: 1, z: -1 }]))
        );

        // axis = -ve x
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: -1, z: 1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 {
                x: -1,
                y: -1,
                z: -1
            }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: 1, z: -1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: 1, z: 1 }]))
        );

        // axis = +ve y
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: 1, z: 1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: 1, z: -1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: 1, z: -1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: 1, z: 1 }]))
        );

        // axis = -ve y
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: -1, z: -1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 {
                x: -1,
                y: -1,
                z: -1
            }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: -1, z: 1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: -1, z: 1 }]))
        );

        // axis = +ve z
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: 1, z: 1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: 1, z: 1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: -1, z: 1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: -1, z: 1 }]))
        );

        // axis = -ve z
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: -1, y: 1, z: -1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 {
                x: -1,
                y: -1,
                z: -1
            }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: -1, z: -1 }]))
        );
        assert_eq!(
            iter.next(),
            Some(HashSet::from_iter([Vec3 { x: 1, y: 1, z: -1 }]))
        );

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_adjust_points_to_basis() {
        assert_eq!(
            adjust_points_to_basis(
                &HashSet::from_iter([
                    Vec3 { x: 1, y: 1, z: 1 },
                    Vec3 { x: 2, y: 2, z: 2 },
                    Vec3 { x: 5, y: 4, z: 9 }
                ]),
                &HashSet::from_iter([
                    Vec3 { x: 7, y: 7, z: 7 },
                    Vec3 { x: 6, y: 6, z: 6 },
                    Vec3 { x: 4, y: 3, z: 2 }
                ]),
                2
            ),
            Some(HashSet::from_iter([
                Vec3 { x: 1, y: 1, z: 1 },
                Vec3 { x: 2, y: 2, z: 2 },
                Vec3 {
                    x: -1,
                    y: -2,
                    z: -3
                },
            ]))
        );
    }

    const SAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "79");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "");
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
