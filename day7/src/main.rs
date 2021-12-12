use std::fs;
use stats::median;
use stats::mean;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let numbers = data.split(',').map(|a: &str| a.to_string().parse::<i32>().expect("Could not parse"));

    let median = median(numbers.clone()).unwrap();

    let mut fuel_count = 0;

    for number in numbers.clone(){
        fuel_count += (median as i32 - number).abs();
    }

    println!("P1 Solution: {}", fuel_count);

    fuel_count=0;
    let mean = mean(numbers.clone()) as i32;
    for number in numbers.to_owned(){
        let count= (mean - number).abs();
        for i in 1..count+1{
            fuel_count+=i;
        }
    }
    println!("P2 Solution: {}", fuel_count);
} 