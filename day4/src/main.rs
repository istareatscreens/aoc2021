use std::fs;
use std::vec::Vec;

/*
struct Board {
    pub board_numbers: HashMap<usize, i32>,
    pub indices: Vec<usize>,
    pub winner: bool,
}
*/
fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");
    let mut lines = data.lines();
    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|a: &str| {
            a.to_string()
                .parse::<i32>()
                .expect("Could not parse number")
        })
        .collect();

    let mut boards: Vec<Vec<i32>> = Vec::new();
    let inner_hash: Vec<i32> = Vec::new();

    for line in lines {
        if line == "" {
            boards.push(inner_hash.clone());
        } else {
            let last_index = boards.len() - 1;
            let row: Vec<_> = line
                .split_whitespace()
                .map(|a: &str| a.to_string().parse::<usize>().unwrap())
                .collect();
            for number in row {
                boards[last_index].push(number as i32);
            }
        }
    }

    let mut winning_indecies: Vec<usize> = Vec::new();
    let mut win_types: Vec<WinType> = Vec::new();
    let mut marked_boards: Vec<Vec<i32>> = Vec::new();

    for i in 0..boards.len() {
        let mut board_marks: Vec<i32> = boards[i].clone();
        for j in 0..numbers.to_owned().len() {
            if boards[i].contains(&numbers[j]) {
                let index = boards[i].iter().position(|&x| x == numbers[j]).unwrap();
                board_marks[index] = -1;
                let win_type: WinType = board_has_bingo(board_marks.to_owned());
                if win_type != WinType::Null {
                    //println!("Bingo: {:?}", board_marks);
                    winning_indecies.push(j);
                    win_types.push(win_type);
                    marked_boards.push(board_marks);
                    break;
                }
            }
        }
    }

    let mut winning_number_index = *winning_indecies.iter().min().unwrap();
    let mut winning_index = winning_indecies
        .iter()
        .position(|&r| r == winning_number_index)
        .unwrap();

    let mut sum = 0;
    for number in &marked_boards[winning_index] {
        if *number != -1 {
            sum += number;
        }
    }

    /*
    println!(
        "Winning board{:?}\nWinning number{:?}\nWinning index{:?}\nSum: {}\ntype:{:?}",
        boards[winning_index],
        numbers[winning_number_index as usize],
        winning_index,
        win_types[winning_index]
    );
    */

    println!(
        "P1 Answer: {}",
        sum * (numbers[winning_number_index] as i32)
    );

    winning_number_index = *winning_indecies.iter().max().unwrap();
    winning_index = winning_indecies
        .iter()
        .position(|&r| r == winning_number_index)
        .unwrap();

    sum = 0;
    for number in &marked_boards[winning_index] {
        if *number != -1 {
            sum += number;
        }
    }

    println!(
        "P2 Answer: {}",
        sum * (numbers[winning_number_index] as i32)
    );
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum WinType {
    Null,
    Row,
    Column,
}

fn board_has_bingo(board_marks: Vec<i32>) -> WinType {
    //println!("BOARD MARKS IN FUNCTION: {:?}", board_marks);
    for i in 0..5 {
        let mut count_row = 0;
        let mut count_col = 0;
        for j in 0..5 {
            if board_marks[j + 5 * i] == -1 {
                count_col += 1;
            }
            if board_marks[i + 5 * j] == -1 {
                count_row += 1;
            }
        }

        if count_row == 5 {
            return WinType::Row;
        }

        if count_col == 5 {
            return WinType::Column;
        }
    }

    return WinType::Null;
}
