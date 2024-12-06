#![allow(dead_code, unused)]

use std::{fs::File, io::Read};

use regex::Regex;

#[tokio::main]
async fn main() {
    let input = get_puzzel_input(
        "https://adventofcode.com/2024/day/2/input",
        &std::env::var("SESSION_COOKIE").expect("Set the AoC session cookie"),
    )
    .await;

    // let mut input: String = String::new();
    // let mut input_amount = File::open("day1").unwrap().read_to_string(&mut input);

    advent_of_code_day_2(input).await;
}

async fn get_puzzel_input(input_url: &str, session_cookie: &str) -> String {
    let client = reqwest::Client::new();
    let input = client
        .get(input_url)
        .header("Cookie", session_cookie)
        .send()
        .await;

    input
        .expect("Shouldn't be possible dick")
        .text()
        .await
        .unwrap()
}

async fn advent_of_code_day_1_part_1(input: String) {
    let mut total_distance = 0;

    let lines: Vec<&str> = input.lines().collect();
    let mut side1: Vec<i32> = Vec::new();
    let mut side2: Vec<i32> = Vec::new();

    for i in lines {
        let hello = i.split_once(" ").unwrap();
        side1.push(hello.0.trim().parse::<i32>().unwrap());
        side2.push(hello.1.trim().parse::<i32>().unwrap());
    }

    side1.sort();
    side2.sort();

    if side1.len() == side2.len() {
        for i in 0..side1.len() {
            total_distance += side1[i].abs_diff(side2[i]);
        }
    }
    println!("Total distance: {}", total_distance);
}

async fn advent_of_code_day_1_part_2(input: String) {
    let mut total_distance = 0;

    let lines: Vec<&str> = input.lines().collect();
    let mut side1: Vec<i32> = Vec::new();
    let mut side2: Vec<i32> = Vec::new();

    for i in lines {
        let hello = i.split_once(" ").unwrap();
        side1.push(hello.0.trim().parse::<i32>().unwrap());
        side2.push(hello.1.trim().parse::<i32>().unwrap());
    }

    side1.sort();

    for i in side1 {
        let mut count = 0;
        for b in side2.clone() {
            if i == b {
                count += 1;
            }
        }
        total_distance += (count * i);
    }

    println!("Total distance: {}", total_distance);
}

async fn advent_of_code_day_2(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let lines_amount = lines.len();
    let mut safe_count = 0;

    for i in lines {
        let mut safe = true;
        let mut unsafe_count = 0;
        let mut non_safe_reason = "".to_string();

        let mut numbers: Vec<i32> = Vec::new();
        let split = i.split(" ");
        for b in split {
            numbers.push(b.parse::<i32>().unwrap());
        }
        if !numbers.is_sorted_by(|a, b| a < b) && !numbers.is_sorted_by(|a, b| a > b) {
            safe = false;
            non_safe_reason = "non-sort".to_string();
        } else {
            let mut numbers_iter = numbers.iter().peekable();
            for x in 0..numbers_iter.len() {
                let i = numbers_iter.next().unwrap();
                if let Some(o) = numbers_iter.peek() {
                    unsafe_count += 1;
                    if i.abs_diff(o.to_owned().to_owned()) > 3 {
                        safe = false;
                        non_safe_reason = "ABS>".to_string();
                    }
                }
            }
        }
        println!(
            "{:?} is safe: {} because {}",
            numbers, safe, non_safe_reason
        );
        if safe {
            safe_count += 1;
        }
    }
    println!(
        "Safe lines are : {} with line count {}",
        safe_count, lines_amount
    )
}

async fn advent_of_code_day_3(input_url: &str, session_cookie: &str) {
    let input = get_puzzel_input(input_url, session_cookie).await;

    let regex = Regex::new(r"mul\((?P<mul1>\d+),(?P<mul2>\d+)\)").unwrap();

    let matches: Vec<i64> = regex
        .captures_iter(&input)
        .map(|i| {
            let mul1 = i.name("mul1").unwrap().as_str().parse::<i64>().unwrap();
            let mul2 = i.name("mul2").unwrap().as_str().parse::<i64>().unwrap();
            mul1 * mul2
        })
        .collect();

    let mut total: i64 = 0;
    for i in matches {
        total += i;
    }
    println!("Total is {}", total)
}

