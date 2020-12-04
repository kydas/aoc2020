use std::fs;
use std::collections::HashMap;

pub fn day1(part: &str) -> i32 {
    let contents = fs::read_to_string("src/day1/input1.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut input = Vec::new();

    for line in lines {
        let curr: i32 = line.parse().unwrap();
        input.push(curr);
    }
    if part == "a" {
        match two_sum(&input, 2020) {
            Ok((f,s)) => f * s,
            Err(e) => panic!("{}!", e)
        }
    } else {
        match three_sum(&input) {
            Ok((f,s,t)) => f * s * t,
            Err(e) => panic!("{}!", e)
        }
    }

    
}

fn two_sum(numbers: &Vec<i32>, sum: i32) -> Result<(i32, i32), &'static str> {
    let mut diff: HashMap<i32, i32> = HashMap::new();
    for int in numbers {
        if diff.contains_key(&int) {
            let val = diff.get(&int).unwrap();
            return Ok((*int, *val));
        } else {
            diff.insert(sum - int, *int);
        }
    }
    Err("Solution cannot be found")
}

fn three_sum(numbers: &Vec<i32>) -> Result<(i32, i32, i32), &'static str> {
    for int in numbers {
        match two_sum(&numbers, 2020 - int) {
            Ok((f,s)) => return Ok((f,s, *int)),
            Err(_e) => ()
        }
    }
    Err("Solution cannot be found")
}