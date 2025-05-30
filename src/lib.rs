use regex::Regex;
use std::cmp::{max, min};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::str::FromStr;

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

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn get_next_step(cur_dir: &Direction, x: &i32, y: &i32) -> (i32, i32) {
    let (mut x_, mut y_) = (*x, *y);
    match cur_dir {
        Direction::UP => {
            x_ = x - 1;
        }
        Direction::DOWN => {
            x_ = x + 1;
        }
        Direction::LEFT => {
            y_ = y - 1;
        }
        Direction::RIGHT => {
            y_ = y + 1;
        }
    }
    (x_, y_)
}

fn turn_direction(cur_dir: &Direction) -> Direction {
    match cur_dir {
        Direction::UP => {
            return Direction::RIGHT;
        }
        Direction::DOWN => {
            return Direction::LEFT;
        }
        Direction::LEFT => {
            return Direction::UP;
        }
        Direction::RIGHT => {
            return Direction::DOWN;
        }
    }
}

fn count_distance_to_exit(
    arr: &Vec<Vec<char>>,
    sx: &i32,
    sy: &i32,
    obst: &Option<(usize, usize)>,
) -> Option<usize> {
    let n: i32 = arr.len().try_into().unwrap();
    let m: i32 = arr[0].len().try_into().unwrap();
    let (mut x, mut y) = (*sx, *sy);
    let mut cur_dir = Direction::UP;
    let mut unique_dir: HashSet<(i32, i32, Direction)> = HashSet::from([(x, y, cur_dir)]);
    loop {
        //println!("{} {} {:?}", x, y, cur_dir);
        let (x_, y_) = get_next_step(&cur_dir, &x, &y);
        let (xs, ys) = (x_ as usize, y_ as usize);
        if x_ < 0 || x_ >= n || y_ < 0 || y_ >= m {
            break;
        }
        if obst.is_some() && obst.unwrap().eq(&(xs, ys)) || arr[xs][ys].eq(&'#') {
            cur_dir = turn_direction(&cur_dir);
        } else {
            (x, y) = (x_, y_);
            if obst.is_some() {
                if unique_dir.contains(&(x, y, cur_dir)) {
                    return None;
                }
                unique_dir.insert((x, y, cur_dir));
            } else {
                unique_dir.insert((x, y, Direction::UP));
            }
        }
    }
    Some(unique_dir.len())
}
pub fn day6() {
    println!("Solving day 6 problems");
    let mut arr = vec![];
    let (mut sx, mut sy) = (0, 0);
    let (mut i, mut j) = (0, 0);
    for line in read_file("day6.txt") {
        let mut l = vec![];
        for c in line.chars() {
            l.push(c);
            if c.eq(&'^') {
                (sx, sy) = (i, j);
            }
            j = j + 1;
        }
        i = i + 1;
        j = 0;
        arr.push(l);
    }
    println!(
        "Part 1 - {}",
        count_distance_to_exit(&arr, &sx, &sy, &None).unwrap()
    );

    //Optimise complexity, runs for almost 2 minutes so commented out
    let n = arr.len();
    let m = arr[0].len();
    let mut obstruction = 0;
    for i in 0..n {
        for j in 0..m {
            if !arr[i][j].eq(&'#') && !arr[i][j].eq(&'^') {
                if count_distance_to_exit(&arr, &sx, &sy, &Some((i, j))).is_none() {
                    //println!("{} {}", i, j);
                    obstruction = obstruction + 1;
                }
            }
        }
    }
    println!("Part 2 - {}", obstruction);
}

enum Operator {
    SUM,
    MULTIPLY,
    CONCATENATION,
}
fn check_possible_combination(
    result: &u64,
    values: &Vec<u64>,
    cur_val: u64,
    cur_idx: usize,
    operator: Operator,
    use_concatenation: bool,
) -> bool {
    if cur_idx >= values.len() {
        return cur_val == *result;
    }
    let next_value;
    match operator {
        Operator::SUM => {
            next_value = cur_val + values[cur_idx];
        }
        Operator::MULTIPLY => {
            next_value = cur_val * values[cur_idx];
        }
        Operator::CONCATENATION => {
            if use_concatenation {
                next_value =
                    u64::from_str(&*(cur_val.to_string() + &values[cur_idx].to_string())).unwrap();
            } else {
                next_value = 0;
            }
        }
    }
    let check_operators: Vec<Operator>;
    if use_concatenation {
        check_operators = Vec::from([Operator::SUM, Operator::MULTIPLY, Operator::CONCATENATION]);
    } else {
        check_operators = Vec::from([Operator::SUM, Operator::MULTIPLY]);
    }
    for op in check_operators {
        if check_possible_combination(
            result,
            values,
            next_value.clone(),
            cur_idx + 1,
            op,
            use_concatenation,
        ) {
            return true;
        }
    }
    false
}

pub fn day7() {
    println!("Solving day 7 problems");
    let mut part1_ans: u64 = 0;
    let mut part2_ans: u64 = 0;
    for line in read_file("day7.txt") {
        let v: Vec<&str> = line.split_whitespace().collect();
        let result = u64::from_str(v[0].split(":").take(1).next().unwrap()).unwrap();
        let values: Vec<u64> = v[1..].iter().map(|x| u64::from_str(x).unwrap()).collect();
        //println!("{} {:?}", sum, values);
        if check_possible_combination(&result, &values, 0, 0, Operator::SUM, false) {
            part1_ans = part1_ans + result;
        }
        if check_possible_combination(&result, &values, 0, 0, Operator::SUM, true) {
            part2_ans = part2_ans + result;
        }
    }
    println!("Part 1 - {}", part1_ans);
    println!("Part 2 - {}", part2_ans);
}

