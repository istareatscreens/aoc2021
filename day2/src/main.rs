use std::fs;
//use std::vec::Vec;

fn main() {
    // --snip--
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //println!("File contents:\n\n{}", data);
    let mut horizontal = 0;
    let mut verticle = 0;

    for n in data.lines() {
        let mut line = n.split_whitespace();
        let direction = line
            .next()
            .as_ref()
            .map(|x| &**x)
            .unwrap_or("default string");
        let amount = line.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => horizontal += amount,
            "down" => verticle += amount,
            "up" => verticle -= amount,
            _ => println!("error"),
        };
    }

    println!("\n\n Result P1: {}\n\n", horizontal * verticle);

    let mut horizontal = 0;
    let mut verticle = 0;
    let mut aim = 0;

    for n in data.lines() {
        let mut line = n.split_whitespace();
        let direction = line
            .next()
            .as_ref()
            .map(|x| &**x)
            .unwrap_or("default string");
        let amount = line.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {
                horizontal += amount;
                verticle += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => println!("error"),
        };
    }

    println!("\n\n Result P2: {}\n\n", horizontal * verticle);
}
