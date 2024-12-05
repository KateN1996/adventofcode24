use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    day2()?;

    Ok(())
}

fn day2() -> Result<(), Box<dyn Error>> {
    let path = "/Users/katenixon/Documents/GitHub/adventofcode24/day1/src/day2.txt";
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);
    let mut safe_reports = 0;

    for line in reader.lines() {
        let line = line?;

        let elements: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if elements.len() < 2 || elements[0] == elements[1] {
            continue;
        }

        let increasing = elements[0] < elements[1];

        let follows_pattern = elements
            .windows(2)
            .all(|pair| {
                let diff = pair[1] - pair[0];
                if increasing {
                    diff > 0 && diff <= 3
                } else {
                    diff < 0 && diff >= -3
                }
            });

        if follows_pattern {
            safe_reports += 1;
        }
    }

    println!("safe reports: {}", safe_reports);

    Ok(())
}

fn day1() -> Result<(), Box<dyn Error>> {
    let path = "/Users/katenixon/Documents/GitHub/adventofcode24/day1/src/input.txt";
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut split_line = line.split_whitespace();

        if let (Some(left_val), Some(right_val)) = (split_line.next(), split_line.next()) {
            match (left_val.parse::<i32>(), right_val.parse::<i32>()) {
                (Ok(l), Ok(r)) => {
                    left.push(l);
                    right.push(r);
                }
                _ => {
                    eprintln!("Failed to parse numbers in line: {}", line);
                }
            }
        }
    }

    left.sort();
    right.sort();

    day1pt1(&left, &right);
    day1pt2(&left, &right);

    Ok(())
}

fn day1pt2(left: &[i32], right: &[i32]) {
    let mut similarity_score = 0;
    for &l in left {
        let count = right.iter().filter(|&&r| r == l).count();
        similarity_score += l * count as i32;
    }

    println!("Similarity score is: {}", similarity_score);
}

fn day1pt1(left: &[i32], right: &[i32]) {
    let mut total_distance = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        if l > r {
            total_distance += l - r;
        } else {
            total_distance += r - l;
        }
    }

    println!("\nTotal distance is: {}", total_distance);
}
