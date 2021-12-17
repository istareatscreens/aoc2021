use log::debug;
use std::{collections::HashMap, fs};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let mut lines = data.lines().map(|s| s.to_string());

    debug!("Input Data: {:?}", lines);
    let polymer: Vec<String> = lines
        .next()
        .unwrap()
        .chars()
        .map(|s| s.to_string())
        .collect();

    lines.next();
    debug!("Polymer Data: {:?}", polymer);

    let rules: Vec<Vec<_>> = lines
        .map(|a| {
            a.split("->")
                .map(|c| c.to_string().trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<String>>>();

    debug!("Chain Data: {:?}", rules);

    let mut rules_map: HashMap<String, String> = HashMap::new();

    for value in rules {
        rules_map.insert(value[0].to_owned(), value[1].to_owned());
    }
    debug!("Rules Map: {:?}", rules_map);

    /*
    for _ in 0..10 {
        let mut new_polymer = polymer.clone();
        let mut insertion_count = 0;
        for i in 0..polymer.len() {
            if i + 1 >= polymer.len() {
                break;
            }
            let key = polymer[i].to_owned() + &polymer[i + 1];
            if rules_map.contains_key(&(key)) {
                new_polymer.insert(
                    i + insertion_count + 1,
                    rules_map.get(&key).unwrap().to_owned(),
                );
                insertion_count += 1;
            }
        }
        polymer = new_polymer.clone();
    }

    debug!("Polymer Map: {:?}", rules_map);
    debug!("Polymer: {:?}", polymer);

    let mut count_elements: HashMap<String, u64> = HashMap::new();

    for unit in polymer.iter() {
        if count_elements.contains_key(unit) {
            *count_elements.get_mut(unit).unwrap() += 1;
        } else {
            count_elements.insert(unit.to_owned(), 1);
        }
    }
    debug!("Count Elements: {:?}", count_elements);

    let mut max = ("", u64::MIN);
    let mut min = ("", u64::MAX);

    for (element, count) in count_elements.iter() {
        if count > &max.1 {
            max = (element, count.to_owned());
        }
        if count < &min.1 {
            min = (element, count.to_owned());
        }
    }
    */
    let mut polymerize = Polymerize { polymer, rules_map };
    println!("P1 Solution: {}", polymerize.polymerize(10));
    println!("P2 Solution: {}", polymerize.polymerize_enhance(30));
}

struct Polymerize {
    polymer: Vec<String>,
    rules_map: HashMap<String, String>,
}

impl Polymerize {
    fn polymerize(&mut self, steps: usize) -> u64 {
        for _ in 0..steps {
            let mut insertion_count = 0;
            let mut new_polymer = self.polymer.clone();
            for i in 0..self.polymer.len() {
                if i + 1 >= self.polymer.len() {
                    break;
                }
                let key = self.polymer[i].to_owned() + &self.polymer[i + 1];
                if self.rules_map.contains_key(&(key)) {
                    new_polymer.insert(
                        i + insertion_count + 1,
                        self.rules_map.get(&key).unwrap().to_owned(),
                    );
                    insertion_count += 1;
                }
            }
            self.polymer = new_polymer.clone();
        }

        debug!("Polymer Map: {:?}", self.rules_map);
        debug!("Polymer: {:?}", self.polymer);

        let mut count_elements: HashMap<String, u64> = HashMap::new();

        for unit in self.polymer.iter() {
            if count_elements.contains_key(unit) {
                *count_elements.get_mut(unit).unwrap() += 1;
            } else {
                count_elements.insert(unit.to_owned(), 1);
            }
        }
        debug!("Count Elements: {:?}", count_elements);

        let mut max = ("", u64::MIN);
        let mut min = ("", u64::MAX);

        for (element, count) in count_elements.iter() {
            if count > &max.1 {
                max = (element, count.to_owned());
            }
            if count < &min.1 {
                min = (element, count.to_owned());
            }
        }
        debug!("MIN: {:?} MAX: {:?}", min, max);
        max.1 - min.1
    }

    fn polymerize_enhance(&mut self, steps: usize) -> u64 {
        let mut pairs: HashMap<String, u64> = HashMap::new();
        for i in 0..self.polymer.len() {
            if i + 1 >= self.polymer.len() {
                break;
            }
            let key = self.polymer[i].to_string() + &self.polymer[i + 1];
            if pairs.contains_key(&key) {
                *pairs.get_mut(&key).unwrap() += 1;
            } else {
                pairs.insert(key.to_owned(), 1);
            }
        }

        for _ in 0..steps {
            let mut temp_array = pairs.clone();
            for (pair, count) in pairs.iter() {
                if self.rules_map.contains_key(&pair.to_owned()) {
                    let letter = self.rules_map.get(&pair.to_owned()).unwrap().to_owned();
                    let key1 =
                        pair.to_owned().chars().next().unwrap().to_string() + letter.as_str();
                    let key2 = letter.to_string()
                        + pair.to_owned().chars().nth(1).unwrap().to_string().as_str();

                    if !temp_array.contains_key(&key2) {
                        temp_array.insert(key2.to_owned(), count.to_owned());
                    } else {
                        *temp_array.get_mut(&key2).unwrap() += count;
                    }

                    if !temp_array.contains_key(&key1) {
                        temp_array.insert(key1.to_owned(), count.to_owned());
                    } else {
                        *temp_array.get_mut(&key1).unwrap() += count;
                    }
                    *temp_array.get_mut(&pair.to_owned()).unwrap() -= count;
                }
            }
            pairs = temp_array.clone();
        }

        debug!("Polymer Map: {:?}", self.rules_map);
        debug!("Polymer: {:?}", pairs);

        let mut count_elements: HashMap<String, u64> = HashMap::new();

        for (unit, count) in pairs.iter() {
            for letter in unit.chars() {
                let key = &letter.to_owned().to_string();
                if count_elements.contains_key(key) {
                    *count_elements.get_mut(key).unwrap() += count;
                } else {
                    count_elements.insert(key.to_owned(), count.to_owned());
                }
            }
        }

        let mut max = ("", u64::MIN);
        let mut min = ("", u64::MAX);

        for (element, count) in count_elements.iter() {
            if count / 2 > max.1 {
                max = (element, count.to_owned() / 2);
            }
            if count / 2 < min.1 {
                min = (element, count.to_owned() / 2);
            }
        }

        // account for ends

        if max.0 == self.polymer.first().unwrap() || max.0 == self.polymer.last().unwrap() {
            max = (max.0, max.1 + 1);
        }

        if min.0 == self.polymer.first().unwrap() || min.0 == self.polymer.last().unwrap() {
            min = (min.0, min.1 + 1);
        }

        debug!("MIN: {:?} MAX: {:?}", min, max);
        max.1 - min.1
    }
}
