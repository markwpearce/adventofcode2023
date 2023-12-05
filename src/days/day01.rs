use crate::util::read_data;

const TEST_DATA: [&str; 4] = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
const TEST_DATA2: [&str; 7] = [
    "two1nine",
    "eightwothree",
    "abcone2threexyz",
    "xtwone3four",
    "4nineeightseven2",
    "zoneight234",
    "7pqrstsixteen",
];

const NUMBER_STRS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part1() {
    let mut sum: i32 = 0;

    let data = read_data::read_lines("./data/day01.txt");

    for test_str in data {
        let word_val = first_int(&*test_str) * 10 + last_int(&*test_str);
        sum = sum + word_val;
    }
    println!("{sum}");
}

fn first_int(my_str: &str) -> i32 {
    for c in my_str.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    return -1;
}

fn last_int(my_str: &str) -> i32 {
    for c in my_str.chars().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    return -1;
}

pub fn part2() {
    let mut sum = 0;

    let data = read_data::read_lines("./data/day01.txt");

    for test_str in data {
        let first = first_int_str(&*test_str);
        let last = last_int_str(&*test_str);
        let word_val = first * 10 + last;
        println!("{test_str}: {first} {last} {word_val}");

        sum = sum + word_val;
    }
    println!("{sum}");
}

fn check_string_for_string(
    my_str: &str,
    search_str: &str,
    in_val: i32,
    first_index: &mut u32,
    value: &mut i32,
) -> bool {
    let found = my_str.find(search_str);
    if found.is_none() {
        return false;
    }
    let found_index = found.unwrap() as u32;
    if found_index < *first_index {
        first_index.clone_from(&found_index);
        value.clone_from(&in_val);
        return true;
    }
    return false;
}

fn check_string_for_last_string(
    my_str: &str,
    search_str: &str,
    in_val: i32,
    last_index: &mut u32,
    value: &mut i32,
) -> bool {
    let found = my_str.rfind(search_str);
    if found.is_none() {
        return false;
    }
    let found_index = found.unwrap() as u32;
    if found_index > *last_index {
        last_index.clone_from(&found_index);
        value.clone_from(&in_val);
        return true;
    }
    return false;
}

fn first_int_str(my_str: &str) -> i32 {
    let mut first = my_str.len() as u32;
    let mut value = -1;

    for i in 0..10 {
        check_string_for_string(my_str, NUMBER_STRS[i], i as i32, &mut first, &mut value);
    }
    let mut i = 0;
    for c in my_str.chars() {
        if c.is_numeric() && i < first {
            return c.to_digit(10).unwrap() as i32;
        }
        if i > first {
            break;
        }
        i += 1;
    }
    return value;
}

fn last_int_str(my_str: &str) -> i32 {
    let mut last = 0;
    let mut value = -1;

    for i in 0..10 {
        check_string_for_last_string(my_str, NUMBER_STRS[i], i as i32, &mut last, &mut value);
    }

    for i in last..(my_str.len() as u32) {
        let c = my_str.chars().nth(i as usize).unwrap();
        if c.is_numeric() {
            value = c.to_digit(10).unwrap() as i32;
        }
    }
    return value;
}
