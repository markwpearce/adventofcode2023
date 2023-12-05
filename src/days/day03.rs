use crate::util::read_data;
use std::{cmp, fmt};

const TEST_DATA: [&str; 10] = [
    "467..114..",
    "...*......",
    "..35..633.",
    "......#...",
    "617*......",
    ".....+.58.",
    "..592.....",
    "......755.",
    "...$.*....",
    ".664.598..",
];

pub fn part1() {
    let mut sum: i32 = 0;

    let data = read_data::read_lines("./data/day03.txt");
    let mut engine: Vec<EngineLine> = Vec::new();

    let mut i = 0;
    for test_str in data {
        engine.push(parse_data_line(test_str.as_str(), i));
        i += 1;
    }

    //println!("{:?}", engine);

    for line in &engine {
        for part in &line.parts {
            if is_part_next_to_symbol(&part, &engine) {
                sum += part.num;
            }
        }
    }

    println!("{:?}", sum);
}

#[derive(Debug)]
struct Part {
    num: i32,
    index: usize,
    char_size: usize,
    line_index: usize,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[P: {} @ {},{}]", self.num, self.line_index, self.index)
    }
}

#[derive(Debug)]
struct Symbol {
    text: char,
    index: usize,
    line_index: usize,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[S: {} @ {},{}]", self.text, self.line_index, self.index)
    }
}

#[derive(Debug)]
struct EngineLine {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
    length: usize,
}

impl fmt::Display for EngineLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Parts: {:?}, Symbols: {:?}]", self.parts, self.symbols)
    }
}

fn parse_data_line(data: &str, line_index: usize) -> EngineLine {
    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut num = 0;
    let mut nums_in_row = 0;
    let mut i = data.len() - 1;

    for c in data.chars().rev() {
        if c.is_numeric() {
            let c_digit = c.to_digit(10).unwrap() as i32;
            let placement = 10_i32.pow(nums_in_row as u32);
            num += placement * c_digit;
            nums_in_row += 1;
        } else if nums_in_row > 0 {
            parts.push(Part {
                num: num,
                index: i + 1,
                char_size: nums_in_row,
                line_index: line_index,
            });
            nums_in_row = 0;
            num = 0;
        }
        if !c.is_numeric() && c != '.' {
            symbols.push(Symbol {
                text: c,
                index: i,
                line_index: line_index,
            })
        }
        if i == 0 && nums_in_row > 0 {
            parts.push(Part {
                num: num,
                index: i,
                char_size: nums_in_row,
                line_index: line_index,
            });
        }
        if i > 0 {
            i -= 1;
        }
    }

    return EngineLine {
        parts: parts,
        symbols: symbols,
        length: data.len().try_into().unwrap(),
    };
}

fn is_part_next_to_symbol(p: &Part, engine: &Vec<EngineLine>) -> bool {
    if p.line_index > 0 {
        if check_line_for_symbol(p, &engine[p.line_index - 1]) {
            return true;
        }
    }
    let cur_line = &engine[p.line_index];
    if check_line_for_symbol(p, cur_line) {
        return true;
    }
    if p.line_index < engine.len() - 1 {
        if check_line_for_symbol(p, &engine[p.line_index + 1]) {
            return true;
        }
    }

    return false;
}

fn check_line_for_symbol(p: &Part, line: &EngineLine) -> bool {
    let start = if p.index > 0 {
        cmp::max(0, p.index - 1)
    } else {
        0
    };
    let end = cmp::min(line.length, p.index + p.char_size);

    for symbol in line.symbols.as_slice() {
        if symbol.index >= start && symbol.index <= end {
            //println!("{} is next to {}", p, symbol);
            return true;
        }
    }
    return false;
}

pub fn part2() {
    let mut sum: i32 = 0;

    let data = read_data::read_lines("./data/day03.txt");
    let mut engine: Vec<EngineLine> = Vec::new();

    let mut i = 0;
    for test_str in data {
        engine.push(parse_data_line(test_str.as_str(), i));
        i += 1;
    }

    //println!("{:?}", engine);

    for line in &engine {
        for symbol in &line.symbols {
            if symbol.text == '*' {
                sum += get_symbol_gear_ratio(&symbol, &engine);
            }
        }
    }

    println!("{:?}", sum);
}

fn get_symbol_gear_ratio(s: &Symbol, engine: &Vec<EngineLine>) -> i32 {
    let mut parts: Vec<&Part> = Vec::new();
    if s.line_index > 0 {
        parts.extend(&check_line_for_parts(s, &engine[s.line_index - 1]));
    }
    parts.extend(&check_line_for_parts(s, &engine[s.line_index]));
    if s.line_index < engine.len() - 1 {
        parts.extend(&check_line_for_parts(s, &engine[s.line_index + 1]));
    }
    if parts.len() == 2 {
        return parts[0].num * parts[1].num;
    }

    return 0;
}

fn check_line_for_parts<'a>(s: &Symbol, line: &'a EngineLine) -> Vec<&'a Part> {
    let start = if s.index > 0 {
        cmp::max(0, s.index - 1)
    } else {
        0
    };
    let end = cmp::min(line.length, s.index + 1);
    let mut parts: Vec<&Part> = Vec::new();
    for part in &line.parts {
        if (part.index + part.char_size - 1) >= start && part.index <= end {
            //println!("{} is next to {}", s, part);
            parts.push(&part);
        }
    }
    return parts;
}