pub fn day8() {
    println!("Solving day 8 problems");
    let mut m = 0;
    let mut n = 0;
    let mut antenna_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for line in read_file("day8.txt") {
        m = 0;
        for c in line.chars() {
            if !c.eq(&'.') {
                match antenna_map.get_mut(&c) {
                    Some(set) => {
                        set.push((n, m));
                    }
                    None => {
                        antenna_map.insert(c, vec![(n, m)]);
                    }
                }
            }
            m = m + 1;
        }
        n = n + 1;
    }
    //println!("{} {}", m, n);
    let mut uniq_antinodes_part1: HashSet<(i32, i32)> = HashSet::new();
    let mut uniq_antinodes_part2: HashSet<(i32, i32)> = HashSet::new();
    for c in antenna_map.keys() {
        let arr = antenna_map.get(c).unwrap();
        //println!("{} - {:?}", c, arr);
        let l = arr.len();
        for i in 0..l {
            for j in i + 1..l {
                let (d1, d2) = (arr[i].0 - arr[j].0, arr[i].1 - arr[j].1);
                let (mut x1, mut y1) = (arr[i].0 + d1, arr[i].1 + d2);
                let (mut x2, mut y2) = (arr[j].0 - d1, arr[j].1 - d2);
                uniq_antinodes_part2.insert((arr[i].0, arr[i].1));
                uniq_antinodes_part2.insert((arr[j].0, arr[j].1));
                if x1 >= 0 && x1 < n && y1 >= 0 && y1 < m {
                    uniq_antinodes_part1.insert((x1, y1));
                }
                if x2 >= 0 && x2 < n && y2 >= 0 && y2 < m {
                    uniq_antinodes_part1.insert((x2, y2));
                }
                while x1 >= 0 && x1 < n && y1 >= 0 && y1 < m {
                    uniq_antinodes_part2.insert((x1, y1));
                    (x1, y1) = (x1 + d1, y1 + d2);
                }
                while x2 >= 0 && x2 < n && y2 >= 0 && y2 < m {
                    uniq_antinodes_part2.insert((x2, y2));
                    (x2, y2) = (x2 - d1, y2 - d2);
                }
            }
        }
    }
    println!("Part 1 - {}", uniq_antinodes_part1.len());
    println!("Part 2 - {}", uniq_antinodes_part2.len());
}

fn move_file_one_block(
    mut occupied_block: Vec<(u32, u32, u32)>,
    mut free_block: Vec<(u32, u32)>,
) -> u64 {
    let mut free_idx: usize = 0;
    let mut reversed_occ_idx: i32 = (occupied_block.len() - 1) as i32;
    let mut checksum: u64 = 0;
    let mut moved_space = 0;
    loop {
        while reversed_occ_idx >= 0 && occupied_block[reversed_occ_idx as usize].2 == 0 {
            reversed_occ_idx = reversed_occ_idx - 1;
        }
        if reversed_occ_idx < 0 {
            break;
        }
        let (occ_id, occ_offset, occ_len) = occupied_block[reversed_occ_idx as usize];
        while free_idx < free_block.len() && free_block[free_idx].1 == 0 {
            free_idx = free_idx + 1;
        }
        if free_idx >= free_block.len() {
            break;
        }
        let (free_offset, free_len) = free_block[free_idx];
        //Free block is after the last file block
        if occ_offset < free_offset {
            break;
        }
        let deduction = min(free_len, occ_len);
        moved_space = moved_space + deduction;
        free_block[free_idx].0 = free_offset + deduction;
        free_block[free_idx].1 = free_len - deduction;
        let adder = (occ_id * (((2 * free_offset + deduction - 1) * deduction) / 2)) as u64;
        //println!("adder - {}", adder);
        checksum = checksum + adder;
        //println!("Current checksum - {}", checksum);
        occupied_block[reversed_occ_idx as usize].2 = occ_len - deduction;
        /*
        println!(
            "occupied block - {:?}",
            occupied_block[reversed_occ_idx as usize]
        );
         */
        //println!("free block - {:?}", free_block[free_idx]);
    }
    //println!("{} {}", reversed_occ_idx, free_idx);
    //println!("moved space {}", moved_space);
    for idx in 0..reversed_occ_idx + 1 {
        let (occ_id, occ_offset, occ_len) = occupied_block[idx as usize];
        //println!("occupied block - {:?}", occupied_block[idx as usize]);
        if occ_len > 0 {
            let adder = (occ_id * (((2 * occ_offset + occ_len - 1) * occ_len) / 2)) as u64;
            //println!("adder - {}", adder);
            checksum = checksum + adder;
        }
        //println!("Current checksum - {}", checksum);
    }
    checksum
}

fn move_file_total(
    mut occupied_block: Vec<(u32, u32, u32)>,
    mut free_block: Vec<(u32, u32)>,
) -> u64 {
    let mut free_idx: usize;
    let mut reversed_occ_idx: i32;
    let mut checksum: u64 = 0;
    let mut moved_space = 0;
    reversed_occ_idx = (occupied_block.len() - 1) as i32;
    loop {
        //println!("occupied block {:?}", occupied_block);
        //println!("free block {:?}", free_block);
        let (free_offset, free_len);
        free_idx = 0;
        while reversed_occ_idx >= 0 && occupied_block[reversed_occ_idx as usize].2 == 0 {
            reversed_occ_idx = reversed_occ_idx - 1;
        }
        if reversed_occ_idx < 0 {
            break;
        }
        while free_idx < free_block.len()
            && (free_block[free_idx].1 == 0
                || free_block[free_idx].1 < occupied_block[reversed_occ_idx as usize].2)
        {
            free_idx = free_idx + 1;
        }
        if reversed_occ_idx < 0 {
            break;
        }
        if free_idx >= free_block.len() {
            reversed_occ_idx = reversed_occ_idx - 1;
            continue;
        }
        (free_offset, free_len) = free_block[free_idx];
        let (occ_id, occ_offset, occ_len) = occupied_block[reversed_occ_idx as usize];
        if occ_offset <= free_offset {
            reversed_occ_idx = reversed_occ_idx - 1;
            continue;
        }
        let deduction = occ_len;
        moved_space = moved_space + deduction;
        free_block[free_idx].0 = free_offset + deduction;
        free_block[free_idx].1 = free_len - deduction;
        if free_block[free_idx].1 == 0 {
            free_block.remove(free_idx);
        }
        let adder = (occ_id * (((2 * free_offset + deduction - 1) * deduction) / 2)) as u64;
        //println!("adder - {}", adder);
        checksum = checksum + adder;
        //println!("Current checksum - {}", checksum);
        occupied_block[reversed_occ_idx as usize].2 = 0;
        let mut free_insert_idx = 0;
        while free_insert_idx < free_block.len() && occ_offset >= free_block[free_insert_idx].0 {
            free_insert_idx = free_insert_idx + 1;
        }
        let (mut new_offset, mut new_len) = (occ_offset, occ_len);
        //check if previous free block can be combined
        if free_insert_idx > 0
            && free_block[free_insert_idx - 1].0 + free_block[free_insert_idx - 1].1 == occ_offset
        {
            let pre_idx = free_insert_idx - 1;
            new_offset = free_block[pre_idx].0;
            new_len = occ_len + free_block[pre_idx].1;
            free_insert_idx = free_insert_idx - 1;
            free_block.remove(pre_idx);
        }
        //check if next free block can be combined
        if free_insert_idx < free_block.len()
            && free_block[free_insert_idx].0 == new_offset + new_len
        {
            free_block[free_insert_idx] = (new_offset, free_block[free_insert_idx].1 + new_len);
        } else {
            free_block.insert(free_insert_idx, (occ_offset, occ_len));
        }
    }
    //println!("occupied block {:?}", occupied_block);
    //println!("free block {:?}", free_block);
    //println!("{} {}", reversed_occ_idx, free_idx);
    //println!("moved space {}", moved_space);
    for idx in 0..occupied_block.len() {
        let (occ_id, occ_offset, occ_len) = occupied_block[idx as usize];
        //println!("occupied block - {:?}", occupied_block[idx as usize]);
        if occ_len > 0 {
            let adder = (occ_id * (((2 * occ_offset + occ_len - 1) * occ_len) / 2)) as u64;
            //println!("adder - {}", adder);
            checksum = checksum + adder;
        }
        //println!("Current checksum - {}", checksum);
    }
    checksum
}
pub fn day9() {
    //(identifier, offset, len)
    let mut occupied_block: Vec<(u32, u32, u32)> = vec![];
    //(offset, len)
    let mut free_block: Vec<(u32, u32)> = vec![];
    let mut offset = 0;
    let mut identifier: u32 = 0;
    let representation: Vec<u32> = read_file("day9.txt")
        .get(0)
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    for d in (0..representation.len()).step_by(2) {
        occupied_block.push((identifier, offset, representation[d]));
        offset = offset + representation[d];
        if d + 1 < representation.len() {
            free_block.push((offset, representation[d + 1]));
            offset = offset + representation[d + 1];
        }
        identifier = identifier + 1;
    }
    println!(
        "Part 1 - {}",
        move_file_one_block(occupied_block.clone(), free_block.clone())
    );
    println!("Part 2 - {}", move_file_total(occupied_block, free_block));
}

