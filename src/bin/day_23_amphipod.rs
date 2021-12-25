/// Solution to an Advent of Code problem, day 23, 2021
/// https://adventofcode.com/2021/day/23
use std::env;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

fn print_maybe_amphipod(maybe_amphipod: Option<Amphipod>) {
    match maybe_amphipod {
        Some(Amphipod::A) => print!("A"),
        Some(Amphipod::B) => print!("B"),
        Some(Amphipod::C) => print!("C"),
        Some(Amphipod::D) => print!("D"),
        _ => print!("."),
    }
}

impl TryFrom<char> for Amphipod {
    type Error = ();
    fn try_from(chr: char) -> Result<Self, Self::Error> {
        match chr {
            'A' => Ok(Amphipod::A),
            'B' => Ok(Amphipod::B),
            'C' => Ok(Amphipod::C),
            'D' => Ok(Amphipod::D),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct BurrowOccupancyV2<const N: usize> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; N]; 4],
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct BurrowOccupancy {
    hallway: [Option<Amphipod>; 11],
    room_a: [Option<Amphipod>; 2],
    room_b: [Option<Amphipod>; 2],
    room_c: [Option<Amphipod>; 2],
    room_d: [Option<Amphipod>; 2],
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct BurrowOccupancyP2 {
    hallway: [Option<Amphipod>; 11],
    room_a: [Option<Amphipod>; 4],
    room_b: [Option<Amphipod>; 4],
    room_c: [Option<Amphipod>; 4],
    room_d: [Option<Amphipod>; 4],
}

fn cost_per_step(amphipod: Amphipod) -> usize {
    match amphipod {
        Amphipod::A => 1,
        Amphipod::B => 10,
        Amphipod::C => 100,
        Amphipod::D => 1000,
    }
}

fn owned_room(amphipod: Amphipod) -> usize {
    match amphipod {
        Amphipod::A => 0,
        Amphipod::B => 1,
        Amphipod::C => 2,
        Amphipod::D => 3,
    }
}

fn room_position(room_index: usize) -> usize {
    match room_index {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => panic!("There are only 4 rooms!"),
    }
}

fn room_owner(room_index: usize) -> Amphipod {
    match room_index {
        0 => Amphipod::A,
        1 => Amphipod::B,
        2 => Amphipod::C,
        3 => Amphipod::D,
        _ => panic!("There are only 4 rooms!"),
    }
}

fn movements_from_room_to_hallway_v2<const N: usize>(
    occupancy: &BurrowOccupancyV2<N>,
    room_index: usize,
) -> [Option<usize>; 11] {
    let mut result = [None; 11];
    let position = room_position(room_index);
    if occupancy.hallway[position].is_some() {
        return result;
    }

    for i in (0..position).rev() {
        if occupancy.hallway[i].is_some() {
            break;
        }
        result[i] = Some(position - i);
    }
    for i in (position + 1)..11 {
        if occupancy.hallway[i].is_some() {
            break;
        }
        result[i] = Some(i - position);
    }
    result
}

fn movements_from_hallway_to_room_v2<const N: usize>(
    occupancy: &BurrowOccupancyV2<N>,
    room_index: usize,
) -> [Option<usize>; 11] {
    let owner = room_owner(room_index);
    let position = room_position(room_index);

    let mut result = [None; 11];
    for i in (0..position + 1).rev() {
        match occupancy.hallway[i] {
            Some(owner) => {
                result[i] = Some(position - i);
                break;
            }
            Some(_) => {
                break;
            }
            _ => {}
        }
    }
    for i in position..11 {
        match occupancy.hallway[i] {
            Some(owner) => {
                result[i] = Some(i - position);
                break;
            }
            Some(_) => {
                break;
            }
            _ => {}
        }
    }

    result
}

fn movements_along_hallway(occupancy: &BurrowOccupancy, from: usize) -> Vec<(usize, usize)> {
    let to_left = (0..from)
        .rev()
        .take_while(|&i| occupancy.hallway[i].is_none())
        .map(|i| (i, from - i));
    let to_right = (from + 1..11)
        .take_while(|&i| occupancy.hallway[i].is_none())
        .map(|i| (i, i - from));

    to_left.chain(to_right).collect::<Vec<(usize, usize)>>()
}

fn movements_from_hallway(occupancy: &BurrowOccupancy, to: usize) -> Vec<(usize, usize)> {
    let to_left = (0..to + 1)
        .rev()
        .filter(|&i| occupancy.hallway[i].is_some())
        .take(1)
        .map(|i| (i, to - i));
    let to_right = (to..11)
        .filter(|&i| occupancy.hallway[i].is_some())
        .take(1)
        .map(|i| (i, i - to));

    to_left.chain(to_right).collect::<Vec<(usize, usize)>>()
}

fn movements_along_hallway_p2(occupancy: &BurrowOccupancyP2, from: usize) -> Vec<(usize, usize)> {
    let to_left = (0..from)
        .rev()
        .take_while(|&i| occupancy.hallway[i].is_none())
        .map(|i| (i, from - i));
    let to_right = (from + 1..11)
        .take_while(|&i| occupancy.hallway[i].is_none())
        .map(|i| (i, i - from));

    to_left.chain(to_right).collect::<Vec<(usize, usize)>>()
}

fn movements_from_hallway_p2(occupancy: &BurrowOccupancyP2, to: usize) -> Vec<(usize, usize)> {
    let to_left = (0..to + 1)
        .rev()
        .filter(|&i| occupancy.hallway[i].is_some())
        .take(1)
        .map(|i| (i, to - i));
    let to_right = (to..11)
        .filter(|&i| occupancy.hallway[i].is_some())
        .take(1)
        .map(|i| (i, i - to));

    to_left.chain(to_right).collect::<Vec<(usize, usize)>>()
}

fn generate_possible_next_configurations(
    occupancy: &BurrowOccupancy,
) -> Vec<(usize, BurrowOccupancy)> {
    let mut configurations = vec![];

    let mut tmp;

    // movements out of room A
    if occupancy.hallway[2].is_none() {
        let room_position = 2;
        let hallway_destinations = movements_along_hallway(occupancy, room_position);
        match occupancy.room_a {
            [Some(Amphipod::A), Some(Amphipod::A)] => {}
            [Some(front), _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_a[0].take();
                    configurations.push((cost_per_step(front) * (1 + cost_along_hallway), tmp));
                }
            }
            [None, Some(Amphipod::A)] => {}
            [None, Some(back)] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_a[1].take();
                    configurations.push((cost_per_step(back) * (2 + cost_along_hallway), tmp));
                }
            }
            _ => {}
        }
    }

    // movements out of room B
    if occupancy.hallway[4].is_none() {
        let room_position = 4;
        let hallway_destinations = movements_along_hallway(occupancy, room_position);
        match occupancy.room_b {
            [Some(Amphipod::B), Some(Amphipod::B)] => {}
            [Some(front), _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_b[0].take();
                    configurations.push((cost_per_step(front) * (1 + cost_along_hallway), tmp));
                }
            }
            [None, Some(Amphipod::B)] => {}
            [None, Some(back)] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_b[1].take();
                    configurations.push((cost_per_step(back) * (2 + cost_along_hallway), tmp));
                }
            }
            _ => {}
        }
    }
    // movements out of room C
    if occupancy.hallway[6].is_none() {
        let room_position = 6;
        let hallway_destinations = movements_along_hallway(occupancy, room_position);
        match occupancy.room_c {
            [Some(Amphipod::C), Some(Amphipod::C)] => {}
            [Some(front), _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_c[0].take();
                    configurations.push((cost_per_step(front) * (1 + cost_along_hallway), tmp));
                }
            }
            [None, Some(Amphipod::C)] => {}
            [None, Some(back)] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_c[1].take();
                    configurations.push((cost_per_step(back) * (2 + cost_along_hallway), tmp));
                }
            }
            _ => {}
        }
    }
    // movements out of room D
    if occupancy.hallway[8].is_none() {
        let room_position = 8;
        let hallway_destinations = movements_along_hallway(occupancy, room_position);
        match occupancy.room_d {
            [Some(Amphipod::D), Some(Amphipod::D)] => {}
            [Some(front), _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_d[0].take();
                    configurations.push((cost_per_step(front) * (1 + cost_along_hallway), tmp));
                }
            }
            [None, Some(Amphipod::D)] => {}
            [None, Some(back)] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_d[1].take();
                    configurations.push((cost_per_step(back) * (2 + cost_along_hallway), tmp));
                }
            }
            _ => {}
        }
    }

    // from hallway to room A
    for (start, cost_along_hallway) in movements_from_hallway(occupancy, 2) {
        match (occupancy.hallway[start], occupancy.room_a) {
            (Some(Amphipod::A), [None, None]) => {
                tmp = *occupancy;
                tmp.room_a[1] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::A) * (2 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::A), [None, Some(Amphipod::A)]) => {
                tmp = *occupancy;
                tmp.room_a[0] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::A) * (1 + cost_along_hallway), tmp));
            }
            _ => {}
        }
    }
    // from hallway to room B
    for (start, cost_along_hallway) in movements_from_hallway(occupancy, 4) {
        match (occupancy.hallway[start], occupancy.room_b) {
            (Some(Amphipod::B), [None, None]) => {
                tmp = *occupancy;
                tmp.room_b[1] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::B) * (2 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::B), [None, Some(Amphipod::B)]) => {
                tmp = *occupancy;
                tmp.room_b[0] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::B) * (1 + cost_along_hallway), tmp));
            }
            _ => {}
        }
    }
    // from hallway to room C
    for (start, cost_along_hallway) in movements_from_hallway(occupancy, 6) {
        match (occupancy.hallway[start], occupancy.room_c) {
            (Some(Amphipod::C), [None, None]) => {
                tmp = *occupancy;
                tmp.room_c[1] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::C) * (2 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::C), [None, Some(Amphipod::C)]) => {
                tmp = *occupancy;
                tmp.room_c[0] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::C) * (1 + cost_along_hallway), tmp));
            }
            _ => {}
        }
    }
    // from hallway to room D
    for (start, cost_along_hallway) in movements_from_hallway(occupancy, 8) {
        match (occupancy.hallway[start], occupancy.room_d) {
            (Some(Amphipod::D), [None, None]) => {
                tmp = *occupancy;
                tmp.room_d[1] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::D) * (2 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::D), [None, Some(Amphipod::D)]) => {
                tmp = *occupancy;
                tmp.room_d[0] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::D) * (1 + cost_along_hallway), tmp));
            }
            _ => {}
        }
    }

    configurations
}

