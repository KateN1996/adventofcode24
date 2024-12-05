use std::fs::File;
use std::io::{self, BufRead};
fn main() -> io::Result<()>{
    
    let path = "/Users/katenixon/Documents/GitHub/adventofcode24/day1/src/input.txt";
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines(){
        let line = line?;
        let mut split_line =  line.split_whitespace();

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

    // day1(left, right);
    day2(&left,&right);


    Ok(())

}

fn day2(left:&Vec<i32>, right:&Vec<i32>){
    let mut similarity_score = 0;
    for &l in left{
        let count = right.iter().filter(|&&r|r == l).count();
        similarity_score += l * count as i32;
    }

    print!("similarity score is {}",similarity_score)

}

fn day1(left:&Vec<i32>, right:&Vec<i32>){
    let mut total_distance = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        if l > r{
            total_distance += l - r;
        }else{
            total_distance += r - l;
        }
    
    }

    println!("/n total distance is: {}", total_distance);
}