pub fn day10() {
    let mut arr: Vec<Vec<u32>> = vec![];
    for line in read_file("day10.txt") {
        let v: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        arr.push(v);
    }
    let (n, m) = (arr.len() as i32, arr[0].len() as i32);
    let xd = [-1, 1, 0, 0];
    let yd = [0, 0, -1, 1];
    let mut part1_total_trailheads: u32 = 0;
    let mut part2_total_unique_trails = 0;
    for i in 0..n {
        for j in 0..m {
            if arr[i as usize][j as usize] == 0 {
                let mut v: VecDeque<(i32, i32, u32)> = VecDeque::new();
                v.push_back((i, j, 0));
                let mut unique_trails: HashSet<(i32, i32)> = HashSet::new();
                while !v.is_empty() {
                    let (x, y, c) = v.pop_front().unwrap();
                    if c == 9 {
                        unique_trails.insert((x, y));
                        part2_total_unique_trails = part2_total_unique_trails + 1;
                    }
                    for k in 0..4 {
                        let (x_, y_) = (x + xd[k], y + yd[k]);
                        if x_ >= 0
                            && x_ < n
                            && y_ >= 0
                            && y_ < m
                            && arr[x_ as usize][y_ as usize] == c + 1
                        {
                            v.push_back((x_, y_, c + 1));
                        }
                    }
                }
                part1_total_trailheads = part1_total_trailheads + (unique_trails.len() as u32);
            }
        }
    }
    println!("Part 1 - {}", part1_total_trailheads);
    println!("Part 2 - {}", part2_total_unique_trails);
}

fn find_next_stones(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next_stones: HashMap<u64, u64> = HashMap::new();

    for kv in stones {
        let new_values: Vec<u64>;
        let s = kv.0;
        if s.eq(&0) {
            new_values = vec![1];
        } else {
            let s_str = s.to_string();
            if s_str.len() % 2 != 0 {
                new_values = vec![s * 2024]
            } else {
                let half = s_str.len() / 2;
                new_values = vec![
                    s_str.get(0..half).unwrap().parse().unwrap(),
                    s_str.get(half..).unwrap().parse().unwrap(),
                ];
            };
        }
        for v in new_values {
            match next_stones.get(&v) {
                Some(pre) => {
                    next_stones.insert(v, pre + kv.1);
                }
                None => {
                    next_stones.insert(v, kv.1);
                }
            }
        }
    }
    //println!("{:?}", next_stones);
    next_stones
}

pub fn day11() {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for v in read_file("day11.txt")[0]
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
    {
        match stones.get(&v) {
            Some(pre) => {
                stones.insert(v, pre + 1);
            }
            None => {
                stones.insert(v, 1);
            }
        }
    }
    let check_at_iteration = [24, 74];
    let max_iteration = *check_at_iteration.iter().max().unwrap() + 1;

    for i in 0..max_iteration {
        //println!("{:?}", next_stones);
        stones = find_next_stones(stones);
        if check_at_iteration.contains(&i) {
            println!(
                "Count after iteration {} - {}",
                i,
                stones.values().sum::<u64>()
            );
        }
    }
}

pub fn bfs(visited: &mut Vec<Vec<bool>>, matrix: &Vec<Vec<char>>, x: i32, y: i32) -> usize {
    let (mut area, mut parameter) = (0, 0);
    visited[x as usize][y as usize] = true;
    let check_char = matrix[x as usize][y as usize];
    let mut q = VecDeque::new();
    q.push_back((x, y));
    let xd = [-1, 1, 0, 0];
    let yd = [0, 0, -1, 1];
    let (n, m) = (matrix.len() as i32, matrix[0].len() as i32);
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        area = area + 1;
        for i in 0..4 {
            let (x_, y_) = (x + xd[i], y + yd[i]);
            if x_ >= 0 && x_ < n && y_ >= 0 && y_ < m {
                let (xu, yu) = (x_ as usize, y_ as usize);
                if matrix[xu][yu].eq(&check_char) && !visited[xu][yu] {
                    q.push_back((x_, y_));
                    visited[xu][yu] = true;
                } else if !matrix[xu][yu].eq(&check_char) {
                    parameter = parameter + 1;
                }
            } else {
                parameter = parameter + 1;
            }
        }
    }
    area * parameter
}

pub fn day12() {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in read_file("day12.txt") {
        let v: Vec<char> = line.chars().collect();
        matrix.push(v);
    }
    let (n, m) = (matrix.len(), matrix[0].len());
    let mut visited = vec![vec![false; m]; n];
    let mut sum = 0;
    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] {
                sum = sum + bfs(&mut visited, &matrix, i as i32, j as i32);
                //println!("{} {} {}", i, j, sum);
            }
        }
    }
    println!("Part 1 - {}", sum);
}