fn generate_possible_next_configurations_v2<const N: usize>(
    occupancy: &BurrowOccupancyV2<N>,
) -> Vec<(usize, BurrowOccupancyV2<N>)> {
    let mut configurations = vec![];

    let mut tmp;

    let mut only_owners_inside = [true; 4];
    for room_index in 0..4 {
        let room_owner = room_owner(room_index);
        for i in 0..N {
            if occupancy.rooms[room_index][i] != Some(room_owner)
                && occupancy.rooms[room_index][i] != None
            {
                only_owners_inside[room_index] = false;
                break;
            }
        }
    }

    // out of each room
    for room_index in 0..4 {
        if !only_owners_inside[room_index] {
            let destinations = movements_from_room_to_hallway_v2(occupancy, room_index);
            for j in 0..N {
                if occupancy.rooms[room_index][j].is_some() {
                    for i in 0..11 {
                        if let Some(cost_along_hallway) = destinations[j] {
                            tmp = *occupancy;
                            tmp.hallway[i] = tmp.rooms[room_index][j].take();
                            configurations.push((
                                cost_per_step(room_owner(room_index))
                                    * (cost_along_hallway + j + 1),
                                tmp,
                            ));
                        }
                    }
                    break;
                }
            }
        }
    }

    // into a home room
    for room_index in 0..4 {
        if only_owners_inside[room_index] {
            for j in (0..N).rev() {
                if occupancy.rooms[room_index][j].is_none() {
                    let sources = movements_from_hallway_to_room_v2(occupancy, room_index);
                    for i in 0..11 {
                        if let Some(cost_along_hallway) = sources[i] {
                            tmp = *occupancy;
                            tmp.rooms[room_index][j] = tmp.hallway[i].take();
                            configurations.push((
                                cost_per_step(room_owner(room_index))
                                    * (cost_along_hallway + j + 1),
                                tmp,
                            ));
                        }
                    }

                    break;
                }
            }
        }
    }

    configurations
}

