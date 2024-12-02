use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_file(name: &str) -> Vec<String> {
    let path = Path::new(name);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<_> = reader.lines().flatten().collect();
    return lines;
}

pub fn day1() {
    println!("Solving day 1 problems");
    let lines = read_file("day1.txt");
    let n = lines.len();
    let (mut arr1, mut arr2) = (vec![], vec![]);
    for l in lines {
        let v: Vec<_> = l.split_whitespace().collect();
        arr1.push(v[0].parse::<i32>().unwrap());
        arr2.push(v[1].parse::<i32>().unwrap());
    }
    arr1.sort();
    arr2.sort();
    let mut h = HashMap::new();
    let mut dist: i64 = 0;
    for i in 0..n {
        //println!("{} {} {}", arr1[i], arr2[i], (arr1[i] - arr2[i]).abs());
        dist += i64::from((arr1[i] - arr2[i]).abs());
        match h.get(&arr2[i]) {
            Some(count) => {
                h.insert(arr2[i], count + 1);
            }
            None => {
                h.insert(arr2[i], 1);
            }
        }
    }
    let mut similarty: i64 = 0;
    for i in 0..n {
        match h.get(&arr1[i]) {
            None => {}
            Some(value) => {
                similarty += i64::from(value * arr1[i]);
            }
        }
    }
    println!("{}", dist);
    println!("{}", similarty);
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let n = report.len();
    for check_increasing in [true, false] {
        let (mut x, mut y);
        let mut safe = true;
        for i in 1..n {
            if check_increasing {
                (x, y) = (report[i - 1], report[i]);
            } else {
                (x, y) = (report[i], report[i - 1]);
            }
            if x < y {
                let delta = y - x;
                if delta >= 1 && delta <= 3 {
                    continue;
                } else {
                    safe = false;
                }
            } else {
                safe = false;
            }
        }
        if safe {
            return true;
        }
    }
    return false;
}

fn is_safe_with_one_deletion(report: &Vec<i32>) -> bool {
    let l = report.len();
    if is_safe_report(report) {
        return true;
    }
    for i in 0..l {
        let mut v = report[0..i].to_vec();
        v.extend_from_slice(&report[i + 1..l]);
        if is_safe_report(&v) {
            //println!("{:?}", report);
            return true;
        }
    }
    return false;
}

pub fn day2() {
    println!("Solving day 2 problems");
    let lines = read_file("day2.txt");
    let mut safe_count: i32 = 0;
    let mut safe_with_one_deletion: i32 = 0;
    for l in lines {
        let v: Vec<i32> = l
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if is_safe_report(&v) {
            //println!("{:?}", v);
            safe_count += 1;
        }
        if is_safe_with_one_deletion(&v) {
            safe_with_one_deletion += 1;
        }
    }
    println!("{}", safe_count);
    println!("{}", safe_with_one_deletion);
}