fn capture_and_get_coordinates(needle: &Regex, haystack: &String) -> (i64, i64) {
    let c = needle.captures(haystack).unwrap();
    let (xa, ya) = (
        i64::from_str(c.get(1).unwrap().as_str()).unwrap(),
        i64::from_str(c.get(2).unwrap().as_str()).unwrap(),
    );
    (xa, ya)
}

/**
Brute force to solve system of linear equation with 2 variables
**/
#[allow(dead_code)]
#[warn(deprecated)]
fn solve_brute_method(
    button_a: &(i64, i64),
    button_b: &(i64, i64),
    prize: &(i64, i64),
) -> Option<i64> {
    let (cost_a, cost_b) = (3, 1);
    let mut min_cost: Option<i64> = None;
    let (x_bound, y_bound) = (
        max(prize.0 / button_a.0, prize.0 / button_b.0) + 1,
        max(prize.1 / button_a.1, prize.1 / button_b.1) + 1,
    );
    for i in 1..x_bound {
        for j in 1..y_bound {
            if i * button_a.0 + j * button_b.0 == prize.0
                && i * button_a.1 + j * button_b.1 == prize.1
            {
                let current = i * cost_a + j * cost_b;
                match min_cost {
                    Some(previous) => {
                        min_cost = Some(min(current, previous));
                    }
                    None => {
                        min_cost = Some(current);
                    }
                }
            }
        }
    }
    min_cost
}

/**
Cramer's rule for solving system of linear equation with 2 variables
**/
fn solve_matrix_method(
    button_a: &(i64, i64),
    button_b: &(i64, i64),
    prize: &(i64, i64),
) -> Option<i64> {
    let (cost_x, cost_y) = (3, 1);
    let mut min_cost: Option<i64> = None;
    let (xa, xb) = (button_a.0, button_b.0);
    let (ya, yb) = (button_a.1, button_b.1);
    let (px, py) = prize;
    let determinant = xa * yb - xb * ya;
    let x = yb * px - xb * py;
    let y = xa * py - ya * px;
    if x % determinant == 0 && y % determinant == 0 {
        let (sol_x, sol_y) = (x / determinant, y / determinant);
        //println!("{} {}", sol_x, sol_y);
        min_cost = Some(sol_x * cost_x + sol_y * cost_y);
    }
    min_cost
}
pub fn day13() {
    let button_pattern = Regex::new("Button [A-B]: X\\+([0-9]+), Y\\+([0-9]+)").unwrap();
    let prize_pattern = Regex::new("Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let lines = read_file("day13.txt");
    let mut file_itr = lines.iter();
    let mut part1_ans: i64 = 0;
    let mut part2_ans: i64 = 0;
    let mut case_no = 0;
    let part_2_prize_correction: i64 = 10000000000000;
    while let Some(button_a) = file_itr.next() {
        case_no = case_no + 1;
        //println!("On case no - {}", case_no);
        let Some(button_b) = file_itr.next() else {
            break;
        };
        let Some(prize) = file_itr.next() else { break };
        let (xa, ya) = capture_and_get_coordinates(&button_pattern, button_a);
        //println!("{} {}", xa, ya);
        let (xb, yb) = capture_and_get_coordinates(&button_pattern, button_b);
        //println!("{} {}", xb, yb);
        let (xp, yp) = capture_and_get_coordinates(&prize_pattern, prize);
        //println!("{} {}", xp, yp);
        match solve_matrix_method(&(xa, ya), &(xb, yb), &(xp, yp)) {
            Some(v) => part1_ans = part1_ans + v,
            _ => {}
        }

        match solve_matrix_method(
            &(xa, ya),
            &(xb, yb),
            &(xp + part_2_prize_correction, yp + part_2_prize_correction),
        ) {
            Some(v) => part2_ans = part2_ans + v,
            _ => {}
        }

        if file_itr.next() == None {
            break;
        }
    }
    println!("Part 1 - {}", part1_ans);
    println!("Part 2 - {}", part2_ans);
}

#[derive(Debug)]
struct Robot {
    init_pos: (i64, i64),
    velocity: (i64, i64),
}

fn display(matrix: &Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{}", matrix[i][j]);
        }
        println!();
    }
    println!("\n\n");
}

fn total_connected_component(matrix: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let (len_x, len_y) = (matrix.len() as i32, matrix[0].len() as i32);
    let mut visited = vec![vec![false; len_y as usize]; len_x as usize];
    let xd = [-1, 1, 0, 0];
    let yd = [0, 0, -1, 1];
    for i in 0..len_x {
        for j in 0..len_y {
            if matrix[i as usize][j as usize].eq(&'1') && !visited[i as usize][j as usize] {
                ans = ans + 1;
                let mut q = VecDeque::new();
                q.push_back((i, j));
                visited[i as usize][j as usize] = true;
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for k in 0..4 {
                        let (x_, y_) = ((x) + xd[k], (y) + yd[k]);
                        if x_ >= 0
                            && x_ < len_x
                            && y_ >= 0
                            && y_ < len_y
                            && matrix[x_ as usize][y_ as usize].eq(&'1')
                            && !visited[x_ as usize][y_ as usize]
                        {
                            q.push_back((x_, y_));
                            visited[x_ as usize][y_ as usize] = true;
                        }
                    }
                }
            }
        }
    }
    //println!("Total components - {}", ans);
    ans
}

fn simulate(robots: &Vec<Robot>, len_x: &i64, len_y: &i64) {
    let mut second = 0;
    //When connected components decrease by 50%, then we have more likelihood of seeing the tree created by close robots
    let cluster_percentage = 0.5;
    let uniq_component_threshold = f64::floor(robots.len() as f64 * cluster_percentage) as usize;
    loop {
        let mut graphical_rep = vec![vec!['.'; *len_y as usize]; *len_x as usize];
        second = second + 1;
        for robot in robots {
            let (init, vel) = (robot.init_pos, robot.velocity);
            //println!("{:?}", robot);
            let (x, y) = (
                (init.0 + second * vel.0).rem_euclid(*len_x),
                (init.1 + second * vel.1).rem_euclid(*len_y),
            );
            //println!("{} {}", x, y);
            graphical_rep[x as usize][y as usize] = '1';
        }
        if total_connected_component(&graphical_rep) <= uniq_component_threshold {
            println!("Current iteration - {}", second);
            display(&graphical_rep);
            println!("\n\n\n\n\n");
            //Saw the tree in first attempt :), more loops may be needed for more intelligent test case
            break;
        }
    }
}

