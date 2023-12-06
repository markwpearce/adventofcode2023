use std::fmt;

use crate::util::read_data;
use regex::Regex;

const TEST_DATA: [&str; 6] = [
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
    "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
    "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
    "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
    "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
    "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
];

#[derive(Debug)]
struct Card {
    id: i32,
    numbers: Vec<i32>,
    winning: Vec<i32>,
}

pub fn part1() {
    let data = read_data::read_lines("./data/day04.txt");
    let mut sum = 0;
    let mut cards: Vec<Card> = Vec::new();
    for test_str in data {
        cards.push(parse_card(test_str.as_str()));
    }

    for card in cards {
        let mut card_sum = 0;
        let mut has_matched = false;
        for num in card.numbers {
            if card.winning.contains(&num) {
                card_sum = if has_matched { card_sum * 2 } else { 1 };
                has_matched = true;
            }
        }
        println!("Card {} points: {}", card.id, card_sum);
        sum += card_sum;
    }
    println!("{sum}");
}

fn parse_card(card_str: &str) -> Card {
    let re = Regex::new(r"Card\s+(\d+)\: (.*)").unwrap();
    let capture = re.captures(card_str).unwrap();
    let line_parts: (&str, [&str; 2]) = capture.extract();
    let id = line_parts.1[0].parse::<i32>().unwrap();

    let all_num_parts_str = line_parts.1[1];
    let numbers = all_num_parts_str
        .split("|")
        .nth(0)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let winning = all_num_parts_str
        .split("|")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    return Card {
        id: id,
        numbers: numbers,
        winning: winning,
    };
}

pub fn part2() {
    let data = read_data::read_lines("./data/day04.txt");
    let mut sum = 0;
    let mut cards: Vec<Card> = Vec::new();
    let mut card_copies: Vec<i32> = Vec::new();
    for test_str in data {
        cards.push(parse_card(test_str.as_str()));
        card_copies.push(1);
    }
    let mut i: usize = 0;
    for card in cards {
        let mut wins = 0;
        for num in card.numbers {
            if card.winning.contains(&num) {
                wins += 1;
            }
        }
        for j in 1..wins + 1 {
            card_copies[((j as usize) + i)] += card_copies[i];
        }

        //println!("Card {} wins: {}", card.id, wins);
        //println!("{:?}", card_copies);
        i += 1;
    }
    //println!("{:?}", card_copies);
    println!("{}", card_copies.iter().sum::<i32>());
}
