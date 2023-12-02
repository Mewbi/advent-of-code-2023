use std::fs::read_to_string;
use std::collections::HashMap;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn part_1() {
    println!("Part 1");
    const RADIX: u32 = 10;
    let mut sum = 0;
    for line in read_lines("input.txt") {
        let mut first_num = 0;
        let mut last_num = 0;
        for char in line.chars() {
            if let Some(digit) = char.to_digit(RADIX) {
                first_num = digit;
                break;
            }
        }

        for char in line.chars().rev() {
            if let Some(digit) = char.to_digit(RADIX) {
                last_num = digit;
                break;
            }
        }
        sum += first_num * 10 + last_num
    }
    println!("Sum = {sum}");
}

fn text_has_string_num(text: &str, in_start: bool) -> u32 {

    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (num_string, num) in numbers {
        if in_start {
            if text.starts_with(num_string) {
                return num;
            }
        } else {
            if text.ends_with(num_string) {
                return num;
            }
        }
    }
    return 0;
}

fn part_2() {
    println!("Part 2");
    const RADIX: u32 = 10;
    let mut sum = 0;
    for line in read_lines("input_2.txt") {
        let mut first_num = 0;
        let mut last_num = 0;
        for (idx, char) in line.char_indices() {
            if let Some(digit) = char.to_digit(RADIX) {
                first_num = digit;
                break;
            }
            let slice = &line[idx..];
            let num = text_has_string_num(slice, true);
            if num != 0 {
                first_num = num;
                break;
            }
        }

        for (idx, char) in line.char_indices().rev() {
            if let Some(digit) = char.to_digit(RADIX) {
                last_num = digit;
                break;
            }
            let slice = &line[..idx + 1];
            let num = text_has_string_num(slice, false);
            if num != 0 {
                last_num = num;
                break;
            }
        }
        sum += first_num * 10 + last_num
    }
    println!("Sum = {sum}");
}

fn main() {
    part_1();
    part_2();
}
