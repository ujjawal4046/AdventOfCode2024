use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
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
    println!("Part 1 - {}", dist);
    println!("Part 2 - {}", similarty);
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
    println!("Part 1 - {}", safe_count);
    println!("Part 2 - {}", safe_with_one_deletion);
}

pub fn day3() {
    enum Symbol {
        Do,
        Dont,
        Mul,
        Invalid,
    }
    println!("Solving day 3 problems");
    let path = Path::new("day3.txt");
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(&file);
    let input = &mut "".to_string();
    reader.read_to_string(input).unwrap();

    for check_corrupted in [false, true] {
        let mut itr = input.chars().peekable();
        let mut answer: u32 = 0;
        let mut pre_symbol = Symbol::Do;
        let mut cur_symbol = Symbol::Invalid;
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut cur: u32 = 0;

        while let Some(c) = itr.peek() {
            if c.is_alphabetic() {
                let mut ident = String::new();
                while let Some(c) = itr.peek() {
                    if c.is_alphabetic() || c.eq(&'\'') {
                        ident.push(*c);
                        itr.next();
                    } else {
                        break;
                    }
                }
                if itr.peek().is_some() && itr.peek().unwrap().eq(&'(') {
                    match String::as_str(&ident) {
                        "do" => {
                            cur_symbol = Symbol::Do;
                        }
                        "don't" => {
                            cur_symbol = Symbol::Dont;
                        }
                        "mul" => {
                            cur_symbol = Symbol::Mul;
                        }
                        _ => {
                            cur_symbol = Symbol::Invalid;
                        }
                    }
                }
            } else if c.is_numeric() {
                while let Some(c) = itr.peek() {
                    if c.is_numeric() {
                        cur = cur * 10 + c.to_digit(10).unwrap();
                        itr.next();
                    } else {
                        break;
                    }
                }
                if itr.peek().is_some() {
                    if itr.peek().unwrap().eq(&',') {
                        x = cur;
                    } else if itr.peek().unwrap().eq(&')') {
                        y = cur;
                    } else {
                        cur_symbol = Symbol::Invalid;
                    }
                } else {
                    cur_symbol = Symbol::Invalid;
                }
                cur = 0;
            } else if c.eq(&')') {
                itr.next();
                match cur_symbol {
                    Symbol::Mul => {
                        if check_corrupted {
                            match pre_symbol {
                                Symbol::Do => {
                                    answer += x * y;
                                    //println!("{} {} {}", x, y, answer);
                                }
                                _ => {}
                            }
                        } else {
                            answer += x * y;
                        }
                    }
                    Symbol::Dont => {
                        pre_symbol = cur_symbol;
                    }
                    Symbol::Do => {
                        pre_symbol = cur_symbol;
                    }
                    _ => {}
                }
                cur_symbol = Symbol::Invalid;
            } else {
                itr.next();
            }
        }

        println!("Check corrupted={},answer={}", check_corrupted, answer);
    }
}

fn check_xmas(arr: &Vec<Vec<char>>, x: i32, y: i32, x_len: usize, y_len: usize) -> i32 {
    let mut ans = 0;
    let xd = [-1, 1, 0, 0, -1, 1, -1, 1];
    let yd = [0, 0, -1, 1, -1, 1, 1, -1];
    for i in 0..8 {
        let mut sat = true;
        for j in 0..4 {
            let x_ = x + xd[i] * j;
            let y_ = y + yd[i] * j;
            if x_ >= 0
                && x_ < x_len.try_into().unwrap()
                && y_ >= 0
                && y_ < y_len.try_into().unwrap()
            {
                let (xdix, ydix) = (x_ as usize, y_ as usize);
                match j {
                    0 => {
                        if !arr[xdix][ydix].eq(&'X') {
                            sat = false;
                        }
                    }
                    1 => {
                        if !arr[xdix][ydix].eq(&'M') {
                            sat = false;
                        }
                    }
                    2 => {
                        if !arr[xdix][ydix].eq(&'A') {
                            sat = false;
                        }
                    }
                    3 => {
                        if !arr[xdix][ydix].eq(&'S') {
                            sat = false;
                        }
                    }
                    _ => {}
                }
            } else {
                sat = false;
            }
            if !sat {
                break;
            }
        }
        if sat {
            //println!("{} {} {}", x, y, i);
            ans += 1;
        }
    }
    ans
}