pub fn day14() {
    let robot_pattern = Regex::new("p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let (len_x, len_y) = (101, 103);
    let time = 100;
    let mut quadrant = [0; 4];
    let mut robots: Vec<Robot> = vec![];

    for line in read_file("day14.txt") {
        let captures = robot_pattern.captures(&line).unwrap();
        let (cur_x, cur_y) = (
            i64::from_str(captures.get(1).unwrap().as_str()).unwrap(),
            i64::from_str(captures.get(2).unwrap().as_str()).unwrap(),
        );
        let (vel_x, vel_y) = (
            i64::from_str(captures.get(3).unwrap().as_str()).unwrap(),
            i64::from_str(captures.get(4).unwrap().as_str()).unwrap(),
        );
        let robot = Robot {
            init_pos: (cur_x, cur_y),
            velocity: (vel_x, vel_y),
        };
        robots.push(robot);
        //println!("{} {} {} {}", cur_x, cur_y, vel_x, vel_y);
        let (new_x, new_y) = (
            //Rust doesn't have modulo, so use rem_euclid instead, ref https://internals.rust-lang.org/t/mathematical-modulo-operator/5952
            (cur_x + time * vel_x).rem_euclid(len_x),
            (cur_y + time * vel_y).rem_euclid(len_y),
        );
        if new_x == len_x / 2 || new_y == len_y / 2 {
            continue;
        }
        //println!("{} {}", new_x, new_y);
        if new_x < len_x / 2 {
            if new_y < len_y / 2 {
                quadrant[0] = quadrant[0] + 1;
            } else {
                quadrant[1] = quadrant[1] + 1;
            }
        } else {
            if new_y < len_y / 2 {
                quadrant[2] = quadrant[2] + 1;
            } else {
                quadrant[3] = quadrant[3] + 1;
            }
        }
    }
    println!("Part 1 - {}", quadrant.iter().fold(1, |acc, v| acc * v));
    simulate(&robots, &len_x, &len_y);
}

fn move_robot_part_1_day_15(
    dir: &char,
    x: usize,
    y: usize,
    positions: &mut Vec<Vec<char>>,
) -> (usize, usize) {
    let moves = HashMap::from([('>', (0, 1)), ('<', (0, -1)), ('v', (1, 0)), ('^', (-1, 0))]);
    let (xd, yd) = moves.get(dir).unwrap();
    let (mut x_, mut y_);
    (x_, y_) = ((x as i32 + xd) as usize, (y as i32 + yd) as usize);
    let (mut new_x, mut new_y) = (x, y);
    if positions[x_][y_].eq(&'.') {
        (new_x, new_y) = (x_, y_)
    } else if positions[x_][y_].eq(&'O') {
        (new_x, new_y) = (x_, y_);
        while positions[x_][y_].eq(&'O') {
            (x_, y_) = ((x_ as i32 + xd) as usize, (y_ as i32 + yd) as usize);
        }
        if !positions[x_][y_].eq(&'#') {
            positions[x_][y_] = 'O';
        } else {
            //Reset position
            (new_x, new_y) = (x, y);
        }
    }

    if new_x != x || new_y != y {
        positions[x][y] = '.';
        positions[new_x][new_y] = '@';
    }
    (new_x, new_y)
}

fn step_forward_robot_part_2_day_15(
    dir: &(i32, i32),
    positions: &mut Vec<Vec<char>>,
    cur_position: Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    if cur_position.iter().any(|x| positions[x.0][x.1].eq(&'#')) {
        None
    } else if cur_position.iter().all(|x| positions[x.0][x.1].eq(&'.')) {
        Some(cur_position)
    } else {
        let next_position: Vec<(usize, usize)> = cur_position
            .iter()
            .map(|p| (p.0 as i32 + dir.0, p.1 as i32 + dir.1))
            .map(|x| (x.0 as usize, x.1 as usize))
            .collect();
        let mut next_position_copy = BTreeSet::new();

        //vertical movement
        for i in 0..next_position.len() {
            let p = next_position[i];
            match positions[p.0][p.1] {
                '[' => {
                    next_position_copy.insert(p);
                    if dir.1 == 0 {
                        next_position_copy.insert((p.0, p.1 + 1));
                    }
                }
                ']' => {
                    next_position_copy.insert(p);
                    if dir.1 == 0 {
                        next_position_copy.insert((p.0, p.1 - 1));
                    }
                }
                '#' => {
                    next_position_copy.insert(p);
                }
                _ => {}
            }
        }

        //Remove redundant empty spaces
        next_position_copy.retain(|x| !positions[x.0][x.1].eq(&'.'));

        match step_forward_robot_part_2_day_15(
            dir,
            positions,
            next_position_copy.into_iter().collect(),
        ) {
            Some(_) => {
                for i in 0..next_position.len() {
                    let n = next_position[i];
                    let c = cur_position[i];
                    positions[n.0][n.1] = positions[c.0][c.1];
                    positions[c.0][c.1] = '.';
                }
                Some(next_position)
            }
            None => None,
        }
    }
}

fn move_robot_part_2_day_15(
    dir: &char,
    x: usize,
    y: usize,
    positions: &mut Vec<Vec<char>>,
) -> (usize, usize) {
    let moves = HashMap::from([('>', (0, 1)), ('<', (0, -1)), ('v', (1, 0)), ('^', (-1, 0))]);
    let delta = moves.get(dir).unwrap();
    match step_forward_robot_part_2_day_15(delta, positions, vec![(x, y)]) {
        Some(v) => v[0],
        None => (x, y),
    }
}

fn calculate_gps_score(positions: &Vec<Vec<char>>, check_char: char) -> usize {
    let (n, m) = (positions.len(), positions[0].len());
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if positions[i][j].eq(&check_char) {
                ans = ans + i * 100 + j;
            }
        }
    }
    ans
}

fn perform_moves_robot(
    positions: &mut Vec<Vec<char>>,
    mut x: usize,
    mut y: usize,
    moves: &Vec<Vec<char>>,
    robot_function: fn(&char, usize, usize, &mut Vec<Vec<char>>) -> (usize, usize),
) {
    //display(positions);
    for l in moves {
        for m in l {
            (x, y) = robot_function(&m, x, y, positions);
            //println!("Move : {}", m);
            //display(positions);
        }
    }
}

