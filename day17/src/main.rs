use log::debug;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[allow(clippy::comparison_chain)]
fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");
    let re = Regex::new(r"[\d|-]+").unwrap();

    let matches: Vec<_> = re
        .captures_iter(&data)
        .map(|a| a.get(0).unwrap().as_str().parse::<i64>().unwrap())
        .collect();
    debug!("targets: {:?}", matches);

    let check_x_past = |pos: i64, goal: (i64, i64), velocity: i64| pos > goal.1 || velocity == 0;
    let check_y_past = |pos: i64, goal: (i64, i64), _: i64| pos < goal.0;

    let calculate_x_velocity = |velocity: &i64| {
        if *velocity > 0 {
            *velocity - 1
        } else if *velocity < 0 {
            *velocity + 1
        } else {
            0
        }
    };

    let calculate_y_velocity = |velocity: &i64| *velocity - 1;

    let range_x = if matches[0] < matches[1] {
        (matches[0], matches[1])
    } else {
        (matches[1], matches[0])
    };

    let range_y = if matches[2] < matches[3] {
        (matches[2], matches[3])
    } else {
        (matches[3], matches[2])
    };
    let p_x = get_parameters(&14, range_x, check_x_past, calculate_x_velocity);
    let p_y = get_parameters(&-10, range_y, check_y_past, calculate_y_velocity);
    debug!("RESULT: \n{:?},\n{:?}", p_x, p_y);

    let mut valid_coordinates: HashSet<String> = HashSet::new();
    let mut x_0_velocity = 0;
    let mut y_0_velocity = range_y.0 - 100;
    let mut valid_shots_x: Vec<Parameters> = Vec::new();
    let mut valid_shots_y: Vec<Parameters> = Vec::new();
    while y_0_velocity < range_y.1.abs() + 100 {
        y_0_velocity += 1;
        while x_0_velocity < range_x.1 + 10 {
            x_0_velocity += 1;
            let p_x = get_parameters(&x_0_velocity, range_x, check_x_past, calculate_x_velocity);
            let p_y = get_parameters(&y_0_velocity, range_y, check_y_past, calculate_y_velocity);

            let mut hit_target = false;

            let min_y_step = p_y.good_steps.iter().max();
            let min_x_step = p_x.good_steps.iter().min();

            if p_x.hit_target && p_y.hit_target.to_owned() {
                for number in p_y.good_steps.iter() {
                    if p_x.good_steps.contains_key(number.0) {
                        hit_target = *p_x.good_steps.get(number.0).unwrap()
                            && *p_y.good_steps.get(number.0).unwrap();
                        if hit_target {
                            break;
                        }
                    }
                }

                if min_x_step < min_y_step && p_x.reached_zero {
                    hit_target = true;
                }

                if hit_target {
                    valid_shots_x.push(p_x);
                    valid_shots_y.push(p_y);
                    valid_coordinates
                        .insert(x_0_velocity.to_string() + "," + y_0_velocity.to_string().as_str());
                }
            }
        }
        x_0_velocity = 0;
    }

    /*
    for i in 0..valid_shots_x.len() {
        debug!(
            "valid shots: {},{}\n",
            valid_shots_x[i].max_position, valid_shots_y[i].max_position
        );
    }
    */

    valid_shots_y.sort_by_key(|k| k.max_position);
    println!(
        "P1 Solution: {}",
        valid_shots_y.last().unwrap().max_position
    );
    for cord in valid_coordinates.iter() {
        debug!("valid coordinates: {:?}", cord);
    }

    println!("P2 Solution: {}", valid_coordinates.len());
}

#[derive(Debug)]
struct Parameters {
    pub good_steps: HashMap<usize, bool>,
    pub velocity_0: i64,
    pub reached_zero: bool,
    pub max_position: i64,
    pub hit_target: bool,
}

fn get_parameters<T, F>(
    velocity_0: &i64,
    goal_range: (i64, i64),
    eval: T,
    calculate_velocity: F,
) -> Parameters
where
    T: Fn(i64, (i64, i64), i64) -> bool,
    F: Fn(&i64) -> i64,
{
    let mut position: i64 = 0;
    let mut step = 0;
    let mut max_position = i64::MIN;
    let mut velocity = *velocity_0;
    let mut hit_target = false;
    let mut good_steps: HashMap<usize, bool> = HashMap::new();
    let mut last_positions: Vec<i64> = Vec::new();
    let mut reached_zero = false;
    loop {
        position += velocity;
        velocity = calculate_velocity(&velocity);
        if !hit_target && position > max_position {
            max_position = position;
        }
        if position <= goal_range.1 && position >= goal_range.0 || hit_target {
            good_steps.insert(step, position <= goal_range.1 && position >= goal_range.0);
            hit_target = true;
            last_positions.push(position);
            if position <= goal_range.1 && position >= goal_range.0 && velocity == 0 {
                reached_zero = true;
            }
        }

        if eval(position, goal_range, velocity) {
            return Parameters {
                reached_zero,
                good_steps,
                velocity_0: *velocity_0,
                max_position,
                hit_target,
            };
        }
        step += 1;
    }
}
