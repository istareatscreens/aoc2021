use log::debug;
use std::{collections::HashMap, fmt::Octal, fs};

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let lines: Vec<Vec<u32>> = data
        .lines()
        .map(|a: &str| {
            a.to_string()
                .chars()
                .map(|c: char| c.to_digit(10).expect("Error parsing"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<u32>>>();

    debug!("Output: {:?}", &lines);

    let mut map: Octoflash = Octoflash { flash: 0, lines };

    for _ in 0..100 {
        map.step();
    }

    println!("P1 Solution: {}", map.get_and_reset_flash());

    let lines2: Vec<Vec<u32>> = data
        .lines()
        .map(|a: &str| {
            a.to_string()
                .chars()
                .map(|c: char| c.to_digit(10).expect("Error parsing"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut map2: Octoflash = Octoflash {
        flash: 0,
        lines: lines2,
    };
    println!("P2 Solution: {}", map2.find_synchronized_step());
}

struct Octoflash {
    flash: u64,
    lines: Vec<Vec<u32>>,
}

impl Octoflash {
    fn get_and_reset_flash(&mut self) -> u64 {
        let temp = self.flash;
        self.flash = 0;
        temp
    }

    fn find_synchronized_step(&mut self) -> u64 {
        self.flash = 0;
        let mut synced = false;
        let mut step: u64 = 0;
        while !synced {
            self.step();
            step += 1;
            synced = true;
            'first: for i in 0..self.lines.len() {
                for j in 0..self.lines[i].len() {
                    if self.lines[i][j] != 0 {
                        synced = false;
                        break 'first;
                    }
                }
            }
        }
        step
    }

    fn step(&mut self) {
        for i in 0..self.lines.len() {
            for j in 0..self.lines[i].len() {
                self.lines[i][j] += 1;
            }
        }

        let check_multiple_indecies = |lines: Vec<Vec<u32>>| {
            move |i: i32, j: i32| {
                check_if_index_exists(&lines, &i) && check_if_index_exists(&lines[0], &j)
            }
        };

        let make_hash = |i: usize, j: usize| i.to_string() + "," + j.to_string().as_str();

        let mut visited: HashMap<String, bool> = HashMap::new();
        let mut done = false;
        while !done {
            done = true;
            for i in 0..self.lines.len() {
                for j in 0..self.lines[i].len() {
                    let check = check_multiple_indecies(self.lines.to_owned());
                    let hash = make_hash(i, j);
                    if !visited.contains_key(&hash) && self.lines[i][j] > 9 {
                        visited.insert(hash, true);
                        if check_if_index_exists(&self.lines[i], &(j as i32 + 1)) {
                            self.lines[i][j + 1] += 1;
                        }
                        if check_if_index_exists(&self.lines[i], &(j as i32 - 1)) {
                            self.lines[i][j - 1] += 1;
                        }
                        if check_if_index_exists(&self.lines, &(i as i32 + 1)) {
                            self.lines[i + 1][j] += 1;
                        }
                        if check_if_index_exists(&self.lines, &(i as i32 - 1)) {
                            self.lines[i - 1][j] += 1;
                        }
                        if check(i as i32 + 1, j as i32 + 1) {
                            self.lines[i + 1][j + 1] += 1;
                        }
                        if check(i as i32 - 1, j as i32 - 1) {
                            self.lines[i - 1][j - 1] += 1;
                        }
                        if check(i as i32 + 1, j as i32 - 1) {
                            self.lines[i + 1][j - 1] += 1;
                        }
                        if check(i as i32 - 1, j as i32 + 1) {
                            self.lines[i - 1][j + 1] += 1;
                        }
                        done = false;
                    }
                }
            }
        }

        for i in 0..self.lines.len() {
            for j in 0..self.lines[i].len() {
                if self.lines[i][j] > 9 {
                    self.lines[i][j] = 0;
                    self.flash += 1;
                }
            }
        }
    }
}

fn check_if_index_exists<T>(vec: &[T], index: &i32) -> bool {
    index < &(vec.len() as i32) && index > &-1
}
