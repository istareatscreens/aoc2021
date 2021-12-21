use min_max::*;
use std::fs;
use std::vec::Vec;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let lines: Vec<String> = data.lines().map(|a: &str| a.to_string()).collect();

    let mut line_coordinates: Vec<([i32; 2], [i32; 2])> = Vec::new();
    for line in &lines {
        let parts: Vec<String> = line
            .split_whitespace()
            .map(|a: &str| a.to_string())
            .collect();
        let start: Vec<_> = parts[0]
            .split(',')
            .map(|a: &str| {
                a.to_string()
                    .parse::<i32>()
                    .expect("Could not parse number")
            })
            .collect();
        let end: Vec<_> = parts[2]
            .split(',')
            .map(|a: &str| {
                a.to_string()
                    .parse::<i32>()
                    .expect("Could not parse number")
            })
            .collect();
        line_coordinates.push(([start[0], start[1]], [end[0], end[1]]));
    }
    //println!("Data: {:?}", line_coordinates);

    let mut map: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    //println!("lines: {:?}\ninput: {:?}", lines, line_coordinates);
    for line_coordinate in &line_coordinates {
        if line_coordinate.0[0] == line_coordinate.1[0] {
            for j in min!(line_coordinate.0[1], line_coordinate.1[1])
                ..(max!(line_coordinate.0[1], line_coordinate.1[1]) + 1)
            {
                map[j as usize][line_coordinate.1[0] as usize] += 1;
            }
        }
        if line_coordinate.0[1] == line_coordinate.1[1] {
            for j in min!(line_coordinate.0[0], line_coordinate.1[0])
                ..(max!(line_coordinate.1[0], line_coordinate.0[0]) + 1)
            {
                map[line_coordinate.1[1] as usize][j as usize] += 1;
            }
        }
        if line_coordinate.0[1] == line_coordinate.1[1]
            && line_coordinate.0[0] == line_coordinate.1[0]
        {
            map[line_coordinate.0[1] as usize][line_coordinate.1[0] as usize] += 1;
        }
    }

    println!("P1 Solution: {}", analyze_map(&map));

    map = vec![vec![0; 1000]; 1000];
    for line_coordinate in &line_coordinates {
        if line_coordinate.0[0] == line_coordinate.1[0] {
            for j in min!(line_coordinate.0[1], line_coordinate.1[1])
                ..(max!(line_coordinate.0[1], line_coordinate.1[1]) + 1)
            {
                map[j as usize][line_coordinate.1[0] as usize] += 1;
            }
        } else if line_coordinate.0[1] == line_coordinate.1[1] {
            for j in min!(line_coordinate.0[0], line_coordinate.1[0])
                ..(max!(line_coordinate.1[0], line_coordinate.0[0]) + 1)
            {
                map[line_coordinate.1[1] as usize][j as usize] += 1;
            }
        } else {
            let mut x = line_coordinate.0[0];
            let mut y = line_coordinate.0[1];
            for _ in min!(line_coordinate.0[1], line_coordinate.1[1])
                ..(max!(line_coordinate.0[1], line_coordinate.1[1]) + 1)
            {
                map[y as usize][x as usize] += 1;
                if line_coordinate.1[0] > line_coordinate.0[0] {
                    x += 1;
                } else {
                    x -= 1;
                }
                if line_coordinate.1[1] > line_coordinate.0[1] {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }
    /*
        println!("MAP:");
        for line in &map {
            println!("{:?}", line)
        }
    */
    println!("P2 Solution: {}", analyze_map(&map));
}

fn analyze_map(map: &[Vec<usize>]) -> i32 {
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] > 1 {
                count += 1;
            }
        }
    }
    count
}
