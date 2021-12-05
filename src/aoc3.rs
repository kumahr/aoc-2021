use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "src/aoc3.txt";

fn aoc3_1() {
    // gamma rate => most common bit
    // epsilon rate => least common bit
    let mut columns: HashMap<usize, Vec<usize>> = HashMap::new();
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(l) = line {
            for char_vector in l.char_indices() {
                if let None = columns.get_mut(&char_vector.0) {
                    // create entry
                    columns.insert(char_vector.0, vec![]);
                }

                if let Some(digit) = char_vector.1.to_digit(10) {
                    if let Some(bits) = columns.get_mut(&char_vector.0) {
                        bits.push(digit as usize);
                    }
                }
            }
        }
    }
    let mut k = columns.keys().map(|k| k).collect::<Vec<&usize>>();
    k.sort();

    let mut gamma_rate_bits = String::from("");
    let mut epsilon_rate_bits = String::from("");
    for bit_index in k {
        if let Some(bits) = columns.get(bit_index) {
            let mut sum_bit_one = 0;
            let mut sum_bit_zero = 0;
            for bit in bits {
                if *bit == 0 {
                    sum_bit_zero += 1;
                }
                if *bit == 1 {
                    sum_bit_one += 1;
                }
            }
            if sum_bit_one > sum_bit_zero {
                gamma_rate_bits.push('1');
                epsilon_rate_bits.push('0');
            } else {
                epsilon_rate_bits.push('1');
                gamma_rate_bits.push('0');
            }
        }
    }
    let numeric_gamma_rate = isize::from_str_radix(&gamma_rate_bits, 2).unwrap();
    let numeric_epsilon_rate = isize::from_str_radix(&epsilon_rate_bits, 2).unwrap();

    println!(
        "AOC 3 Part 1 Result {}",
        numeric_epsilon_rate * numeric_gamma_rate
    )
}

fn filter_ratings(mut ratings: Vec<String>, bit_criteria: impl Fn(i32) -> char) -> isize {
    let mut char_index = 0;
    while ratings.len() > 1 {
        let mut b_values: Vec<u32> = vec![];
        for bits in &ratings {
            if char_index == bits.len() {
                char_index = 0;
            }
            if let Some(bit) = bits.chars().nth(char_index) {
                b_values.push(bit.to_digit(10).unwrap());
            }
        }
        let mut sum_bit_zero = 0;
        let mut sum_bit_one = 0;
        for v in b_values {
            if v == 0 {
                sum_bit_zero += 1;
            }
            if v == 1 {
                sum_bit_one += 1;
            }
        }
        let delta = sum_bit_one - sum_bit_zero;
        let filterting_bit: char = bit_criteria(delta);
        ratings = ratings
            .iter()
            .filter(|s| {
                if let Some(c) = s.chars().nth(char_index) {
                    return c == filterting_bit;
                }
                false
            })
            .cloned()
            .collect::<Vec<String>>();
        char_index += 1;
    }
    if ratings.len() == 1 {
        return isize::from_str_radix(&ratings[0], 2).unwrap();
    }
    return -1;
}

fn aoc3_2() {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let mut oxygen_ratings: Vec<String> = vec![];
    let mut carbon_dioxyde_ratings: Vec<String> = vec![];

    for line in reader.lines() {
        if let Ok(l) = line {
            oxygen_ratings.push(l.clone());
            carbon_dioxyde_ratings.push(l.clone());
        }
    }

    let oxygen_bit_criteria = |d: i32| -> char {
        if d < 0 {
            return '0';
        }
        return '1';
    };
    let oxygen_rating = filter_ratings(oxygen_ratings, oxygen_bit_criteria);
    let carbon_bit_criteria = |d: i32| -> char {
        if d < 0 {
            return '1';
        }
        return '0';
    };
    let carbon_dioxyde_rating = filter_ratings(carbon_dioxyde_ratings, carbon_bit_criteria);
    println!(
        "AOC 3 Part 2 Result {}",
        oxygen_rating * carbon_dioxyde_rating
    )
}

pub fn aoc3() {
    aoc3_1();
    aoc3_2()
}
