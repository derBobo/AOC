use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "INPUT21.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|x| x != "")
        .collect();
    let mut player1 = (
        lines[0]
            .split(":")
            .last()
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap(),
        0,
    );
    let mut player2 = (
        lines[1]
            .split(":")
            .last()
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap(),
        0,
    );
    part1(player1.clone(), player2.clone());
    let res = part2(player1.clone(), player2.clone(), 1);
    println!("{}", std::cmp::max(res.0, res.1));
}
fn part2(player1: (u32, u32), player2: (u32, u32), player: u8) -> (u64, u64) {
    let mut res = (0, 0);
    for i in 3..=9 {
        let mut curr_player = (0, 0);
        if player == 1 {
            curr_player = player1.clone();
        } else {
            curr_player = player2.clone()
        }
        curr_player.0 = (curr_player.0 + i) % 10;
        if curr_player.0 == 0 {
            curr_player.0 = 10
        }
        curr_player.1 += curr_player.0;
        if curr_player.1 > 20 {
            if player == 1 {
                match i {
                    3 | 9 => res.0 += 1,
                    4 | 8 => res.0 += 3,
                    5 | 7 => res.0 += 6,
                    _ => res.0 += 7,
                }
            } else {
                match i {
                    3 | 9 => res.1 += 1,
                    4 | 8 => res.1 += 3,
                    5 | 7 => res.1 += 6,
                    _ => res.1 += 7,
                }
            }
        } else {
            let mut cur = (0, 0);
            if player == 1 {
                cur = part2(curr_player, player2, 2);
            } else {
                cur = part2(player1, curr_player, 1)
            }
            match i {
                3 | 9 => {
                    res.0 += cur.0;
                    res.1 += cur.1;
                }
                4 | 8 => {
                    res.0 += 3 * cur.0;
                    res.1 += 3 * cur.1;
                }
                5 | 7 => {
                    res.0 += 6 * cur.0;
                    res.1 += 6 * cur.1;
                }
                _ => {
                    res.0 += 7 * cur.0;
                    res.1 += 7 * cur.1;
                }
            }
        }
    }

    res
}
fn part1(mut player1: (u32, u32), mut player2: (u32, u32)) {
    let mut throws: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < 300 {
        throws.push(i % 100 + (i + 1) % 100 + (i + 2) % 100 + 3);
        i += 3;
    }
    let mut turn = 1;
    loop {
        player1.0 = (player1.0 + throws[(turn - 1) % 100]) % 10;
        if player1.0 == 0 {
            player1.0 = 10;
        }
        player1.1 += player1.0;
        if player1.1 > 999 {
            println!("{}", player2.1 as u32 * turn as u32 * 3);
            break;
        }
        turn += 1;
        player2.0 = (player2.0 + throws[(turn - 1) % 100]) % 10;
        if player2.0 == 0 {
            player2.0 = 10;
        }
        player2.1 += player2.0;
        if player2.1 > 999 {
            println!("{}", player1.1 as u32 * turn as u32 * 3);
            break;
        }
        turn += 1;
    }
}