fn generate_possible_next_configurations_p2(
    occupancy: &BurrowOccupancyP2,
) -> Vec<(usize, BurrowOccupancyP2)> {
    let mut configurations = vec![];

    let mut tmp;

    // movements out of room A
    if occupancy.hallway[2].is_none() {
        let room_position = 2;
        let hallway_destinations = movements_along_hallway_p2(occupancy, room_position);
        match occupancy.room_a {
            [Some(Amphipod::A), Some(Amphipod::A), Some(Amphipod::A), Some(Amphipod::A)] => {}
            [None, Some(Amphipod::A), Some(Amphipod::A), Some(Amphipod::A)] => {}
            [None, None, Some(Amphipod::A), Some(Amphipod::A)] => {}
            [None, None, None, Some(Amphipod::A)] => {}
            [None, None, None, None] => {}
            [Some(out), _, _, _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_a[0].take();
                    configurations.push((cost_per_step(out) * (1 + cost_along_hallway), tmp));
                }
            }
            [None, Some(out), _, _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_a[1].take();
                    configurations.push((cost_per_step(out) * (2 + cost_along_hallway), tmp));
                }
            }
            [None, None, Some(out), _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_a[2].take();
                    configurations.push((cost_per_step(out) * (3 + cost_along_hallway), tmp));
                }
            }
            [None, None, None, Some(out)] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_a[3].take();
                    configurations.push((cost_per_step(out) * (4 + cost_along_hallway), tmp));
                }
            }
        }
    }
    // movements out of room B
    if occupancy.hallway[4].is_none() {
        let room_position = 4;
        let hallway_destinations = movements_along_hallway_p2(occupancy, room_position);
        match occupancy.room_b {
            [Some(Amphipod::B), Some(Amphipod::B), Some(Amphipod::B), Some(Amphipod::B)] => {}
            [None, Some(Amphipod::B), Some(Amphipod::B), Some(Amphipod::B)] => {}
            [None, None, Some(Amphipod::B), Some(Amphipod::B)] => {}
            [None, None, None, Some(Amphipod::B)] => {}
            [None, None, None, None] => {}
            [Some(out), _, _, _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_b[0].take();
                    configurations.push((cost_per_step(out) * (1 + cost_along_hallway), tmp));
                }
            }
            [None, Some(out), _, _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_b[1].take();
                    configurations.push((cost_per_step(out) * (2 + cost_along_hallway), tmp));
                }
            }
            [None, None, Some(out), _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_b[2].take();
                    configurations.push((cost_per_step(out) * (3 + cost_along_hallway), tmp));
                }
            }
            [None, None, None, Some(out)] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_b[3].take();
                    configurations.push((cost_per_step(out) * (4 + cost_along_hallway), tmp));
                }
            }
        }
    }
    // movements out of room C
    if occupancy.hallway[6].is_none() {
        let room_position = 6;
        let hallway_destinations = movements_along_hallway_p2(occupancy, room_position);
        match occupancy.room_c {
            [Some(Amphipod::C), Some(Amphipod::C), Some(Amphipod::C), Some(Amphipod::C)] => {}
            [None, Some(Amphipod::C), Some(Amphipod::C), Some(Amphipod::C)] => {}
            [None, None, Some(Amphipod::C), Some(Amphipod::C)] => {}
            [None, None, None, Some(Amphipod::C)] => {}
            [None, None, None, None] => {}
            [Some(out), _, _, _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_c[0].take();
                    configurations.push((cost_per_step(out) * (1 + cost_along_hallway), tmp));
                }
            }
            [None, Some(out), _, _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_c[1].take();
                    configurations.push((cost_per_step(out) * (2 + cost_along_hallway), tmp));
                }
            }
            [None, None, Some(out), _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_c[2].take();
                    configurations.push((cost_per_step(out) * (3 + cost_along_hallway), tmp));
                }
            }
            [None, None, None, Some(out)] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_c[3].take();
                    configurations.push((cost_per_step(out) * (4 + cost_along_hallway), tmp));
                }
            }
        }
    }

    // movements out of room D
    if occupancy.hallway[8].is_none() {
        let room_position = 8;
        let hallway_destinations = movements_along_hallway_p2(occupancy, room_position);
        match occupancy.room_d {
            [Some(Amphipod::D), Some(Amphipod::D), Some(Amphipod::D), Some(Amphipod::D)] => {}
            [None, Some(Amphipod::D), Some(Amphipod::D), Some(Amphipod::D)] => {}
            [None, None, Some(Amphipod::D), Some(Amphipod::D)] => {}
            [None, None, None, Some(Amphipod::D)] => {}
            [None, None, None, None] => {}
            [Some(out), _, _, _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_d[0].take();
                    configurations.push((cost_per_step(out) * (1 + cost_along_hallway), tmp));
                }
            }
            [None, Some(out), _, _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_d[1].take();
                    configurations.push((cost_per_step(out) * (2 + cost_along_hallway), tmp));
                }
            }
            [None, None, Some(out), _] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_d[2].take();
                    configurations.push((cost_per_step(out) * (3 + cost_along_hallway), tmp));
                }
            }
            [None, None, None, Some(out)] => {
                for &(hallway_position, cost_along_hallway) in hallway_destinations.iter() {
                    tmp = *occupancy;
                    tmp.hallway[hallway_position] = tmp.room_d[2].take();
                    configurations.push((cost_per_step(out) * (4 + cost_along_hallway), tmp));
                }
            }
        }
    }

    // from hallway to room A
    for (start, cost_along_hallway) in movements_from_hallway_p2(occupancy, 2) {
        match (occupancy.hallway[start], occupancy.room_a) {
            (Some(Amphipod::A), [None, None, None, None]) => {
                tmp = *occupancy;
                tmp.room_a[3] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::A) * (4 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::A), [None, None, None, Some(Amphipod::A)]) => {
                tmp = *occupancy;
                tmp.room_a[2] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::A) * (3 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::A), [None, None, Some(Amphipod::A), Some(Amphipod::A)]) => {
                tmp = *occupancy;
                tmp.room_a[1] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::A) * (2 + cost_along_hallway), tmp));
            }
            (
                Some(Amphipod::A),
                [None, Some(Amphipod::A), Some(Amphipod::A), Some(Amphipod::A)],
            ) => {
                tmp = *occupancy;
                tmp.room_a[0] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::A) * (1 + cost_along_hallway), tmp));
            }
            _ => {}
        }
    }
    // from hallway to room B
    for (start, cost_along_hallway) in movements_from_hallway_p2(occupancy, 4) {
        match (occupancy.hallway[start], occupancy.room_b) {
            (Some(Amphipod::B), [None, None, None, None]) => {
                tmp = *occupancy;
                tmp.room_b[3] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::B) * (4 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::B), [None, None, None, Some(Amphipod::B)]) => {
                tmp = *occupancy;
                tmp.room_b[2] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::B) * (3 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::B), [None, None, Some(Amphipod::B), Some(Amphipod::B)]) => {
                tmp = *occupancy;
                tmp.room_b[1] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::B) * (2 + cost_along_hallway), tmp));
            }
            (
                Some(Amphipod::B),
                [None, Some(Amphipod::B), Some(Amphipod::B), Some(Amphipod::B)],
            ) => {
                tmp = *occupancy;
                tmp.room_b[0] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::B) * (1 + cost_along_hallway), tmp));
            }
            _ => {}
        }
    }
    // from hallway to room C
    for (start, cost_along_hallway) in movements_from_hallway_p2(occupancy, 6) {
        match (occupancy.hallway[start], occupancy.room_c) {
            (Some(Amphipod::C), [None, None, None, None]) => {
                tmp = *occupancy;
                tmp.room_c[3] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::C) * (4 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::C), [None, None, None, Some(Amphipod::C)]) => {
                tmp = *occupancy;
                tmp.room_c[2] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::C) * (3 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::C), [None, None, Some(Amphipod::C), Some(Amphipod::C)]) => {
                tmp = *occupancy;
                tmp.room_c[1] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::C) * (2 + cost_along_hallway), tmp));
            }
            (
                Some(Amphipod::C),
                [None, Some(Amphipod::C), Some(Amphipod::C), Some(Amphipod::C)],
            ) => {
                tmp = *occupancy;
                tmp.room_c[0] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::C) * (1 + cost_along_hallway), tmp));
            }
            _ => {}
        }
    }
    // from hallway to room D
    for (start, cost_along_hallway) in movements_from_hallway_p2(occupancy, 8) {
        match (occupancy.hallway[start], occupancy.room_d) {
            (Some(Amphipod::D), [None, None, None, None]) => {
                tmp = *occupancy;
                tmp.room_d[3] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::D) * (4 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::D), [None, None, None, Some(Amphipod::D)]) => {
                tmp = *occupancy;
                tmp.room_d[2] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::D) * (3 + cost_along_hallway), tmp));
            }
            (Some(Amphipod::D), [None, None, Some(Amphipod::D), Some(Amphipod::D)]) => {
                tmp = *occupancy;
                tmp.room_d[1] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::D) * (2 + cost_along_hallway), tmp));
            }
            (
                Some(Amphipod::D),
                [None, Some(Amphipod::D), Some(Amphipod::D), Some(Amphipod::D)],
            ) => {
                tmp = *occupancy;
                tmp.room_d[0] = tmp.hallway[start].take();
                configurations.push((cost_per_step(Amphipod::D) * (1 + cost_along_hallway), tmp));
            }
            _ => {}
        }
    }
    configurations
}