async fn advent_of_code_day_3_part2(input_url: &str, session_cookie: &str) {
    let client = reqwest::Client::new();
    let input = client
        .get(input_url)
        .header("Cookie", session_cookie)
        .send()
        .await;

    let input = input
        .expect("Shouldn't be possible dick")
        .text()
        .await
        .unwrap();

    let regex = Regex::new(r"mul\((?P<mul1>\d+),(?P<mul2>\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut active = true;
    println!("{}", input);
    let matches: Vec<i64> = regex
        .captures_iter(&input)
        .map(|i| {
            let string = i.get(0).unwrap().as_str();
            if string.contains("don't") {
                active = false;
            } else if string.contains("do") {
                active = true;
            } else if active {
                let mul1 = i.name("mul1").unwrap().as_str().parse::<i64>().unwrap();
                let mul2 = i.name("mul2").unwrap().as_str().parse::<i64>().unwrap();
                return mul1 * mul2;
            }
            0
        })
        .collect();

    let mut total: i64 = 0;
    for i in matches {
        total += i;
    }
    println!("Total is {}", total)
}

async fn advent_of_code_day_4(input: String) {
    let mut xmas_count = 0;

    let lines = input.lines();
    let line_amount = lines.clone().count();
    let line_index = line_amount - 1;
    let mut array: Vec<Vec<char>> = Vec::new();

    println!("Line amount is: {}", line_amount);
    println!("Array size is: {}", array.len());

    array.push(Vec::new());
    for (c, i) in lines.into_iter().enumerate() {
        array.push(Vec::new());
        for (a, s) in i.chars().enumerate() {
            array[c].push(s);
        }
    }
    let charachters_per_line = array[0].len();
    let charachters_per_line_index = charachters_per_line - 1;

    for i in 0..line_index {
        for b in 0..charachters_per_line_index {
            let forwards: String = array[i].get(b..4).unwrap_or(&['L']).iter().collect();

            let backwards: String;
            if b >= 4 {
                backwards = array[i].get((b - 4)..b).unwrap_or(&['L']).iter().collect();
            } else {
                backwards = array[i].get(0..b).unwrap_or(&['L']).iter().collect();
            }
            let mut downward: String = "".to_string();
            let mut upwards: String = "".to_string();
            let mut diaganol: Vec<String> = Vec::new();

            for d in i..i + 3 {
                downward.push_str(&array[d].get(b).unwrap_or(&'L').clone().to_string());
            }

            if i > 3 {
                for d in i - 3..i {
                    upwards.push(array[d].get(b).unwrap_or(&'L').to_owned());
                }
            } else {
                for d in 0..i {
                    upwards.push(array[d].get(b).unwrap_or(&'L').to_owned());
                }
            }

            for m in [
                [(1, 1), (2, 2), (3, 3)],
                [(1, -1), (2, -2), (3, -3)],
                [(-1, 1), (-2, 2), (-3, 3)],
                [(-1, -1), (-2, -2), (-3, -3)],
            ] {
                let mut hello = "".to_string();
                hello.push(array[i].get(b).unwrap_or(&'L').to_owned());
                for c in m {
                    let mut first_index = 0;
                    let mut second_index = 0;

                    if (i as i32 + c.0) > 0 {
                        first_index = (i as i32 + c.0) as usize;
                    }

                    if (b as i32 + c.1) > 0 {
                        second_index = (b as i32 + c.1) as usize;
                    }
                    hello.push({
                        let gotten = array.get(first_index);
                        if let Some(o) = gotten {
                            o.get(second_index).unwrap_or(&'L').to_owned()
                        } else {
                            'L'
                        }
                    })
                }
                if !hello.contains("L") && hello != "" {
                    println!("Diagn string: {}", hello);
                }
                diaganol.push(hello);
            }

            if forwards.contains("XMAS") || forwards.contains("SAMX") {
                xmas_count += 1;
            }
            if backwards.contains("XMAS") || backwards.contains("SAMX") {
                xmas_count += 1;
            }
            if downward.contains("XMAS") || downward.contains("SAMX") {
                xmas_count += 1;
            }
            if upwards.contains("XMAS") || upwards.contains("SAMX") {
                xmas_count += 1;
            }
            for s in diaganol {
                if s.contains("XMAS") || s.contains("SAMX") {
                    xmas_count += 1;
                }
            }
        }
    }
    println!("XMAS COUNT IS: {}", xmas_count);
}
