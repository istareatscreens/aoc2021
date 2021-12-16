use log::debug;
use std::{collections::HashMap, fs};

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let mut lines: Vec<String> = data.lines().map(|s| s.to_string()).collect();
    let break_index: usize = lines
        .clone()
        .into_iter()
        .position(|s| s == *"".to_string())
        .unwrap();

    debug!("Input Data: {:?}", lines);
    debug!("Break Index: {}", break_index);
    lines.remove(break_index);
    let folds: Vec<Vec<_>> = lines
        .split_off(break_index)
        .iter()
        .map(|a| {
            a[11..]
                .split('=')
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<String>>>();

    debug!("Folds Data: {:?}", folds);

    let cooridnates: Vec<Vec<i32>> = lines
        .iter()
        .map(|a| {
            a.split(',')
                .map(|c| c.to_string().parse::<i32>().expect("Error parsing {}"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut paper: HashMap<String, i32> = HashMap::new();
    for coordinate in cooridnates.iter() {
        paper.insert(
            PaperFold::make_key(&[coordinate[0].to_string(), coordinate[1].to_string()]),
            0,
        );
    }
    debug!("\nCoordinates: {:?}", cooridnates);

    let mut paper_fold = PaperFold {
        paper: paper.clone(),
    };

    paper_fold.fold((
        folds[0][0].to_owned(),
        folds[0][1].parse::<i32>().expect("Error parsing"),
    ));

    println!("P1 Solution: {:?}", paper_fold.count_dots());

    for (i, _) in folds.iter().enumerate() {
        if i == 0 {
            continue;
        }
        paper_fold.fold((
            folds[i][0].to_owned(),
            folds[i][1].parse::<i32>().expect("Error parsing"),
        ));
    }

    println!("P2 Solution:");
    paper_fold.print_dots();
}

struct PaperFold {
    paper: HashMap<String, i32>,
}

impl PaperFold {
    pub fn count_dots(&self) -> usize {
        self.paper.len()
    }

    pub fn print_dots(&self) {
        let mut x_max = i32::MIN;
        let mut x_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut y_min = i32::MAX;
        for (cell, _) in self.paper.iter() {
            let (x, y) = self.parse_key(cell);
            if x > x_max {
                x_max = x;
            }
            if x < x_min {
                x_min = x;
            }
            if y > y_max {
                y_max = y;
            }
            if y < y_min {
                y_min = y;
            }
        }

        for i in 0..(y_max - y_min) + 1 {
            for j in 0..(x_max - x_min) + 1 {
                if self.paper.contains_key(&PaperFold::make_key(&[
                    (j + x_min).to_string(),
                    (i + y_min).to_string(),
                ])) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn fold(&mut self, fold: (String, i32)) {
        if fold.0 == "x" {
            self.fold_left_verticle(fold.1);
        } else if fold.0 == "y" {
            self.fold_up_horizontal(fold.1);
        } else {
            debug!("You goofed");
        }
    }

    fn fold_up_horizontal(&mut self, fold_location: i32) {
        for (coordinate, _) in self.paper.to_owned().iter() {
            let (x, mut y) = self.parse_key(&coordinate.to_owned());
            if y > fold_location {
                y = fold_location - (y - fold_location);
                self.paper.remove(coordinate);
                let key = &PaperFold::make_key(&[x.to_string(), y.to_string()]);
                if let std::collections::hash_map::Entry::Vacant(e) =
                    self.paper.entry(key.to_owned())
                {
                    e.insert(0);
                }
            }
        }
    }

    fn fold_left_verticle(&mut self, fold_location: i32) {
        for (coordinate, _) in self.paper.to_owned().iter() {
            let (mut x, y) = self.parse_key(&coordinate.to_owned());
            if x > fold_location {
                x = fold_location - (x - fold_location);
                self.paper.remove(coordinate);
                let key = &PaperFold::make_key(&[x.to_string(), y.to_string()]);
                if let std::collections::hash_map::Entry::Vacant(e) =
                    self.paper.entry(key.to_owned())
                {
                    e.insert(0);
                }
            }
        }
    }

    fn parse_key(&self, strings: &str) -> (i32, i32) {
        let temp: Vec<_> = strings.split(',').collect();
        (
            temp[0].parse::<i32>().unwrap(),
            temp[1].parse::<i32>().unwrap(),
        )
    }

    pub fn make_key(strings: &[String]) -> String {
        return strings
            .iter()
            .map(|s| s.to_owned() + ",")
            .collect::<Vec<String>>()
            .concat();
    }
}