/// Parse input
/// Input example:
/// #############
/// #...........#
/// ###B#C#B#D###
///   #A#D#C#A#
///   #########
fn parse_input_p1(input: &str) -> Option<BurrowOccupancyV2<2>> {
    let mut lines = input.lines();
    lines.next()?;
    let hallway_line = lines.next()?;
    let rooms_front_line = lines.next()?;
    let rooms_back_line = lines.next()?;

    let hallway = hallway_line
        .chars()
        .skip(1)
        .take(11)
        .map(|chr| Amphipod::try_from(chr).ok())
        .collect::<Vec<Option<Amphipod>>>();

    let mut rooms_front_chars = rooms_front_line.chars().skip(3);
    let mut rooms_back_chars = rooms_back_line.chars().skip(3);

    let room_a = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_b = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_c = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_d = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];

    println!("Got all rooms");

    Some(BurrowOccupancyV2 {
        hallway: hallway.try_into().ok()?,
        rooms: [room_a, room_b, room_c, room_d],
    })
}

/// Parse input
/// Input example:
/// #############
/// #...........#
/// ###B#C#B#D###
///   #A#D#C#A#
///   #########
fn parse_input(input: &str) -> Option<BurrowOccupancy> {
    let mut lines = input.lines();
    lines.next()?;
    let hallway_line = lines.next()?;
    let rooms_front_line = lines.next()?;
    let rooms_back_line = lines.next()?;

    let hallway = hallway_line
        .chars()
        .skip(1)
        .take(11)
        .map(|chr| Amphipod::try_from(chr).ok())
        .collect::<Vec<Option<Amphipod>>>();

    let mut rooms_front_chars = rooms_front_line.chars().skip(3);
    let mut rooms_back_chars = rooms_back_line.chars().skip(3);

    let room_a = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_b = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_c = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_d = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];

    Some(BurrowOccupancy {
        hallway: hallway.try_into().ok()?,
        room_a,
        room_b,
        room_c,
        room_d,
    })
}

