/// Solution to an Advent of Code problem, day 19, 2021
/// https://adventofcode.com/2021/day/19
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
struct Scanner {
    beacons: Vec<(i64, i64, i64)>,
    beacons_rotated: [Vec<(i64, i64, i64)>; 24],
    //axis_signatures: [AxisSignatures; 24]
}

/// combination: 0..24
/// ( x, y, z)
/// ( x,-y,-z)
/// ( x, z,-y)
/// ( x,-z, y)
///
/// ( y, x,-z)
/// ( y,-x, z)
/// ( y, z, x)
/// ( y,-z,-x)
///
/// ( z, x, y)
/// ( z,-x,-y)
/// ( z, y,-x)
/// ( z,-y, x)
///
/// (-x, y,-z)
/// (-x,-y, z)
/// (-x, z, y)
/// (-x,-z,-y)
///
/// (-y, x, z)
/// (-y,-x,-z)
/// (-y, z,-x)
/// (-y,-z, x)
///
/// (-z, x,-y)
/// (-z,-x, y)
/// (-z, y, x)
/// (-z,-y,-x)
fn rotate_point((x, y, z): (i64, i64, i64), combination: u8) -> (i64, i64, i64) {
    match combination {
        0 => (x, y, z),
        1 => (x, -y, -z),
        2 => (x, z, -y),
        3 => (x, -z, y),
        4 => (y, x, -z),
        5 => (y, -x, z),
        6 => (y, z, x),
        7 => (y, -z, -x),
        8 => (z, x, y),
        9 => (z, -x, -y),
        10 => (z, y, -x),
        11 => (z, -y, x),
        12 => (-x, y, -z),
        13 => (-x, -y, z),
        14 => (-x, z, y),
        15 => (-x, -z, -y),
        16 => (-y, x, z),
        17 => (-y, -x, -z),
        18 => (-y, z, -x),
        19 => (-y, -z, x),
        20 => (-z, x, -y),
        21 => (-z, -x, y),
        22 => (-z, y, x),
        23 => (-z, -y, -x),

        _ => panic!("Invalid rotation"),
    }
}

fn translate_point((x, y, z): (i64, i64, i64), dx: i64, dy: i64, dz: i64) -> (i64, i64, i64) {
    (x + dx, y + dy, z + dz)
}

impl Scanner {
    fn new(mut beacons: Vec<(i64, i64, i64)>) -> Scanner {
        let beacons_rotated: [Vec<(i64, i64, i64)>; 24] = (0..24)
            .map(|combination| {
                let mut rotated_beacons = beacons
                    .iter()
                    .map(|beacon| rotate_point(*beacon, combination))
                    .collect::<Vec<(i64, i64, i64)>>();
                rotated_beacons.sort_unstable();
                rotated_beacons
            })
            .collect::<Vec<Vec<(i64, i64, i64)>>>()
            .try_into()
            .unwrap();

        beacons.sort_unstable();

        Scanner {
            beacons,
            beacons_rotated,
        }
    }
}
/*
#[derive(Debug)]
struct AxisSignature(Vec<i64>);

#[derive(Debug)]
struct AxisSignatures {
    x: AxisSignature,
    y: AxisSignature,
    z: AxisSignature,
}

impl AxisSignatures {
    fn from_beacons(beacons: &[(i64, i64, i64)]) -> AxisSignatures {
        let mut x = Vec::with_capacity(beacons.len());
        let mut y = Vec::with_capacity(beacons.len());
        let mut z = Vec::with_capacity(beacons.len());

        for beacon in beacons {
            x.push(beacon.0);
            y.push(beacon.1);
            z.push(beacon.2);
        }

        x.sort_unstable();
        y.sort_unstable();
        z.sort_unstable();

        AxisSignatures {
            x: AxisSignature(x),
            y: AxisSignature(y),
            z: AxisSignature(z),
        }
    }
}
*/
fn histogram_from_sorted(a: &[i64]) -> Vec<(i64, u64)> {
    if a.is_empty() {
        return vec![];
    }

    let mut histogram = vec![];
    let mut prev = a[0];
    let mut count = 0;
    for &a_ in a {
        if a_ != prev {
            histogram.push((prev, count));
            count = 0;
        }
        prev = a_;
        count += 1;
    }
    if count > 0 {
        histogram.push((prev, count));
    }
    histogram
}

