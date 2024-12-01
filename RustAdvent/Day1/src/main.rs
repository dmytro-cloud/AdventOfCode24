use std::env;
use std::fs;
use std::collections::HashMap;

fn list_difference() {
    let file_path = "/Users/minchenk/Documents/Day1/src/input1.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut vec1 = Vec::<i32>::new();
    let mut vec2 = Vec::<i32>::new();
    for line in contents.lines() {
        let values: Vec<&str> = line.split("   ").collect();
        let intdata1 =  values[0].parse::<i32>();
        let intdata2 =  values[1].parse::<i32>();
        vec1.push(intdata1.unwrap());
        vec2.push(intdata2.unwrap());
        // println!("Got: {:?} {:?}", intdata1.unwrap(), intdata2.unwrap());
    }
    vec1.sort();
    vec2.sort();
    let mut result: i32 = 0;
    for i in 0..vec1.len() {
        result += (vec1[i] - vec2[i]).abs();
    }
    println!("Result:\n{result}");
}

fn similarity_score() {
    let file_path = "/Users/minchenk/Documents/Day1/src/input1.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut vec1 = Vec::<i32>::new();
    let mut vec2 = Vec::<i32>::new();
    for line in contents.lines() {
        let values: Vec<&str> = line.split("   ").collect();
        let intdata1 =  values[0].parse::<i32>();
        let intdata2 =  values[1].parse::<i32>();
        vec1.push(intdata1.unwrap());
        vec2.push(intdata2.unwrap());
    }

    let mut scores = HashMap::new();

    for val in vec2 {
        let count = scores.entry(val).or_insert(0);
        *count += 1;
    }

    let mut result_similarity: i32 = 0;
    for val in vec1 {
        let score = scores.get(&val).copied().unwrap_or(0);
        result_similarity += val * score;
    }

    println!("Score:{result_similarity}");
}

fn main() {
    similarity_score();
}