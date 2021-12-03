use std::fs;
use std::vec::Vec;

fn main() {
    // --snip--
    let data = fs::read_to_string("i1.txt").expect("Unable to read file");
    let mut previous = -1;
    //println!("File contents:\n\n{}", data);
    let mut count = 0;

    for n in data.lines() {
        let number = n.to_string().parse::<i32>().unwrap();
        if number > previous && previous != -1 {
            count += 1;
            println!("increase: {}", number);
        } else {
            println!("decrease: {}", number);
        }
        previous = number;
    }

    println!("\n\n Result P1: {}\n\n", count);

    count = 0;
    let mut array: Vec<_> = data
        .lines()
        .map(|s: &str| s.to_string().parse::<i32>().unwrap())
        .collect();

    print!("length {}", array.len());
    for i in 0..array.len() {
        if i + 3 >= array.len() {
            break;
        }

        let window1 = window(&mut array, i);
        let window2 = window(&mut array, i + 1);

        if window1 < window2 {
            count += 1;
        }
    }
    println!("\n\n Result P2: {}\n\n", count);
}

fn window(array: &mut Vec<i32>, index: usize) -> i32 {
    return array[index] + array[index + 1] + array[index + 2];
}