pub fn day15() {
    let mut positions: Vec<Vec<char>> = vec![];
    let mut positions2: Vec<Vec<char>> = vec![];
    let mut moves: Vec<Vec<char>> = vec![];
    let (mut x, mut y) = (0, 0);
    for line in read_file("day15.txt") {
        if line.starts_with("#") {
            positions.push(line.chars().collect());
            match positions[positions.len() - 1]
                .iter()
                .position(|x| x.eq(&'@'))
            {
                Some(v) => {
                    x = positions.len() - 1;
                    y = v;
                }
                _ => {}
            }

            let mut vec2 = vec![];
            for c in line.chars() {
                if c.eq(&'@') {
                    vec2.push(c);
                    vec2.push('.');
                } else if c.eq(&'O') {
                    vec2.push('[');
                    vec2.push(']');
                } else {
                    vec2.push(c);
                    vec2.push(c);
                }
            }
            positions2.push(vec2);
        } else if line.starts_with(|x: char| !x.is_whitespace()) {
            moves.push(line.chars().collect());
        }
    }

    perform_moves_robot(&mut positions, x, y, &moves, move_robot_part_1_day_15);
    perform_moves_robot(&mut positions2, x, 2 * y, &moves, move_robot_part_2_day_15);
    println!("Part 1 - {}", calculate_gps_score(&positions, 'O'));
    println!("Part 2 - {}", calculate_gps_score(&positions2, '['));
}

fn calculate_direction_cost(cur_dir: &char, new_dir: &char) -> i32 {
    if cur_dir.eq(new_dir) {
        0
    } else if (cur_dir.eq(&'s') && new_dir.eq(&'n'))
        || (cur_dir.eq(&'n') && new_dir.eq(&'s'))
        || (cur_dir.eq(&'e') && new_dir.eq(&'w'))
        || (cur_dir.eq(&'w') && new_dir.eq(&'e'))
    {
        2000
    } else {
        1000
    }
}
fn solve_day_16(positions: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) {
    let mut pq = BinaryHeap::new();
    const INFINITY: i32 = i32::MAX;
    pq.push((0, start, 'e', start, 'e'));
    let (n, m) = (positions.len() as i32, positions[0].len() as i32);
    let mut visited_with_directions = HashMap::new();
    for c in ['s', 'e', 'w', 'n'] {
        visited_with_directions.insert(
            c,
            vec![vec![(INFINITY, HashSet::new()); m as usize]; n as usize],
        );
        visited_with_directions.get_mut(&c).unwrap()[start.0 as usize][start.1 as usize].0 = 0;
    }

    let dir = HashMap::from([('e', (0, 1)), ('w', (0, -1)), ('n', (-1, 0)), ('s', (1, 0))]);
    while !pq.is_empty() {
        let (dist, (ux, uy), cur_dir, pred, pred_dir) = pq.pop().unwrap();
        /*
        println!(
            "{} {} {} {} {} {} {}",
            dist, ux, uy, cur_dir, pred.0, pred.1, pred_dir
        );
         */
        let visited = visited_with_directions.get_mut(&cur_dir).unwrap();
        if !start.eq(&(ux, uy)) && -dist == visited[ux as usize][uy as usize].0 {
            visited[ux as usize][uy as usize].1.insert((pred, pred_dir));
        }
        for new_dir in dir.keys() {
            let cost = calculate_direction_cost(&cur_dir, new_dir);
            let (vx, vy) = (
                (ux + dir[new_dir].0) as usize,
                (uy + dir[new_dir].1) as usize,
            );
            let visited = visited_with_directions.get_mut(&new_dir).unwrap();
            if !positions[vx][vy].eq(&'#') && (-dist + 1 + cost) <= visited[vx][vy].0 {
                visited[vx][vy].0 = (-dist) + 1 + cost;
                pq.push((
                    -visited[vx][vy].0,
                    (vx as i32, vy as i32),
                    *new_dir,
                    (ux, uy),
                    cur_dir,
                ));
            }
        }
    }
    let min_dist = visited_with_directions
        .values()
        .map(|x| x[end.0 as usize][end.1 as usize].0)
        .min()
        .unwrap();
    println!("Part 1 - {}", min_dist);

    let mut good_seats: HashSet<(i32, i32)> = HashSet::new();
    let mut q: VecDeque<((i32, i32), char)> = VecDeque::new();
    for c in ['s', 'e', 'w', 'n'] {
        if visited_with_directions.get_mut(&c).unwrap()[end.0 as usize][end.1 as usize].0
            == min_dist
        {
            q.push_back((end, c));
        }
    }
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        //println!("{:?}", cur);
        good_seats.insert(cur.0);
        let visited = visited_with_directions.get_mut(&cur.1).unwrap();
        for pred in &visited[cur.0 .0 as usize][cur.0 .1 as usize].1 {
            q.push_back(*pred);
        }
    }
    println!("Part 2 - {}", good_seats.len());
}

pub fn day16() {
    let mut positions: Vec<Vec<char>> = vec![];
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    for line in read_file("day16.txt") {
        positions.push(line.chars().collect());
        match positions[positions.len() - 1]
            .iter()
            .position(|x| x.eq(&'S') || x.eq(&'E'))
        {
            Some(v) => {
                if positions[positions.len() - 1][v].eq(&'S') {
                    start = (positions.len() as i32 - 1, v as i32);
                } else {
                    end = (positions.len() as i32 - 1, v as i32);
                }
            }
            _ => {}
        }
    }
    solve_day_16(&positions, start, end);
}

fn get_combo_operand_value(operand: &u64, registers: Vec<u64>) -> u64 {
    if *operand <= 3 {
        u64::from(*operand)
    } else if *operand <= 7 {
        registers[(operand - 4) as usize]
    } else {
        panic!("Reserved operand {}", operand)
    }
}

fn run_virtual_machine(
    mut reg_a: u64,
    mut reg_b: u64,
    mut reg_c: u64,
    program: &Vec<u64>,
) -> Vec<u64> {
    let mut int_ptr = 0;
    let mut out_stream = vec![];
    while int_ptr < program.len() {
        let (opcode, operand) = (program[int_ptr], program[int_ptr + 1]);
        match opcode {
            0 => {
                reg_a = reg_a
                    / 2_u64
                        .pow(get_combo_operand_value(&operand, vec![reg_a, reg_b, reg_c]) as u32);
            }
            1 => {
                reg_b = reg_b ^ operand;
            }
            2 => {
                reg_b = get_combo_operand_value(&operand, vec![reg_a, reg_b, reg_c]).rem_euclid(8);
            }
            3 => {
                if reg_a != 0 {
                    int_ptr = operand as usize;
                    continue;
                }
            }
            4 => {
                reg_b = reg_b ^ reg_c;
            }
            5 => {
                out_stream.push(
                    get_combo_operand_value(&operand, vec![reg_a, reg_b, reg_c]).rem_euclid(8),
                );
            }
            6 => {
                reg_b = reg_a
                    / 2_u64
                        .pow(get_combo_operand_value(&operand, vec![reg_a, reg_b, reg_c]) as u32);
            }
            7 => {
                reg_c = reg_a
                    / 2_u64
                        .pow(get_combo_operand_value(&operand, vec![reg_a, reg_b, reg_c]) as u32);
            }
            _ => {
                panic!("Invalid opcode {}", opcode)
            }
        }
        int_ptr = int_ptr + 2;
    }
    out_stream
}

