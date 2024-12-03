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
    }

    println!("Result:\n{result}");
}

fn dp(data: &Vec<&str>, used_dumper: &mut bool, seq_neg: &mut bool, idx1: usize, idx2: usize) -> bool {
    if idx2 + 1 == data.len() {
        let v_next = data[idx2].parse::<i32>().unwrap();
        let v_prev = data[idx1].parse::<i32>().unwrap();

        if (v_next - v_prev).abs() > 3 || (v_next - v_prev).abs() < 1 ||
        (v_next - v_prev > 0 && *seq_neg) || (v_next - v_prev < 0 && !*seq_neg) {
            if !*used_dumper {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    let v_next = data[idx2].parse::<i32>().unwrap();
    let v_prev = data[idx1].parse::<i32>().unwrap();

    if idx1 == 0 {
        *seq_neg = v_next - v_prev < 0;
    }
    if *used_dumper && idx1 == 1 && idx2 == 2 {
        *seq_neg = v_next - v_prev < 0;
    }


    if (v_next - v_prev).abs() > 3 || (v_next - v_prev).abs() < 1 ||
    (v_next - v_prev > 0 && *seq_neg) || (v_next - v_prev < 0 && !*seq_neg) {
        if !*used_dumper {
            *used_dumper = true;
            let mut ret1 = dp(data, used_dumper, seq_neg, idx1, idx1 + 2);

            if !ret1 && idx1 + 1 < data.len() && idx1 != 0 {
                let vnext_2 = data[idx1 + 1].parse::<i32>().unwrap();
                let vprev_2 = data[idx1 - 1].parse::<i32>().unwrap();
                if !((vnext_2 - vprev_2).abs() > 3) && !((vnext_2 - vprev_2).abs() < 1) 
                 && !(vnext_2 - vprev_2 > 0 && *seq_neg) && !(vnext_2 - vprev_2 < 0 && !*seq_neg) {
                    ret1 = dp(data, used_dumper, seq_neg, idx1 + 1, idx1 + 2);
                } 
            }
            return ret1;
        } else {
            return false;
        }
    }

    let ret1 = dp(data, used_dumper, seq_neg, idx2, idx2 + 1);
    ret1
}

fn problem_dumper() {
    let file_path = "input1.txt";
    println!("In file {file_path}");

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
        if res == true {
            result += 1;
        }    
        
    }

    println!("Result:\n{result}");
}


fn main() {
    problem_dumper();
}
