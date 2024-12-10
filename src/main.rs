use advent_of_code2024::*;
use std::env;
use std::time::Instant;

fn run_for_day(day_no: i32) {
    match day_no {
        1 => {
            run_measured(day1);
        }
        2 => {
            run_measured(day2);
        }
        3 => {
            run_measured(day3);
        }
        4 => {
            run_measured(day4);
        }
        5 => {
            run_measured(day5);
        }
        6 => {
            run_measured(day6);
        }
        7 => {
            run_measured(day7);
        }
        8 => {
            run_measured(day8);
        }
        9 => {
            run_measured(day9);
        }
        10 => {
            run_measured(day10);
        }
        _ => {
            println!("Day {} is far or I don't know how to solve :)", { day_no });
        }
    }
}

fn run_measured(f: fn()) {
    let before = Instant::now();
    f();
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_no = args[1].parse().unwrap();
    run_for_day(day_no);
}
