use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "src/aoc1.txt";

fn aoc1_1() {
    let mut measurements: Vec<i32> = Vec::new();
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                measurements.push(l.parse::<i32>().unwrap());
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    let mut count: i32 = 0;

    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            count += 1;
        }
    }
    println!("AOC 1 Part 1 Result {}", count);
}

fn aoc1_2() {
    let mut three_dim_measurements: Vec<i64> = Vec::new();
    let mut measurements: Vec<i64> = Vec::new();
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);

    // Convert file into measurements
    for line in reader.lines() {
        match line {
            Ok(l) => {
                measurements.push(l.parse::<i64>().unwrap());
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }

    // Build three dim measurements
    for i in 0..measurements.len() {
        if i + 2 > measurements.len() - 1 {
            break;
        }
        three_dim_measurements.push(measurements[i] + measurements[i + 1] + measurements[i + 2]);
    }
    let mut count = 0;
    for i in 0..three_dim_measurements.len() {
        if i + 1 > three_dim_measurements.len() - 1 {
            break;
        }
        if three_dim_measurements[i] < three_dim_measurements[i + 1] {
            count += 1;
        }
    }

    println!("AOC 1 Part 2 Result {}", count);
}

pub fn aoc1() {
    aoc1_1();
    aoc1_2();
}