fn find_overlap_candidates(a: &[(i64, i64, i64)], b: &[(i64, i64, i64)], axis: u8) -> Vec<i64> {
    let mut deltas = vec![];

    match axis {
        0 => {
            for a_ in a {
                for b_ in b {
                    deltas.push(b_.0 - a_.0);
                }
            }
        }
        1 => {
            for a_ in a {
                for b_ in b {
                    deltas.push(b_.1 - a_.1);
                }
            }
        }
        2 => {
            for a_ in a {
                for b_ in b {
                    deltas.push(b_.2 - a_.2);
                }
            }
        }
        _ => panic!("Axis must be one of 0, 1, 2"),
    }

    deltas.sort_unstable();

    histogram_from_sorted(&deltas)
        .iter()
        .filter_map(|&(val, freq)| if freq >= 12 { Some(val) } else { None })
        .collect()
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = vec![];
    let mut beacons = vec![];
    for line in input.lines() {
        if line.is_empty() {
            scanners.push(Scanner::new(beacons));
            beacons = vec![];
        } else if line.contains("scanner") {
        } else {
            let mut xyz = line.split(',').map(|v| v.parse::<i64>().unwrap());
            beacons.push((
                xyz.next().unwrap(),
                xyz.next().unwrap(),
                xyz.next().unwrap(),
            ));
        }
    }
    if !beacons.is_empty() {
        scanners.push(Scanner::new(beacons));
    }
    scanners
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut scanners = parse_input(&contents);
    //println!("{:?}", scanners);

    let mut beacon_positions_from_resolved_scanners = vec![scanners.remove(0).beacons];
    let mut scanner_positions = vec![(0, 0, 0)];

    while !scanners.is_empty() {
        'match_scanner: for i in 0..scanners.len() {
            let scanner = &scanners[i];
            for orientation in 0..24 {
                let scanner_beacons = &scanner.beacons_rotated[orientation];
                for resolved_beacon_positions in &beacon_positions_from_resolved_scanners {
                    let x_deltas =
                        find_overlap_candidates(scanner_beacons, resolved_beacon_positions, 0);
                    let y_deltas =
                        find_overlap_candidates(scanner_beacons, resolved_beacon_positions, 1);
                    let z_deltas =
                        find_overlap_candidates(scanner_beacons, resolved_beacon_positions, 2);
                    for &x in &x_deltas {
                        for &y in &y_deltas {
                            for &z in &z_deltas {
                                let resolved_beacons = scanner_beacons
                                    .iter()
                                    .map(|p| translate_point(*p, x, y, z))
                                    .collect::<Vec<(i64, i64, i64)>>();

                                let match_count = resolved_beacons
                                    .iter()
                                    .filter(|b| resolved_beacon_positions.binary_search(b).is_ok())
                                    .count();

                                if match_count >= 12 {
                                    scanners.remove(i);
                                    scanner_positions.push((x, y, z));
                                    beacon_positions_from_resolved_scanners.push(resolved_beacons);
                                    break 'match_scanner;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut all_beacons = beacon_positions_from_resolved_scanners.concat();
    all_beacons.sort_unstable();
    all_beacons.dedup();
    println!("[part 1] Beacon count: {}", all_beacons.len());


    let mut max_manhattan_distance = 0;
    for a in &scanner_positions {
        for b in &scanner_positions {
            max_manhattan_distance = std::cmp::max(
                max_manhattan_distance,
                (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs(),
            );
        }
    }
    println!(
        "[part 2] Max Manhattan distance between any two scanners: {}",
        max_manhattan_distance
    );
}

/* Orientations */
// ( x, y, z)
// ( x,-y,-z)
// ( x, z,-y)
// ( x,-z, y)
// ( y, x,-z)
// ( y,-x, z)
// ( y, z, x)
// ( y,-z,-x)
// ( z, x, y)
// ( z,-x,-y)
// ( z, y,-x)
// ( z,-y, x)
// (-x, y,-z)
// (-x,-y, z)
// (-x, z, y)
// (-x,-z,-y)
// (-y, x, z)
// (-y,-x,-z)
// (-y, z,-x)
// (-y,-z, x)
// (-z, x,-y)
// (-z,-x, y)
// (-z, y, x)
// (-z,-y,-x)
