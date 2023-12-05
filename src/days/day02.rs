use crate::util::read_data;
use regex::Regex;
use std::{cmp, fmt};

const TEST_DATA: [&str; 5] = [
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
];

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    rounds: Vec<Round>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rounds_str = "".to_owned();
        for round in &self.rounds {
            if rounds_str.len() > 0 {
                rounds_str.push_str(", ");
            }
            rounds_str.push_str(
                format!("(R: {}, G: {}, B: {})", round.red, round.green, round.blue).as_str(),
            );
        }
        write!(f, "{}, [{}]", self.id, rounds_str)
    }
}

fn parse_game(game_str: &str) -> Game {
    let re = Regex::new(r"Game (\d+)\: (.*)").unwrap();
    //let id_match = re.captures(game_str).try_into();
    let mut game_id = "";
    let mut rounds_str = "";
    for (_, [id, rounds]) in re.captures_iter(game_str).map(|c| c.extract()) {
        game_id = id;
        rounds_str = rounds;
        break;
    }
    let mut rounds_vec: Vec<Round> = Vec::new();
    let round_red = Regex::new(r"(\d+) red").unwrap();
    let round_green = Regex::new(r"(\d+) green").unwrap();
    let round_blue = Regex::new(r"(\d+) blue").unwrap();
    for round in rounds_str.split(";") {
        let red_str = match round_red.captures(round) {
            Some(value) => value.get(1).unwrap().as_str(),
            None => "0",
        };
        let green_str = match round_green.captures(round) {
            Some(value) => value.get(1).unwrap().as_str(),
            None => "0",
        };
        let blue_str = match round_blue.captures(round) {
            Some(value) => value.get(1).unwrap().as_str(),
            None => "0",
        };

        rounds_vec.push(Round {
            red: red_str.parse::<i32>().unwrap(),
            green: green_str.parse::<i32>().unwrap(),
            blue: blue_str.parse::<i32>().unwrap(),
        });
    }

    let game = Game {
        id: game_id.to_string().parse().unwrap(), //id_match.unwrap().as_str().parse().unwrap(),
        rounds: rounds_vec,
    };
    return game;
}

pub fn part1() {
    let data = read_data::read_lines("./data/day02.txt");
    let mut sum = 0;
    for test_str in data {
        let game = parse_game(&test_str);
        if check_game(&game) {
            sum += game.id;
        }
    }
    println!("{sum}");
}

pub fn part2() {
    let data = read_data::read_lines("./data/day02.txt");
    let mut sum = 0;
    for test_str in data {
        let game = parse_game(&test_str);
        sum += get_game_power(&game);
    }
    println!("{sum}");
}

fn check_game(g: &Game) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    for r in &g.rounds {
        if r.red > max_red || r.green > max_green || r.blue > max_blue {
            return false;
        }
    }
    return true;
}

fn get_game_power(g: &Game) -> i32 {
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;

    for r in &g.rounds {
        min_red = cmp::max(min_red, r.red);
        min_green = cmp::max(min_green, r.green);
        min_blue = cmp::max(min_blue, r.blue);
    }
    return min_red * min_green * min_blue;
}
