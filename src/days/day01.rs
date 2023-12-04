use crate::util::read_data;

const TEST_DATA: [&str; 4] = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

pub fn part1() {
    let mut sum = 0;

    let data = read_data::read_lines("./data/day01.txt");

    for test_str in data {
        let word_val = first_int(&*test_str) * 10 + last_int(&*test_str);
        sum = sum + word_val;
    }
    println!("{sum}");
}

fn first_int(my_str: &str) -> u32 {
    for c in my_str.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }
    return 11;
}

fn last_int(my_str: &str) -> u32 {
    for c in my_str.chars().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }
    return 11;
}
