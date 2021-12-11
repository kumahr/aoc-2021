use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

const INPUT: &str = "src/aoc5.txt";

fn aoc5_1() {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let mut segments: Vec<Vec<(i32, i32)>> = vec![];
    let mut max_x = 0;
    let mut max_y = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            let vectors_string = l
                .replace(" ", "")
                .split("->")
                .map(|x| String::from(x))
                .collect::<Vec<String>>();
            let left_vector = &vectors_string[0]
                .split(",")
                .map(|coordinate| {
                    if let Ok(n) = i32::from_str_radix(coordinate, 10) {
                        return n;
                    }
                    i32::MIN
                })
                .collect::<Vec<i32>>();
            let right_vector = &vectors_string[1]
                .split(",")
                .map(|coordinate| {
                    if let Ok(n) = i32::from_str_radix(coordinate, 10) {
                        return n;
                    }
                    i32::MIN
                })
                .collect::<Vec<i32>>();
            let start = (left_vector[0], left_vector[1]);
            let end = (right_vector[0], right_vector[1]);
            if start.0 == end.0 || start.1 == end.1 {
                if start.0 > max_x {
                    max_x = start.0;
                } else if end.0 > max_x {
                    max_x = end.0;
                } else if start.1 > max_y {
                    max_y = start.1;
                } else if start.1 > max_y {
                    max_y = start.1;
                }
                let segment_coordinates: Vec<(i32, i32)> = vec![start, end];
                segments.push(segment_coordinates);
            }
        }
    }
    max_x += 1;
    max_y += 1;

    let mut safe_spots = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(max_y as usize);
    for _i in 0..max_y as usize {
        let row: Vec<i32> = vec![0; max_x as usize];
        matrix.push(row);
    }
    for segment in segments {
        let start = segment[0];
        let end = segment[1];
        if start.0 == end.0 {
            let increment = if start.1 >= end.1 { -1 } else { 1 };
            let mut i = start.1;
            while i != (end.1 + increment) {
                matrix[i as usize][start.0 as usize] += 1;
                i += increment;
            }
        } else if start.1 == end.1 {
            let increment = if start.0 >= end.0 { -1 } else { 1 };
            let mut i = start.0;
            while i != (end.0 + increment) {
                matrix[start.1 as usize][i as usize] += 1;
                i += increment;
            }
        }
    }
    for row in matrix {
        safe_spots += row.iter().filter(|p| p >= &&2).count();
    }
    println!("AOC 5 Part 1 Result {:?}", safe_spots);
}

fn aoc5_2() {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let mut segments: Vec<Vec<(i32, i32)>> = vec![];
    let mut max_x = 0;
    let mut max_y = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            let vectors_string = l
                .replace(" ", "")
                .split("->")
                .map(|x| String::from(x))
                .collect::<Vec<String>>();
            let left_vector = &vectors_string[0]
                .split(",")
                .map(|coordinate| {
                    if let Ok(n) = i32::from_str_radix(coordinate, 10) {
                        return n;
                    }
                    i32::MIN
                })
                .collect::<Vec<i32>>();
            let right_vector = &vectors_string[1]
                .split(",")
                .map(|coordinate| {
                    if let Ok(n) = i32::from_str_radix(coordinate, 10) {
                        return n;
                    }
                    i32::MIN
                })
                .collect::<Vec<i32>>();
            let start = (left_vector[0], left_vector[1]);
            let end = (right_vector[0], right_vector[1]);
            if start.0 == end.0
                || start.1 == end.1
                || (start.0 - end.0).abs() == (start.1 - end.1).abs()
            {
                if start.0 > max_x {
                    max_x = start.0;
                } else if end.0 > max_x {
                    max_x = end.0;
                } else if start.1 > max_y {
                    max_y = start.1;
                } else if start.1 > max_y {
                    max_y = start.1;
                }
                let segment_coordinates: Vec<(i32, i32)> = vec![start, end];
                segments.push(segment_coordinates);
            }
        }
    }
    max_x += 1;
    max_y += 1;

    let mut safe_spots = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(max_y as usize);
    for _i in 0..max_y as usize {
        let row: Vec<i32> = vec![0; max_x as usize];
        matrix.push(row);
    }
    for segment in segments {
        let start = segment[0];
        let end = segment[1];
        if start.0 == end.0 {
            let increment = if start.1 >= end.1 { -1 } else { 1 };
            let mut i = start.1;
            while i != (end.1 + increment) {
                matrix[i as usize][start.0 as usize] += 1;
                i += increment;
            }
        } else if start.1 == end.1 {
            let increment = if start.0 >= end.0 { -1 } else { 1 };
            let mut i = start.0;
            while i != (end.0 + increment) {
                matrix[start.1 as usize][i as usize] += 1;
                i += increment;
            }
        } else if (start.0 - end.0).abs() == (start.1 - end.1).abs() {
            // Diagonal
            let horizontal_increment = if start.0 >= end.0 { -1 } else { 1 };
            let vertical_increment = if start.1 >= end.1 { -1 } else { 1 };
            let mut x = start.0;
            let mut y = start.1;
            while x != (end.0 + horizontal_increment) && y != (end.1 + vertical_increment) {
                matrix[y as usize][x as usize] += 1;
                x += horizontal_increment;
                y += vertical_increment;
            }
        }
    }
    for row in matrix {
        safe_spots += row.iter().filter(|p| p >= &&2).count();
    }
    println!("AOC 5 Part 2 Result {:?}", safe_spots);
}

pub fn aoc5() {
    aoc5_1();
    aoc5_2()
}