pub fn day17() {
    let register_regex = Regex::new("Register [A-Z]: ([0-9]+)").unwrap();
    let program_regex = Regex::new("Program: ([0-9,]+)").unwrap();
    let lines = read_file("day17.txt");
    let reg_a = u64::from_str(&register_regex.captures(lines[0].as_str()).unwrap()[1]).unwrap();
    let reg_b = u64::from_str(&register_regex.captures(lines[1].as_str()).unwrap()[1]).unwrap();
    let reg_c = u64::from_str(&register_regex.captures(lines[2].as_str()).unwrap()[1]).unwrap();
    let program: Vec<u64> = program_regex.captures(lines[4].as_str()).unwrap()[1]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut out_stream = run_virtual_machine(reg_a, reg_b, reg_c, &program);
    println!(
        "Part 1 - {}",
        out_stream
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
    let mut reg_a_cand: u64 = 0;
    loop {
        out_stream = run_virtual_machine(reg_a_cand, reg_b, reg_c, &program);
        println!("{}'s iteration -  {:?}", reg_a_cand, out_stream);
        if out_stream.eq(&program) || reg_a_cand >= 10000 {
            break;
        }
        reg_a_cand = reg_a_cand + 1;
    }
    println!("Part 2 - {}", reg_a_cand);
    /*
    let mut low = 0;
    let mut high = u64::MAX;
    let mut mid = 0;
    while low < high {
        mid = (low) + (high-low) / 2;
        out_stream = run_virtual_machine(mid, reg_b, reg_c, &program);
        println!("{}'s iteration -  {:?}", mid, out_stream);
        if out_stream.eq(&program) {
            break;
        }
        if out_stream.len() < program.len() {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
     println!("Part 2 - {}", mid);
     */
}

fn check_reachable_bfs(corrupted: &Vec<Vec<bool>>) -> Option<i32> {
    let n = corrupted.len();
    let mut visited = vec![vec![-1; n]; n];
    visited[0][0] = 0;
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let xd: [i32; 4] = [-1, 1, 0, 0];
    let yd: [i32; 4] = [0, 0, -1, 1];
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for i in 0..4 {
            let (x_, y_) = (x + xd[i], y + yd[i]);
            if x_ >= 0
                && x_ < n as i32
                && y_ >= 0
                && y_ < n as i32
                && !corrupted[x_ as usize][y_ as usize]
                && visited[x_ as usize][y_ as usize] < 0
            {
                q.push_back((x_, y_));
                visited[x_ as usize][y_ as usize] = visited[x as usize][y as usize] + 1;
            }
        }
    }
    if visited[n - 1][n - 1] < 0 {
        None
    } else {
        Some(visited[n - 1][n - 1])
    }
}
pub fn day18() {
    let n = 71;
    let mut corrupted = vec![vec![false; n]; n];
    let mut count = 0;
    let mut byte_pos = vec![];
    for l in read_file("day18.txt") {
        count = count + 1;
        let v: Vec<usize> = l.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        byte_pos.push((v[0], v[1]));
    }
    for i in 0..byte_pos.len() {
        corrupted[byte_pos[i].0][byte_pos[i].1] = true;
        if i >= 1024 {
            match check_reachable_bfs(&corrupted) {
                Some(v) => {
                    if i == 1024 {
                        println!("Part 1 - {}", v);
                    }
                }
                None => {
                    println!("Part 2 - {},{}", byte_pos[i].0, byte_pos[i].1);
                    break;
                }
            }
        }
    }
}

pub fn day19() {
    let lines = read_file("day19.txt");
    let atoms: HashMap<&str, bool> =
        HashMap::from_iter(lines[0].split(',').map(|x| x.trim()).map(|x| (x, true)));
    let mut molecules: HashMap<&str, u64> = HashMap::new();
    let mut part_1_ans = 0;
    let mut part_2_ans = 0;
    for i in 2..lines.len() {
        let count = can_be_designed(&atoms, &mut molecules, lines[i].as_str());
        if count > 0 {
            //println!("Checking for {}", lines[i]);
            part_1_ans = part_1_ans + 1;
        }
        part_2_ans = part_2_ans + count;
    }
    println!("Part 1 - {}", part_1_ans);
    println!("Part 2 - {}", part_2_ans);
}

fn can_be_designed<'a>(
    atoms: &HashMap<&str, bool>,
    molecules: &mut HashMap<&'a str, u64>,
    candidate: &'a str,
) -> u64 {
    if candidate.is_empty() {
        return 1;
    }
    if molecules.contains_key(candidate) {
        return molecules[candidate];
    }
    if atoms.contains_key(candidate) {
        molecules.insert(candidate, 1);
    } else {
        molecules.insert(candidate, 0);
    }
    for i in 1..candidate.len() {
        if atoms.contains_key(&candidate[0..i]) {
            *molecules.get_mut(candidate).unwrap() +=
                can_be_designed(atoms, molecules, &candidate[i..]);
        }
    }
    //println!("{} {}", candidate, molecules[candidate]);
    molecules[candidate]
}

fn min_picoseconds(matrix: &Vec<Vec<char>>, s: (usize, usize), e: (usize, usize)) -> u64 {
    let mut q = VecDeque::new();
    let (n, m) = (matrix.len() as i32, matrix[0].len() as i32);
    q.push_back((s.0 as i32, s.1 as i32));
    let mut dist = HashMap::new();
    dist.insert(s, 0);
    let xd = [-1, 1, 0, 0];
    let yd = [0, 0, -1, 1];
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        if v.eq(&(e.0 as i32, e.1 as i32)) {
            break;
        }
        for i in 0..4 {
            let (x_, y_) = (v.0 + xd[i], v.1 + yd[i]);
            if x_ >= 0
                && x_ < n
                && y_ >= 0
                && y_ < m
                && !matrix[x_ as usize][y_ as usize].eq(&'#')
                && !dist.contains_key(&(x_ as usize, y_ as usize))
            {
                dist.insert(
                    (x_ as usize, y_ as usize),
                    dist[&(v.0 as usize, v.1 as usize)] + 1,
                );
                q.push_back((x_, y_));
            }
        }
    }
    dist[&e]
}

