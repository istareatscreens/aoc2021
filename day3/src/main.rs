use std::fs;
use std::vec::Vec;

#[allow(clippy::all)]
fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");
    //println!("File contents:\n\n{}", data);
    let mut count = [0; 12];
    let mut lines = 0;

    for line in data.lines() {
        lines += 1;
        let mut number = bit_str_to_int(line);
        for i in 0..line.len() {
            count[i] += 1 & number;
            number >>= 1;
        }
    }

    let mut gamma_str: String = "".to_string();
    for i in count {
        if lines / 2 < i {
            gamma_str = "1".to_string() + gamma_str.as_str();
        } else {
            gamma_str = "0".to_string() + gamma_str.as_str();
        }
    }
    println!("\n\n Gama: {}\n\n", gamma_str);

    let gamma = bit_str_to_int(gamma_str.as_str());
    let epsilon = gamma ^ 0b111111111111;

    println!("Gama: {}", gamma);
    println!("Epsilon: {}\n", epsilon);

    println!("\n\n Result P1: {}\n\n", gamma * epsilon);

    let o2_check = |majority_bit: usize, size: usize, bit_count: usize| {
        move |a: usize| {
            ((size as f32) / 2 as f32 == bit_count as f32 && a == 1)
                || (a == majority_bit && !((size as f32) / 2 as f32 == bit_count as f32))
        }
    };
    let co2_check = |majority_bit: usize, size: usize, bit_count: usize| {
        move |a: usize| {
            ((size as f32) / 2 as f32 == bit_count as f32 && a == 0)
                || (a != majority_bit && !((size as f32) / 2 as f32 == bit_count as f32))
        }
    };

    let numbers: Vec<_> = data.lines().map(|a: &str| a.to_string()).collect();

    let o2_sensor = find_sensor_reading(numbers.clone(), o2_check);
    let co2_sensor = find_sensor_reading(numbers.clone(), co2_check);
    println!("o2: {}, co2: {}", o2_sensor, co2_sensor);
    let result = o2_sensor * co2_sensor;

    println!("\n\n Result P2: {}\n\n", result);
}

#[allow(clippy::all)]
fn find_sensor_reading<T, F>(numbers: Vec<String>, eval: T) -> usize
where
    F: Fn(usize) -> bool,
    T: Fn(usize, usize, usize) -> F,
{
    let mut sensor_numbers = numbers.to_owned().clone();
    for i in 0..numbers[0].len() {
        let bit_count: Vec<usize> = get_bit_count(&sensor_numbers);
        let majority_bit: Vec<usize> = get_majority_bits(&sensor_numbers, &bit_count);
        /*
        println!(
            "numbers: {:?}, \nmajority: {:?}, \ncount: {:?}, \nlength: {} \nIndex: {}\n",
            sensor_numbers,
            majority_bit,
            bit_count,
            sensor_numbers.len(),
            i
        );
        */
        sensor_numbers = get_valid_numbers(
            &sensor_numbers,
            i,
            eval(majority_bit[i], sensor_numbers.len(), bit_count[i]),
        );
        if sensor_numbers.len() == 1 {
            return bit_str_to_int(sensor_numbers[0].as_str());
        }
    }

    return 0;
}

#[allow(clippy::all)]
fn get_bit_count(numbers: &Vec<String>) -> Vec<usize> {
    let mut bit_count: Vec<usize> = vec![0; numbers[0].len()];
    for number in numbers.iter() {
        for i in 0..number.len() {
            if number.chars().nth(i).unwrap() == '1' {
                bit_count[i] += 1;
            }
        }
    }

    bit_count
}

#[allow(clippy::all)]
fn get_majority_bits(numbers: &Vec<String>, bit_count: &Vec<usize>) -> Vec<usize> {
    let mut majority_bit: Vec<usize> = vec![0; numbers[0].len()];
    for i in 0..bit_count.len() {
        if bit_count[i] > numbers.len() / 2 {
            majority_bit[i] = 1;
        } else {
            majority_bit[i] = 0;
        }
    }
    majority_bit
}

#[allow(clippy::all)]
fn get_valid_numbers<T>(numbers: &Vec<String>, index: usize, eval: T) -> Vec<String>
where
    T: Fn(usize) -> bool,
{
    let mut valid_numbers: Vec<String> = Vec::new();

    for number in numbers.iter() {
        if eval(number.chars().nth(index).unwrap().to_digit(10).unwrap() as usize) {
            valid_numbers.push(number.to_owned());
        }
    }

    valid_numbers
}

fn bit_str_to_int(s: &str) -> usize {
    isize::from_str_radix(s, 2).unwrap() as usize
}
