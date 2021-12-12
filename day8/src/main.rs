use std::{char, fs};

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let input_output: Vec<Vec<_>> = data
        .lines()
        .map(|a: &str| {
            a.to_string()
                .split('|')
                .map(|a: &str| a.to_string())
                .collect()
        })
        .collect();

    let mut inputs: Vec<Vec<String>> = Vec::new();
    let mut outputs: Vec<Vec<String>> = Vec::new();
    for line in input_output {
        inputs.push(
            line[0]
                .split_whitespace()
                .map(|a: &str| a.to_string())
                .collect(),
        );
        outputs.push(
            line[1]
                .split_whitespace()
                .map(|a: &str| a.to_string())
                .collect(),
        );
    }

    let mut count = 0;
    for line in outputs.iter() {
        for chars in line.iter() {
            match chars.len() {
                7 => count += 1, //8
                2 => count += 1, //1
                3 => count += 1, //7
                4 => count += 1,
                _ => (),
            }
        }
    }
    println!("P1 Solution: {}", count);

    let mut sum: u64 = 0;
    for i in 0..inputs.len() {
        let digits = decode_input(&inputs[i]);
        sum += decode_output(&outputs[i], &digits)
    }

    println!("P2 Solution: {}", sum);
}

fn decode_output(outputs: &[String], digits: &[Vec<char>]) -> u64 {
    let mut output_digits: Vec<String> = Vec::new();
    for c in outputs.iter() {
        for (j, digit) in digits.iter().enumerate() {
            if compare_code::<char>(&c.chars().collect::<Vec<_>>(), digit) {
                output_digits.push(j.to_string());
            }
        }
    }
    output_digits
        .concat()
        .parse::<u64>()
        .expect("error parsing in decode output")
}

fn compare_code<T: Copy + Eq>(arr1: &[T], arr2: &[T]) -> bool {
    let mut count: usize = 0;

    if arr2.len() != arr1.len() {
        return false;
    }

    for element1 in arr1.iter() {
        for element2 in arr2.iter() {
            if element1 == element2 {
                count += 1;
                break;
            }
        }
    }
    count == arr1.len()
}

fn decode_input(inputs: &[String]) -> Vec<Vec<char>> {
    let mut letters: Vec<char> = vec!['-'; 7];
    let mut digits: Vec<Vec<char>> = vec![Vec::new(); 10];
    let mut unknown_digits: Vec<Vec<Vec<char>>> = vec![Vec::new(); 6];

    // TODO: Insert loop around everything
    for display in inputs.iter() {
        let chars = display.chars().collect::<Vec<_>>();
        //println!("DIGITS: {:?}, LEN: {}", chars, chars.len());
        match display.len() {
            7 => digits[8] = chars, //8
            2 => digits[1] = chars, //1
            3 => digits[7] = chars, //7
            4 => digits[4] = chars, //4
            _ => unknown_digits[chars.len() - 1].push(chars),
        }
    }

    /*
       for i in 0..digits.len() {
           println!("Digits row: {} | {:?}", i, digits[i]);
       }

       for i in 0..unknown_digits.len() {
           println!("Unknown Digits row: {} | {:?}", i, unknown_digits[i]);
       }
    */
    // compare 1 and 7 to find letter 0
    letters[0] = subtract::<char>(&digits[7], &digits[1]).pop().unwrap();

    //Compare 4 with 6 lengthed strings and 0 -> find one difference gives 9 and letter 6
    let mut temp = digits[4].clone();
    temp.push(letters[0]);

    for i in 0..unknown_digits[5].len() {
        let intersect = subtract::<char>(&unknown_digits[5][i], &temp);
        if intersect.len() == 1 {
            letters[6] = intersect[0];
            digits[9] = unknown_digits[5][i].clone();
            unknown_digits[5].remove(i);
            break;
        }
    }

    //Compare 9 and 8 differences is letter 4
    letters[4] = subtract::<char>(&digits[8], &digits[9])[0];

    //Search 5 length string and look for letter 4 that is 2
    'outer: for i in 0..unknown_digits[4].len() {
        for c in unknown_digits[4][i].iter() {
            if *c == letters[4] {
                digits[2] = unknown_digits[4][i].clone();
                unknown_digits[4].remove(i);
                break 'outer;
            }
        }
    }

    // Use 1 to find rest of digits
    for (i, digit) in unknown_digits[4].iter().enumerate() {
        temp = subtract(digit, &digits[1]);
        if temp.len() == 3 {
            digits[3] = digit.clone();
            unknown_digits[4].remove(i);
            break;
        }
    }

    digits[5] = unknown_digits[4][0].clone();
    unknown_digits[4].remove(0);

    for (i, digit) in unknown_digits[5].iter().enumerate() {
        temp = subtract(digit, &digits[1]);
        if temp.len() == 4 {
            digits[0] = digit.clone();
            unknown_digits[5].remove(i);
            break;
        }
    }

    digits[6] = unknown_digits[5][0].clone();
    unknown_digits[5].remove(0);

    /*
       for (i, digit) in digits.iter().enumerate() {
           println!("number: {} digit codes: {:?}", i, digit);
       }
    */

    digits
}

fn subtract<T: Copy + Eq>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let mut diff: Vec<T> = Vec::new();
    for element1 in arr1.iter() {
        let mut found = false;
        for element2 in arr2.iter() {
            if element1 == element2 {
                found = true;
                break;
            }
        }
        if !found {
            diff.push(*element1);
        }
    }
    diff
}
