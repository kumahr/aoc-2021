use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "src/aoc6.txt";

fn aoc6_1() {
    const DAYS: i32 = 80;
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let mut lanternfishes: Vec<i32> = vec![];
    for line in reader.lines() {
        if let Ok(l) = line {
            lanternfishes = l
                .split(",")
                .map(|str| {
                    if let Ok(n) = i32::from_str_radix(str, 10) {
                        return n;
                    }
                    -1
                })
                .filter(|x| x >= &0)
                .collect()
        }
    }

    const NEW_FISH: i32 = 8;
    for _i in 0..DAYS {
        let mut new_fishes = 0;
        for fish in 0..lanternfishes.len() {
            lanternfishes[fish] -= 1;
            if lanternfishes[fish] < 0 {
                new_fishes += 1;
                lanternfishes[fish] = 6;
            }
        }
        for _j in 0..new_fishes {
            lanternfishes.push(NEW_FISH);
        }
    }
    println!("AOC 6 Part 1 Result {}", lanternfishes.len());
}

fn aoc6_2() {
    const DAYS: i32 = 256;
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let mut fish_types: [i64; 9] = [0; 9];
    for line in reader.lines() {
        if let Ok(l) = line {
            l.split(",").for_each(|str| {
                if let Ok(n) = usize::from_str_radix(str, 10) {
                    if n < fish_types.len() {
                        fish_types[n] += 1;
                    }
                }
            });
        }
    }
    for _i in 0..DAYS {
        let spawn = fish_types[0];
        for i in 1..fish_types.len() {
            fish_types[i - 1] = fish_types[i];
        }
        fish_types[8] = spawn;
        fish_types[6] += spawn;
    }
    let total_fishes = fish_types.iter().fold(0, |acc, x| acc + x);
    println!("AOC 6 Part 2 Result {}", total_fishes);
}

pub fn aoc6() {
    aoc6_1();
    aoc6_2();
}