/// Parse input, part 2
/// Input example:
/// #############
/// #...........#
/// ###B#C#B#D###
///   #A#D#C#A#
///   #########
///
/// Additional lines:
///   #D#C#B#A#
///   #D#B#A#C#
fn parse_input_p2(input: &str) -> Option<BurrowOccupancyP2> {
    let mut lines = input.lines();
    lines.next()?;
    let hallway_line = lines.next()?;
    let rooms_front_line = lines.next()?;
    let rooms_back_line = lines.next()?;

    let hallway = hallway_line
        .chars()
        .skip(1)
        .take(11)
        .map(|chr| Amphipod::try_from(chr).ok())
        .collect::<Vec<Option<Amphipod>>>();

    let mut rooms_front_chars = rooms_front_line.chars().skip(3);
    let mut rooms_back_chars = rooms_back_line.chars().skip(3);

    let room_a = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Some(Amphipod::D),
        Some(Amphipod::D),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_b = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Some(Amphipod::C),
        Some(Amphipod::B),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_c = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Some(Amphipod::B),
        Some(Amphipod::A),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];
    rooms_front_chars.next();
    rooms_back_chars.next();
    let room_d = [
        Amphipod::try_from(rooms_front_chars.next()?).ok(),
        Some(Amphipod::A),
        Some(Amphipod::C),
        Amphipod::try_from(rooms_back_chars.next()?).ok(),
    ];

    Some(BurrowOccupancyP2 {
        hallway: hallway.try_into().ok()?,
        room_a,
        room_b,
        room_c,
        room_d,
    })
}

