/// Solution to an Advent of Code problem, day 21, 2021
/// https://adventofcode.com/2021/day/21
use std::env;
use std::fs;

struct Player {
    position: u64,
    score: u64,
}

struct Die {
    count: u64,
    next: u64,
}

impl Die {
    fn new() -> Die {
        Die { count: 0, next: 1 }
    }
}

impl Iterator for Die {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next;
        self.next = (self.next % 100) + 1;
        self.count += 1;
        Some(ret)
    }
}

/// Throws where sum = 3:
/// 111
/// Throws where sum = 4:
/// 112, 121, 211
/// Throws where sum = 5:
/// 113, 131, 311, 221, 212, 122
/// Throws where sum = 6:
/// 222, 123, 312, 231, 132, 213, 321
/// Throws where sum = 7:
/// 223, 322, 232, 331, 313, 133
/// Throws where sum = 8:
/// 332, 323, 233
/// Throws where sum = 9:
/// 333
///
///  Sum of three throws | Number of universes
/// ---------------------|---------------------
///  3                   | 1                   
///  4                   | 3                   
///  5                   | 6                   
///  6                   | 7                   
///  7                   | 6                   
///  8                   | 3                   
///  9                   | 1                   
///  TOTAL               | 27                  
const DIRAC_DICE_OUTCOMES: [(u64, u64); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

// player 1 score, player 1 position, player 2 score, player 2 position
type GameStateHistogram = [[[[u64; 10]; 22]; 10]; 22];

fn dirac_round(mut prev: GameStateHistogram) -> GameStateHistogram {
    let mut next: GameStateHistogram = [[[[0; 10]; 22]; 10]; 22];

    // player 1 throws the dice
    for player_1_score in 0..22 {
        for player_1_position in 0..10 {
            for player_2_score in 0..22 {
                for player_2_position in 0..10 {
                    if player_1_score < 21 && player_2_score < 21 {
                        for (throws_sum, count) in DIRAC_DICE_OUTCOMES {
                            let new_player_1_position = (player_1_position + throws_sum) % 10;
                            let new_player_1_score =
                                std::cmp::min(21, player_1_score + new_player_1_position + 1);
                            next[new_player_1_score as usize][new_player_1_position as usize]
                                [player_2_score as usize]
                                [player_2_position as usize] += prev[player_1_score as usize]
                                [player_1_position as usize]
                                [player_2_score as usize]
                                [player_2_position as usize]
                                * count;
                        }
                    } else {
                        // these games already ended
                        next[player_1_score as usize][player_1_position as usize]
                            [player_2_score as usize][player_2_position as usize] += prev
                            [player_1_score as usize][player_1_position as usize]
                            [player_2_score as usize][player_2_position as usize];
                    }
                }
            }
        }
    }

    prev = next;
    next = [[[[0; 10]; 22]; 10]; 22];
    // player 2 throws the dice
    for player_1_score in 0..22 {
        for player_1_position in 0..10 {
            for player_2_score in 0..22 {
                for player_2_position in 0..10 {
                    if player_1_score < 21 && player_2_score < 21 {
                        for (throws_sum, count) in DIRAC_DICE_OUTCOMES {
                            let new_player_2_position = (player_2_position + throws_sum) % 10;
                            let new_player_2_score =
                                std::cmp::min(21, player_2_score + new_player_2_position + 1);
                            next[player_1_score as usize][player_1_position as usize]
                                [new_player_2_score as usize]
                                [new_player_2_position as usize] += prev[player_1_score as usize]
                                [player_1_position as usize]
                                [player_2_score as usize]
                                [player_2_position as usize]
                                * count;
                        }
                    } else {
                        // these games already ended
                        next[player_1_score as usize][player_1_position as usize]
                            [player_2_score as usize][player_2_position as usize] += prev
                            [player_1_score as usize][player_1_position as usize]
                            [player_2_score as usize][player_2_position as usize];
                    }
                }
            }
        }
    }

    next
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let initial_positions = contents
        .lines()
        .map(|l| {
            let (_, pos) = l.split_once(": ").unwrap();
            pos.parse().unwrap()
        })
        .collect::<Vec<u64>>();

    /* Part 1 */
    let mut players = initial_positions
        .iter()
        .map(|&position| Player { position, score: 0 })
        .collect::<Vec<Player>>();
    let mut die = Die::new();
    'game_loop: loop {
        for (player_i, player) in players.iter_mut().enumerate() {
            let moves: u64 = die.by_ref().take(3).sum();
            player.position = (player.position + moves - 1) % 10 + 1;
            player.score += player.position as u64;
            if player.score >= 1000 {
                println!(
                    "[part 1] losing score * die roll count: {}",
                    players[(player_i + 1) % 2].score * die.count
                );
                break 'game_loop;
            }
        }
    }

    /* Part 2 */
    let mut game_state_histogram: GameStateHistogram = [[[[0; 10]; 22]; 10]; 22];
    game_state_histogram[0][initial_positions[0] as usize - 1][0]
        [initial_positions[1] as usize - 1] = 1;

    loop {
        let next = dirac_round(game_state_histogram);
        if next == game_state_histogram {
            // All games ended
            let mut player_1_wins = 0;
            let mut player_2_wins = 0;
            for player_1_position in 0..10 {
                for player_2_position in 0..10 {
                    for player_2_score in 0..21 {
                        player_1_wins += game_state_histogram[21][player_1_position]
                            [player_2_score][player_2_position];
                    }
                }
            }
            for player_1_position in 0..10 {
                for player_2_position in 0..10 {
                    #[allow(clippy::needless_range_loop)]
                    for player_1_score in 0..21 {
                        player_2_wins += game_state_histogram[player_1_score][player_1_position]
                            [21][player_2_position];
                    }
                }
            }

            println!("[part 2] Player 1 won in {} universes", player_1_wins);
            println!("[part 2] Player 2 won in {} universes", player_2_wins);
            break;
        } else {
            game_state_histogram = next;
        }
    }
}
