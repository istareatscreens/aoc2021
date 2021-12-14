use log::debug;
use std::{collections::HashMap, fs};

const C_ROUND_BRACKET: char = ')';
const C_CURLY_BRACKET: char = '}';
const C_ANGEL_BRACKET: char = '>';
const C_SQUARE_BRACKET: char = ']';
const O_ROUND_BRACKET: char = '(';
const O_CURLY_BRACKET: char = '{';
const O_ANGEL_BRACKET: char = '<';
const O_SQUARE_BRACKET: char = '[';
const INVALID_BRACKET: char = '-';

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let lines: Vec<Vec<_>> = data
        .lines()
        .map(|a: &str| a.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    debug!("Output: {:?}", lines);
    let mut char_map: HashMap<char, u64> = HashMap::from([
        (C_ANGEL_BRACKET, 0),
        (C_CURLY_BRACKET, 0),
        (C_ROUND_BRACKET, 0),
        (C_SQUARE_BRACKET, 0),
        (INVALID_BRACKET, 0),
    ]);
    for line in lines.iter() {
        *char_map.get_mut(&find_first_chunk_error(line)).unwrap() += 1;
    }

    let mut sum: u64 = 0;
    for (bracket, count) in char_map.iter() {
        sum += get_points(*bracket) * (*count);
    }

    println!("P1 Solution: {}", sum);

    let mut scores: Vec<u64> = Vec::new();

    for line in lines.iter() {
        let mut score: u64 = 0;
        if find_first_chunk_error(line) == INVALID_BRACKET {
            for bracket in get_leftover_brackets(line).iter().rev() {
                score = get_score(score, *bracket);
            }
            scores.push(score);
        }
    }

    scores.sort_unstable();

    println!("P2 Solution: {}", scores[scores.len() / 2])
}

fn get_score(score: u64, bracket: char) -> u64 {
    let temp: u64 = score * 5;
    match bracket {
        O_ROUND_BRACKET => temp + 1,
        O_SQUARE_BRACKET => temp + 2,
        O_CURLY_BRACKET => temp + 3,
        O_ANGEL_BRACKET => temp + 4,
        _ => temp,
    }
}

fn get_leftover_brackets(line: &[char]) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();

    for bracket in line.iter() {
        if O_ROUND_BRACKET == *bracket
            || O_CURLY_BRACKET == *bracket
            || O_ANGEL_BRACKET == *bracket
            || O_SQUARE_BRACKET == *bracket
        {
            stack.push(*bracket);
        } else {
            stack.pop().expect("Error Empty");
        }
    }
    stack
}

fn find_first_chunk_error(line: &[char]) -> char {
    let mut stack: Vec<char> = Vec::new();

    for bracket in line.iter() {
        let mut expected: char = INVALID_BRACKET;
        if O_ROUND_BRACKET == *bracket
            || O_CURLY_BRACKET == *bracket
            || O_ANGEL_BRACKET == *bracket
            || O_SQUARE_BRACKET == *bracket
        {
            stack.push(*bracket);
        } else {
            expected = stack.pop().expect("Error Empty");
        }

        if expected != INVALID_BRACKET {
            let is_match: bool;
            match expected {
                O_ROUND_BRACKET => is_match = *bracket == C_ROUND_BRACKET,
                O_SQUARE_BRACKET => is_match = *bracket == C_SQUARE_BRACKET,
                O_CURLY_BRACKET => is_match = *bracket == C_CURLY_BRACKET,
                O_ANGEL_BRACKET => is_match = *bracket == C_ANGEL_BRACKET,
                _ => is_match = false,
            }

            if !is_match {
                return *bracket;
            }
        }
    }
    INVALID_BRACKET
}

fn get_points(bracket: char) -> u64 {
    match bracket {
        C_ROUND_BRACKET => 3,
        C_SQUARE_BRACKET => 57,
        C_CURLY_BRACKET => 1197,
        C_ANGEL_BRACKET => 25137,
        _ => 0,
    }
}