fn dump(occupancy: &BurrowOccupancy) {
    println!("#############");
    print!("#");
    for i in 0..11 {
        print_maybe_amphipod(occupancy.hallway[i]);
    }
    println!("#");
    print!("###");
    print_maybe_amphipod(occupancy.room_a[0]);
    print!("#");
    print_maybe_amphipod(occupancy.room_b[0]);
    print!("#");
    print_maybe_amphipod(occupancy.room_c[0]);
    print!("#");
    print_maybe_amphipod(occupancy.room_d[0]);
    println!("###");
    print!("  #");
    print_maybe_amphipod(occupancy.room_a[1]);
    print!("#");
    print_maybe_amphipod(occupancy.room_b[1]);
    print!("#");
    print_maybe_amphipod(occupancy.room_c[1]);
    print!("#");
    print_maybe_amphipod(occupancy.room_d[1]);
    println!("#  ");
    println!("  #########  ");
}

fn is_done_v2<const N: usize>(occupancy: &BurrowOccupancyV2<N>) -> bool {
    for room_index in 0..4 {
        let room_owner = room_owner(room_index);
        for i in 0..N {
            if occupancy.rooms[room_index][i] != Some(room_owner) {
                return false;
            }
        }
    }
    true
}

fn is_done(occupancy: &BurrowOccupancy) -> bool {
    matches!(occupancy.room_a, [Some(Amphipod::A), Some(Amphipod::A)])
        && matches!(occupancy.room_b, [Some(Amphipod::B), Some(Amphipod::B)])
        && matches!(occupancy.room_c, [Some(Amphipod::C), Some(Amphipod::C)])
        && matches!(occupancy.room_d, [Some(Amphipod::D), Some(Amphipod::D)])
}
fn is_done_p2(occupancy: &BurrowOccupancyP2) -> bool {
    matches!(
        occupancy.room_a,
        [
            Some(Amphipod::A),
            Some(Amphipod::A),
            Some(Amphipod::A),
            Some(Amphipod::A)
        ]
    ) && matches!(
        occupancy.room_b,
        [
            Some(Amphipod::B),
            Some(Amphipod::B),
            Some(Amphipod::B),
            Some(Amphipod::B)
        ]
    ) && matches!(
        occupancy.room_c,
        [
            Some(Amphipod::C),
            Some(Amphipod::C),
            Some(Amphipod::C),
            Some(Amphipod::C)
        ]
    ) && matches!(
        occupancy.room_d,
        [
            Some(Amphipod::D),
            Some(Amphipod::D),
            Some(Amphipod::D),
            Some(Amphipod::D)
        ]
    )
}