pub fn day20() {
    let mut matrix: Vec<Vec<char>> = vec![];
    let (mut s, mut e) = ((0, 0), (0, 0));
    for l in read_file("day20.txt") {
        matrix.push(l.chars().collect());
        if let Some(p) = matrix
            .last()
            .unwrap()
            .iter()
            .position(|x| x.eq(&'S') || x.eq(&'E'))
        {
            if matrix.last().unwrap()[p].eq(&'S') {
                s = (matrix.len() - 1, p);
            } else {
                e = (matrix.len() - 1, p);
            }
        }
    }
    let (n, m) = (matrix.len(), matrix[0].len());
    //println!("{:?} {:?}", s, e);
    let original_time = min_picoseconds(&matrix, s, e);
    println!("Original time - {}", original_time);
    let mut part_1_ans = 0;
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j].eq(&'#') {
                matrix[i][j] = '.';
                let cur_time = min_picoseconds(&matrix, s, e);
                println!("New time with {},{} is {}", i, j, cur_time);
                if cur_time < original_time && original_time - cur_time >= 100 {
                    part_1_ans = part_1_ans + 1;
                }
                matrix[i][j] = '#';
            }
        }
    }

    println!("Part 1 - {}", part_1_ans);
}

fn perform_binary_op(op: &str, v1: &u64, v2: &u64) -> u64 {
    let  ans ;
    match op {
        "AND" => {
            ans = v1 & v2
        }
        "OR" => {
            ans = v1 | v2
        }
        "XOR" => {
            ans = v1 ^ v2
        }
        _ => {
            panic!("Unexpected op: {}", op);
        }
    }
    //println!("{} {} {}: {}", v1, op, v2, ans);
    ans
}

fn update_graph<'a >(adj_list: &mut HashMap<&'a str, Vec<&'a str>>, u: &'a str, v: &'a str) {
    match adj_list.get_mut(u) {
        Some(list) => {
            list.push(v);
        }
        None => {
            let list = vec![v];
            adj_list.insert(u, list);

        }
    }
}

pub fn day23() {
    let lines = read_file("day23.txt");
    let link_pattern = Regex::new("([[:alnum:]]+)-([[:alnum:]]+)").unwrap();
    let mut adj_list:HashMap<&str, Vec<&str>> = HashMap::new();
    for i in 0..lines.len() {
        let c = link_pattern.captures(lines[i].as_str()).unwrap();
        let (u, v) = (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str());
        //println!("{} {}", u, v);
        update_graph(&mut adj_list, u, v);
        update_graph(&mut adj_list, v, u);
    }
    println!("{:?}", adj_list);
    let mut unique_tuple = HashSet::new();
    for k in adj_list.keys() {
        if k.starts_with('t') {
            println!("{}", k);
            let neighbours = adj_list.get(k).unwrap();
            for i in 0..neighbours.len() {
                for j in i+1..neighbours.len() {
                    if adj_list.get(neighbours[i]).unwrap().contains(&neighbours[j]) {
                        let mut tuple = [k, neighbours[i], neighbours[j]];
                        tuple.sort();
                        unique_tuple.insert(tuple);
                    }
                }
            }
        }
    }
    println!("{:?}", unique_tuple);
    println!("Part 1 - {}", unique_tuple.len());
}

pub fn day24() {
    let lines = read_file("day24.txt");
    let reg_pattern = Regex::new("([[:alnum:]]+): ([0-9]+)").unwrap();
    let mut reg_values = HashMap::new();
    let op_pattern =
        Regex::new("([[:alnum:]]+) ([[:alnum:]]+) ([[:alnum:]]+) -> ([[:alnum:]]+)").unwrap();
    let mut pending_ops = HashSet::new();
    for i in 0..lines.len() {
        let l_str = lines[i].as_str();
        match reg_pattern.captures(l_str) {
            Some(c) => {
                let (k, v) = (
                    c.get(1).unwrap().as_str(),
                    c.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                );
                reg_values.insert(k, v);
            }
            None => {}
        }
        match op_pattern.captures(l_str) {
            Some(c) => {
                let (op1, op, op2, res) = (
                    c.get(1).unwrap().as_str(),
                    c.get(2).unwrap().as_str(),
                    c.get(3).unwrap().as_str(),
                    c.get(4).unwrap().as_str(),
                );
                pending_ops.insert((op1, op, op2, res));
            }
            None => {}
        }
    }
    while !pending_ops.is_empty() {
        pending_ops.retain(|ins| {
            let (op1, op, op2, res) = (ins.0, ins.1, ins.2, ins.3);
            if reg_values.contains_key(op1) && reg_values.contains_key(op2) {
                let (v1, v2) = (reg_values.get(op1).unwrap(), reg_values.get(op2).unwrap());
                reg_values.insert(res, perform_binary_op(op, v1, v2));
                false
            } else {
                true
            }
        });
    }
    let mut counter = 99;
    let mut part_1_ans: u64 = 0;
    while counter >= 0 {
        let k = format!("z{:02}", counter);
        if reg_values.contains_key(k.as_str()){
            let v = reg_values.get(k.as_str()).unwrap();
            //println!("{} {}", k, v);
            part_1_ans = 2 * part_1_ans + v;
        }
        counter = counter - 1;
    }
    println!("Part 1 - {}", part_1_ans);

}

pub fn day25() {
    let lines = read_file("day25.txt");
    let (mut locks, mut keys) = (HashSet::new(), HashSet::new());
    let mut i = 0;
    while i < lines.len() {
        let mut v = vec![-1; 5];
        let mut is_lock = false;
        if lines[i].contains('#') {
            is_lock = true;
        }
        for l in lines[i..i + 7].iter() {
            for c in l.chars().enumerate() {
                if c.1.eq(&'#') {
                    v[c.0] = v[c.0] + 1;
                }
            }
        }
        if is_lock {
            locks.insert(v);
        } else {
            keys.insert(v);
        }
        i = i + 8;
    }
    //println!("{:?}", locks);
    //println!("{:?}", keys);
    let mut part_1_ans = 0;
    for l in locks {
        for k in keys.iter() {
            if l.iter().zip(k.iter()).all(|(x, y)| x + y <= 5) {
                part_1_ans = part_1_ans + 1;
            }
        }
    }
    println!("Part 1 - {}", part_1_ans);
}
