use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "src/aoc2.txt";

fn aoc2_1() {
    let mut position: (i32, i32) = (0, 0);
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let a = l.split(" ").collect::<Vec<&str>>();
                let step = a[1].parse::<i32>().unwrap();
                match a[0] {
                    "forward" => {
                        position.0 += step;
                    }
                    "up" => {
                        position.1 -= step;
                    }
                    "down" => {
                        position.1 += step;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    println!("AOC 2 Part 1 Result {}", position.0 * position.1)
}

fn aoc2_2() {
    // vector (x,y,z)
    // x: horizontal position
    // y: depth
    // z: aim
    let mut position: (i32, i32, i32) = (0, 0, 0);
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let a = l.split(" ").collect::<Vec<&str>>();
                let step = a[1].parse::<i32>().unwrap();
                match a[0] {
                    "forward" => {
                        position.0 += step;
                        position.1 += position.2 * step;
                    }
                    "up" => {
                        position.2 -= step;
                    }
                    "down" => {
                        position.2 += step;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    println!("AOC 2 Part 2 Result {}", position.0 * position.1)
}

pub fn aoc2() {
    aoc2_1();
    aoc2_2();
}
