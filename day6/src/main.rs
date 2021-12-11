use std::fs;
use std::vec::Vec;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let numbers: Vec<usize> = data.split(',').map(|a: &str| a.to_string().parse::<usize>().expect("Could not parse")).collect();

    println!("P1 Solution: {}", compute_fish(&mut numbers.clone(), 80));
    
    let mut fish_count: Vec<u128> = vec![0;9];
    for number in numbers{
        fish_count[number] += 1;
    }
    println!("fish_count: {:?}",fish_count);

    for _ in 0..256{
            let temp: u128 = fish_count[0].to_owned(); 
            for j in 0..fish_count.len(){
                if j + 1 == fish_count.len(){
                   fish_count[6] += temp; 
                   fish_count[8] = temp;
                }else{
                    fish_count[j] = fish_count[j+1]
                }
            }
            println!("AFTER ARRAY SIZE: {:?}", fish_count);
    }


    println!("fish_count: {:?}",fish_count);
    let mut count = 0;
    for number in fish_count{
       count+=number; 
    }

    println!("P2 Solution: {}", count);
}

fn compute_fish(numbers: &mut Vec<usize>, days: usize) -> usize{
    for _ in 0..days{
        for j in 0..numbers.len(){
            if numbers[j] == 0{
                numbers[j] = 6;
                numbers.push(8);
            }else{
                numbers[j]-=1;
            }
        }
    }
    numbers.len()
}