fn check_mas_in_x_shape(arr: &Vec<Vec<char>>, x: i32, y: i32, x_len: usize, y_len: usize) -> bool {
    let xd = [-1, -1, 1, 1];
    let yd = [-1, 1, -1, 1];
    let m = &'M';
    let s = &'S';
    for i in 0..4 {
        let x_ = x + xd[i];
        let y_ = y + yd[i];
        if x_ < 0 || x_ >= x_len.try_into().unwrap() || y_ < 0 || y_ >= y_len.try_into().unwrap() {
            return false;
        }
    }
    let xidx = [
        (x + xd[0]) as usize,
        (x + xd[1]) as usize,
        (x + xd[2]) as usize,
        (x + xd[3]) as usize,
    ];
    let yidx = [
        (y + yd[0]) as usize,
        (y + yd[1]) as usize,
        (y + yd[2]) as usize,
        (y + yd[3]) as usize,
    ];
    if (arr[xidx[0]][yidx[0]].eq(m)
        && arr[xidx[1]][yidx[1]].eq(m)
        && arr[xidx[2]][yidx[2]].eq(s)
        && arr[xidx[3]][yidx[3]].eq(s))
        || (arr[xidx[0]][yidx[0]].eq(s)
            && arr[xidx[1]][yidx[1]].eq(s)
            && arr[xidx[2]][yidx[2]].eq(m)
            && arr[xidx[3]][yidx[3]].eq(m))
        || (arr[xidx[0]][yidx[0]].eq(m)
            && arr[xidx[1]][yidx[1]].eq(s)
            && arr[xidx[2]][yidx[2]].eq(m)
            && arr[xidx[3]][yidx[3]].eq(s))
        || (arr[xidx[0]][yidx[0]].eq(s)
            && arr[xidx[1]][yidx[1]].eq(m)
            && arr[xidx[2]][yidx[2]].eq(s)
            && arr[xidx[3]][yidx[3]].eq(m))
    {
        return true;
    }
    return false;
}

pub fn day4() {
    println!("Solving day 4 problems");
    let mut arr = vec![];
    for line in read_file("day4.txt") {
        let mut l = vec![];
        for c in line.chars() {
            l.push(c);
        }
        arr.push(l);
    }
    let n = arr.len().try_into().unwrap();
    let m = arr[0].len().try_into().unwrap();
    let (mut ans1, mut ans2) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if arr[i][j].eq(&'X') {
                ans1 += check_xmas(&arr, i.try_into().unwrap(), j.try_into().unwrap(), n, m);
            } else if arr[i][j].eq(&'A')
                && check_mas_in_x_shape(&arr, i.try_into().unwrap(), j.try_into().unwrap(), n, m)
            {
                ans2 += 1;
            }
        }
    }
    println!("Part 1 - {}", ans1);
    println!("Part 2 - {}", ans2);
}

pub fn day5() {
    println!("Solving day 5 problems");
    let mut adj_list: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut queries: Vec<Vec<i32>> = vec![];
    for line in read_file("day5.txt") {
        if line.contains("|") {
            let v: Vec<&str> = line.split("|").collect();
            let x = v[0].parse::<i32>().unwrap();
            let y = v[1].parse::<i32>().unwrap();
            match adj_list.get_mut(&x) {
                Some(set) => {
                    set.insert(y);
                }
                None => {
                    adj_list.insert(x, HashSet::from([y]));
                }
            }
        } else if line.contains(",") {
            let v: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            queries.push(v);
        }
    }
    let mut ans_safe = 0;
    let mut ans_reordered_safe = 0;
    let mut safe;
    for query in &queries {
        //println!("{:?}", query);
        let l = query.len();
        safe = true;
        for i in 0..l {
            for j in i + 1..l {
                if adj_list.get(&query[j]).is_some()
                    && adj_list.get(&query[j]).unwrap().contains(&query[i])
                {
                    safe = false;
                }
                if !safe {
                    break;
                }
            }
            if !safe {
                break;
            }
        }
        if safe {
            let middle = query[l / 2];
            ans_safe = ans_safe + middle;
            //println!("{:?}", middle);
        } else {
            let mut adj_list_q: HashMap<&i32, &HashSet<i32>> = HashMap::new();
            for v in query {
                if adj_list.get(v).is_some() {
                    adj_list_q.insert(v, adj_list.get(v).unwrap());
                }
            }
            let ordered_q = topological_sort(&adj_list_q, &query);
            //println!("{:?}", ordered_q);
            ans_reordered_safe = ans_reordered_safe + ordered_q[l / 2];
        }
    }
    //println!("{} {}", queries.len(), safe_queries);
    println!("Part 1 - {}", ans_safe);
    println!("Part 2 - {}", ans_reordered_safe);
}

fn topological_sort(adj_ist: &HashMap<&i32, &HashSet<i32>>, nodes: &Vec<i32>) -> Vec<i32> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut order = vec![];
    let node_set = HashSet::from_iter(nodes);
    for v in nodes {
        if visited.get(v).is_none() {
            dfs(adj_ist, v, &mut order, &mut visited, &node_set);
        }
    }
    order.reverse();
    order
}

fn dfs(
    adj_list: &HashMap<&i32, &HashSet<i32>>,
    v: &i32,
    order: &mut Vec<i32>,
    visited: &mut HashSet<i32>,
    node_set: &HashSet<&i32>,
) {
    visited.insert(*v);
    for u in adj_list.get(v).unwrap().iter() {
        if node_set.contains(u) && !visited.contains(u) {
            dfs(adj_list, u, order, visited, node_set);
        }
    }
    order.push(*v);
}
