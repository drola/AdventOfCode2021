#![allow(clippy::needless_range_loop)]
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

#[derive(Copy, Clone, PartialEq)]
struct BurrowOccupancyV2<const N: usize> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; N]; 4],
}

fn cost_per_step(amphipod: Amphipod) -> usize {
    match amphipod {
        Amphipod::A => 1,
        Amphipod::B => 10,
        Amphipod::C => 100,
        Amphipod::D => 1000,
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
        if i != room_position(0)
            && i != room_position(1)
            && i != room_position(2)
            && i != room_position(3)
        {
            result[i] = Some(position - i);
        }
    }
    for i in (position + 1)..11 {
        if occupancy.hallway[i].is_some() {
            break;
        }
        if i != room_position(0)
            && i != room_position(1)
            && i != room_position(2)
            && i != room_position(3)
        {
            result[i] = Some(i - position);
        }
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
        if occupancy.hallway[i] == Some(owner) {
            result[i] = Some(position - i);
            break;
        } else if occupancy.hallway[i].is_some() {
            break;
        }
    }
    for i in position..11 {
        if occupancy.hallway[i] == Some(owner) {
            result[i] = Some(i - position);
            break;
        } else if occupancy.hallway[i].is_some() {
            break;
        }
    }

    result
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
    // into a home room.
    // Short-circuiting when going into a home room is the
    // most significant optimization when exploring options in this game
    for room_index in 0..4 {
        if only_owners_inside[room_index] {
            let sources = movements_from_hallway_to_room_v2(occupancy, room_index);
            for j in (0..N).rev() {
                if occupancy.rooms[room_index][j].is_none() {
                    for i in 0..11 {
                        if let Some(cost_along_hallway) = sources[i] {
                            if Some(room_owner(room_index)) == occupancy.hallway[i] {
                                tmp = *occupancy;
                                tmp.rooms[room_index][j] = tmp.hallway[i].take();
                                return vec![(
                                    cost_per_step(room_owner(room_index))
                                        * (cost_along_hallway + j + 1),
                                    tmp,
                                )];
                            }
                        }
                    }

                    break;
                }
            }
        }
    }

    // out of each room
    for room_index in 0..4 {
        if !only_owners_inside[room_index] {
            let destinations = movements_from_room_to_hallway_v2(occupancy, room_index);
            for j in 0..N {
                if let Some(moved) = occupancy.rooms[room_index][j] {
                    let cps = cost_per_step(moved);
                    for i in 0..11 {
                        if let Some(cost_along_hallway) = destinations[i] {
                            tmp = *occupancy;
                            tmp.hallway[i] = tmp.rooms[room_index][j].take();
                            configurations.push((cps * (cost_along_hallway + j + 1), tmp));
                        }
                    }
                    break;
                }
            }
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
fn parse_input_p2(input: &str) -> Option<BurrowOccupancyV2<4>> {
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

    Some(BurrowOccupancyV2 {
        hallway: hallway.try_into().ok()?,
        rooms: [room_a, room_b, room_c, room_d],
    })
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

fn solve<const N: usize>(input: BurrowOccupancyV2<N>) -> usize {
    let mut options = vec![(0, input)];
    let mut current_min_cost = usize::MAX;
    while let Some((cost, occupancy)) = options.pop() {
        if cost >= current_min_cost { //skip
        } else if is_done_v2(&occupancy) {
            current_min_cost = cost;
        } else {
            for (cost_next, next) in generate_possible_next_configurations_v2(&occupancy) {
                options.push((cost + cost_next, next));
            }
        }
    }
    current_min_cost
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let input_p1 = parse_input_p1(&contents).unwrap();
    println!("[part 1] Min cost: {}", solve(input_p1));
    let input_p2 = parse_input_p2(&contents).unwrap();
    println!("[part 2] Min cost: {}", solve(input_p2));
}