// Optimization idea:
// - estimate bottom boundary for a cost to reach final positions from a specific state
//     -> remove states where this bottom boundary would be bigger than the optimum found so far

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let input_p1 = parse_input_p1(&contents).unwrap();
    let mut options_p1 = vec![(0, input_p1)];
    let mut current_min_cost_p1 = 20000; // usize::MAX;
    let mut diagnostics_counter_p1: u64 = 0;
    let mut dedup_counter_p1: u64 = 0;
    while let Some((cost, occupancy)) = options_p1.pop() {
        if cost >= current_min_cost_p1 { //skip
        } else if is_done_v2(&occupancy) {
            current_min_cost_p1 = cost;
            println!("[part 1] New min cost: {}", current_min_cost_p1);
        } else {
            for (cost_next, next) in generate_possible_next_configurations_v2(&occupancy) {
                if cost + cost_next < current_min_cost_p1 {
                    if is_done_v2(&next) {
                        current_min_cost_p1 = cost + cost_next;
                        println!("[part 1] New min cost: {}", current_min_cost_p1);
                    } else {
                        options_p1.push((cost + cost_next, next));
                    }
                }
            }
        }
        diagnostics_counter_p1 += 1;
        if diagnostics_counter_p1 > 1000000 {
            println!(
                "[part 1] Options count: {}, min cost: {}",
                options_p1.len(),
                current_min_cost_p1
            );
            diagnostics_counter_p1 = 0;
        }

        /*dedup_counter_p1 += 1;
        if dedup_counter_p1 > 1000000 {
            options_p1.reverse();
            options_p1.sort_unstable_by_key(|k| k.0);
            options_p1.dedup_by_key(|k| k.1);
            options_p1.reverse();
            dedup_counter_p1 = 0;
        }*/
    }

    let input_p2 = parse_input_p2(&contents).unwrap();
    let mut options_p2 = vec![(0, input_p2)];
    let mut current_min_cost_p2 = usize::MAX;
    let mut diagnostics_counter_p2: u64 = 0;
    let mut dedup_counter_p2: u64 = 0;

    while let Some((cost, occupancy)) = options_p2.pop() {
        if cost >= current_min_cost_p2 { //skip
        } else if is_done_p2(&occupancy) {
            current_min_cost_p2 = cost;
            println!("[part 2] New min cost: {}", current_min_cost_p2);
        } else {
            for (cost_next, next) in generate_possible_next_configurations_p2(&occupancy) {
                if cost + cost_next < current_min_cost_p2 {
                    if is_done_p2(&next) {
                        current_min_cost_p2 = cost + cost_next;
                        println!("[part 2] New min cost: {}", current_min_cost_p2);
                    } else {
                        options_p2.push((cost + cost_next, next));
                    }
                }
            }
        }
        diagnostics_counter_p2 += 1;
        if diagnostics_counter_p2 > 10000 {
            println!(
                "[part 2] Options count: {}, min cost: {}",
                options_p2.len(),
                current_min_cost_p2
            );
            diagnostics_counter_p2 = 0;
        }

        dedup_counter_p2 += 1;
        if dedup_counter_p2 > 1000 {
            options_p2.reverse();
            options_p2.sort_unstable_by_key(|k| k.0);
            options_p2.dedup_by_key(|k| k.1);
            options_p2.reverse();
            dedup_counter_p2 = 0;
        }
    }

    let input = parse_input(&contents).unwrap();
    println!("{:?}", input);

    let mut options = vec![(0, input)];
    let mut current_min_cost = usize::MAX;

    let mut diagnostics_counter: u64 = 0;
    let mut dedup_counter: u64 = 0;

    while let Some((cost, occupancy)) = options.pop() {
        if cost >= current_min_cost { // skip
        } else if is_done(&occupancy) {
            current_min_cost = cost;
            println!("New min cost: {}", current_min_cost);
        } else {
            for (cost_next, next) in generate_possible_next_configurations(&occupancy) {
                if cost + cost_next < current_min_cost {
                    if is_done(&next) {
                        current_min_cost = cost + cost_next;
                        println!("New min cost: {}", current_min_cost);
                    } else {
                        options.push((cost + cost_next, next));
                    }
                }
            }
        }
        diagnostics_counter += 1;
        if diagnostics_counter > 1000000 {
            println!(
                "Options count: {}, min cost: {}",
                options.len(),
                current_min_cost
            );
            diagnostics_counter = 0;
        }

        dedup_counter += 1;
        if dedup_counter > 1 {
            options.reverse();
            options.sort_unstable_by_key(|k| k.0);
            options.dedup_by_key(|k| k.1);
            options.reverse();
            dedup_counter = 0;
        }
    }

    println!("Min cost: {}", current_min_cost);

    /*let mut taken_options: Vec<usize> = vec![0, 0, 0, 0, 4, 2, 6, 2, 6, 2, 22];
    while !options.is_empty() {
        let prev_options = options;
        options = Vec::with_capacity(prev_options.len() * 80);
        for (cost, occupancy) in prev_options {
            if cost > current_min_cost {
            } else if is_done(&occupancy) {
                current_min_cost = cost;
            } else {
                for (cost_next, next) in generate_possible_next_configurations(&occupancy) {
                    options.push((cost + cost_next, next));
                }
            }
        }
        println!(
            "Options: {}, current min cost: {}",
            options.len(),
            current_min_cost
        );

        // for (opt_index, option) in options.iter().enumerate() {
        //     println!("Cost: {}, index: {}", option.0, opt_index);
        //     dump(&option.1);
        // }

        //if let Some(taken_option) = taken_options.pop() {
        //    options = vec![options.remove(taken_option)];
        //} else {
        //break;
        //}
    }*/
}
