use log::debug;
use std::collections::HashMap;
use std::fs;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let numbers: Vec<Vec<_>> = data
        .lines()
        .map(|a: &str| {
            a.to_string()
                .chars()
                .map(|c| c.to_digit(10).expect("Could not parse") as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    debug!("Output: {:?}", numbers);
    let mut risk_level = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for (i, digits) in numbers.iter().enumerate() {
        for (j, digit) in digits.iter().enumerate() {
            if (!check_if_index_exists(digits, &(j as i32 + 1)) || numbers[i][j + 1] > *digit)
                && (!check_if_index_exists(digits, &(j as i32 - 1)) || numbers[i][j - 1] > *digit)
                && (!check_if_index_exists(&numbers, &(i as i32 + 1)) || numbers[i + 1][j] > *digit)
                && (!check_if_index_exists(&numbers, &(i as i32 - 1)) || numbers[i - 1][j] > *digit)
            {
                risk_level += *digit + 1;
                low_points.push((i, j));
            }
        }
    }

    println!("P1 Solution: {}", risk_level);

    let mut counts: Vec<usize> = Vec::new();
    let bound: usize = 9;
    let mut paint_fill: PaintFill = PaintFill {
        count: 0,
        visited: HashMap::new(),
    };
    for low_point in low_points.iter() {
        paint_fill.fill(
            low_point.0,
            low_point.1,
            &numbers,
            //&mut count,
            //&mut visited,
            bound,
        );
        counts.push(paint_fill.reset_and_return_count());
    }

    counts.sort_unstable();
    let mut result = counts.iter().rev();
    println!(
        "P2 Solution: {}",
        result.next().unwrap() * result.next().unwrap() * result.next().unwrap()
    );
}

//count: usize = 0;
//visited: HashMap<String, bool> = HashMap::new();

struct PaintFill {
    count: usize,
    visited: HashMap<String, bool>,
}

impl PaintFill {
    pub fn reset_and_return_count(&mut self) -> usize {
        let temp = self.count;
        self.count = 0;
        temp
    }

    pub fn fill<T: Eq + Copy>(&mut self, i: usize, j: usize, arr: &[Vec<T>], bound: T) {
        let concat = |s1: &usize, s2: &usize| s1.to_string() + "," + s2.to_string().as_str();
        let id = &concat(&i, &j);

        if self.visited.contains_key(id) {
            return;
        }

        self.visited.insert(id.to_owned(), true);
        self.count += 1;

        let mut neighbours: Vec<(usize, usize)> = Vec::new();
        if check_if_index_exists(&arr[i], &(j as i32 + 1)) && arr[i][j + 1] != bound {
            neighbours.push((i, j + 1));
        }
        if check_if_index_exists(&arr[i], &(j as i32 - 1)) && arr[i][j - 1] != bound {
            neighbours.push((i, j - 1));
        }
        if check_if_index_exists(arr, &(i as i32 + 1)) && arr[i + 1][j] != bound {
            neighbours.push((i + 1, j));
        }
        if check_if_index_exists(arr, &(i as i32 - 1)) && arr[i - 1][j] != bound {
            neighbours.push((i - 1, j));
        }

        for neighbour in &mut neighbours {
            self.fill(neighbour.0, neighbour.1, arr, bound)
        }
    }
}

fn check_if_index_exists<T>(vec: &[T], index: &i32) -> bool {
    index < &(vec.len() as i32) && index > &-1
}
