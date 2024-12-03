use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

fn unusual_data() {
    let file_path = "input1.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut result = 0;

    for line in contents.lines() {
        let values: Vec<&str> = line.split(' ').collect();
        let v0 = values[0].parse::<i32>().unwrap();
        let v1 = values[1].parse::<i32>().unwrap();
        let seq_neg = v1 - v0 < 0;
        if (v1 - v0).abs() > 3 || (v1 - v0).abs() < 1 {
            continue;
        }
        for i in 2..values.len() {
            let v_next = values[i].parse::<i32>().unwrap();
            let v_prev = values[i - 1].parse::<i32>().unwrap();
            if (v_next - v_prev).abs() > 3 || (v_next - v_prev).abs() < 1 {
                break;
            }
            if (v_next - v_prev > 0 && seq_neg) || (v_next - v_prev < 0 && !seq_neg) {
                break;
            }
            if i == values.len() - 1 {
                result += 1;
            }
        }
        // println!("Got: {:?} {:?}", intdata1.unwrap(), intdata2.unwrap());
    }

    println!("Result:\n{result}");
}

fn dp(data: &Vec<&str>, used_dumper: &mut bool, seq_neg: &mut bool, idx1: usize, idx2: usize) -> bool {
    // println!("idx1 {idx1}");
    // println!("idx2:\n{idx2}");
    // println!("Break:\n");
    if idx1 + 1 >= data.len() {
        return true;
    }
    //     println!("idx1 {idx1}");
    // println!("idx2:\n{idx2}");
    // println!("Break:\n");

    if idx2 + 1 == data.len() {
        let v_next = data[idx2].parse::<i32>().unwrap();
        let v_prev = data[idx1].parse::<i32>().unwrap();

        if (v_next - v_prev).abs() > 3 || (v_next - v_prev).abs() < 1 ||
        (v_next - v_prev > 0 && *seq_neg) || (v_next - v_prev < 0 && !*seq_neg) {
            if !*used_dumper {
                return true;
                // *seq_neg = v_next - v_prev < 0;
            } else {
                return false;
            }
        }
        return true;
    }
    // println!("idx1 {idx1}");
    // println!("idx2:\n{idx2}");
    // println!("Break:\n");


    let v_next = data[idx2].parse::<i32>().unwrap();
    let v_prev = data[idx1].parse::<i32>().unwrap();

    if idx1 == 0 {
        *seq_neg = v_next - v_prev < 0;
    }
    if *used_dumper && idx1 == 1 {
        *seq_neg = v_next - v_prev < 0;
    }

    if (v_next - v_prev).abs() > 3 || (v_next - v_prev).abs() < 1 ||
    (v_next - v_prev > 0 && *seq_neg) || (v_next - v_prev < 0 && !*seq_neg) {
        if !*used_dumper {
            *used_dumper = true;
            // *seq_neg = v_next - v_prev < 0;
        } else {
            return false;
        }
    }

    let mut ret1 = dp(data, used_dumper, seq_neg, idx2, idx2 + 1);
    // println!("ret1 prev {ret1}");
    if !*used_dumper && !ret1 {
        *seq_neg = v_next - v_prev < 0;
        *used_dumper = true;
        // println!("ret1 {ret1}");
        ret1 = dp(data, used_dumper, seq_neg, idx2, idx2 + 2);
        // println!("ret1 2 {ret1}");
        if !ret1 {
            ret1 = dp(data, used_dumper, seq_neg, idx2 + 1, idx2 + 2);
            // println!("ret1 3 {ret1}");
        }
    }

    ret1
}

fn problem_dumper() {
    // let file_path = "/Users/minchenk/Documents/AdventOfCode24/RustAdvent/Day2/src/foo.txt";
    let file_path = "input1.txt";
    println!("In file {file_path}");
    // let mut file = File::create("foo.txt").expect("SDsd");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut result = 0;

    for line in contents.lines() {
        let values: Vec<&str> = line.split(' ').collect();
        let mut used_dumper = false;
        let mut seq_neg = false;

        let mut res = dp(&values, &mut used_dumper, &mut seq_neg, 0, 1);
        if res == false {
            used_dumper = true;
            seq_neg = false;
            res = dp(&values, &mut used_dumper, &mut seq_neg, 1, 2);

            if res == false {
                used_dumper = true;
                seq_neg = false;
                res = dp(&values, &mut used_dumper, &mut seq_neg, 0, 2);
            }

        }
        // println!("res:{res}\n");
        
        if res == true {
            result += 1;
        } else {
            // file.write_all(line.as_bytes());
            // file.write_all("\n".as_bytes());
        }       
        
    }

    println!("Result:\n{result}");
}


fn main() {
    problem_dumper();
}