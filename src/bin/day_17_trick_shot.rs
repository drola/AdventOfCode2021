/// Solution to an Advent of Code problem, day 17, 2021
/// https://adventofcode.com/2021/day/17
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
struct TargetArea {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

#[derive(Debug)]
struct ProbeState {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

fn step(initial_probe_state: ProbeState) -> ProbeState {
    ProbeState {
        x: initial_probe_state.x + initial_probe_state.vx,
        y: initial_probe_state.y + initial_probe_state.vy,
        vx: match initial_probe_state.vx {
            vx if vx > 0 => vx - 1,
            vx if vx < 0 => vx + 1,
            _ => 0,
        },
        vy: initial_probe_state.vy - 1,
    }
}

fn is_in_target_area(target_area: &TargetArea, probe_state: &ProbeState) -> bool {
    target_area.x1 <= probe_state.x
        && probe_state.x <= target_area.x2
        && target_area.y1 <= probe_state.y
        && probe_state.y <= target_area.y2
}

fn try_find_max_y_if_hit(target_area: &TargetArea, initial_vxvy: (i64, i64)) -> Option<i64> {
    let mut max_y = 0;
    let mut probe_state = ProbeState {
        x: 0,
        y: 0,
        vx: initial_vxvy.0,
        vy: initial_vxvy.1,
    };

    // when probe is falling and below target area, hope is lost
    while !(probe_state.vy < 0 && probe_state.y < target_area.y1) {
        if is_in_target_area(target_area, &probe_state) {
            return Some(max_y);
        }

        probe_state = step(probe_state);
        max_y = std::cmp::max(max_y, probe_state.y);
    }

    None
}

/// example:
/// target area: x=20..30, y=-10..-5
fn parse_target_area(s: &str) -> Option<TargetArea> {
    let (_, ranges) = s.split_once(": ")?;
    let (xx_range, yy_range) = ranges.split_once(", ")?;
    let (_, x_range) = xx_range.split_once('=')?;
    let (_, y_range) = yy_range.split_once('=')?;
    let (x1, x2) = x_range.split_once("..")?;
    let (y1, y2) = y_range.split_once("..")?;
    Some(TargetArea {
        x1: x1.parse().ok()?,
        x2: x2.parse().ok()?,
        y1: y1.parse().ok()?,
        y2: y2.parse().ok()?,
    })
}

fn initial_velocities(target_area: &TargetArea) -> impl std::iter::Iterator<Item = (i64, i64)> {
    // lower vx bound: In test data, the target area is always in the positive x direction
    // upper vx bound: If vx > target_area.x2, probe will go beyond target area in first step
    // lower vy bound: If vy < target_area.y1, probe will go below target area in first step
    // upper vy bound: For positive initial vy, the probe will always come again to position y=0 with vy=-initial vy.
    //                 For initial vy > -target_area.y1, the probe will go below target area in one step after that.
    let x_range = 0..target_area.x2 + 1;
    let y_range = target_area.y1..-target_area.y1;
    x_range
        .into_iter()
        .flat_map(move |x| y_range.clone().map(move |y| (x, y)))
}

fn optimize_initial_vx_vy_for_max_y(target_area: &TargetArea) -> i64 {
    initial_velocities(target_area)
        .filter_map(|initial_vxvy| try_find_max_y_if_hit(target_area, initial_vxvy))
        .max()
        .unwrap()
}

fn count_distinct_vx_vy_that_hit(target_area: &TargetArea) -> usize {
    initial_velocities(target_area)
        .filter_map(|initial_vxvy| try_find_max_y_if_hit(target_area, initial_vxvy))
        .count()
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    for line in contents.lines() {
        let target_area = parse_target_area(line).ok_or(())?;
        println!(
            "[part 1]: Max height reached, while hitting the target = {}",
            optimize_initial_vx_vy_for_max_y(&target_area)
        );
        println!(
            "[part 2]: Count of distinct initial (vx, vy), that hit the target = {}",
            count_distinct_vx_vy_that_hit(&target_area)
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_target_area() {
        assert_eq!(
            parse_target_area("target area: x=20..30, y=-10..-5").unwrap(),
            TargetArea {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5
            }
        );
    }

    #[test]
    fn test_simulate() {
        assert_eq!(
            try_find_max_y_if_hit(
                &TargetArea {
                    x1: 20,
                    x2: 30,
                    y1: -10,
                    y2: -5
                },
                (7, 2)
            ),
            Some(3)
        );
        assert_eq!(
            try_find_max_y_if_hit(
                &TargetArea {
                    x1: 20,
                    x2: 30,
                    y1: -10,
                    y2: -5
                },
                (17, -4)
            ),
            None
        );
        assert_eq!(
            try_find_max_y_if_hit(
                &TargetArea {
                    x1: 20,
                    x2: 30,
                    y1: -10,
                    y2: -5
                },
                (6, 9)
            ),
            Some(45)
        );
    }

    #[test]
    fn test_optimize_initial_vx_vy_for_max_y() {
        assert_eq!(
            optimize_initial_vx_vy_for_max_y(&TargetArea {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5
            }),
            45
        )
    }
    #[test]
    fn test_count_distinct_vx_vy_that_hit() {
        assert_eq!(
            count_distinct_vx_vy_that_hit(&TargetArea {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5
            }),
            112
        )
    }
}
