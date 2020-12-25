use error_chain::error_chain;
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::hash::Hash;
use structopt::StructOpt;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Pie(std::num::ParseIntError);
        Pce(std::char::ParseCharError);
    }
}

#[derive(StructOpt)]
struct Cli {
    day_number: i16,
    input_file: std::path::PathBuf,
}

fn day1(input: &str) -> Result<()> {
    let xs: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    for x in &xs {
        for y in &xs {
            for z in &xs {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

fn d2_is_c(password: &str, index: i32, c: char) -> bool {
    let index = index as usize;
    let password = password.as_bytes();
    let c = c as u8;
    1 <= index && index <= password.len() && password[index - 1] == c
}

fn day2(input: &str) -> Result<()> {
    let mut ok1_count = 0;
    let mut ok2_count = 0;
    for line in input.lines() {
        let mut it = line.split(|c| "-: ".contains(c));
        let minimum: i32 = it.next().ok_or("d2")?.parse()?;
        let maximum: i32 = it.next().ok_or("d2")?.parse()?;
        let c: char = it.next().ok_or("d2")?.parse()?;
        it.next();
        let password = it.next().ok_or("d2")?;
        let c_count = password.chars().filter(|d| c == *d).count() as i32;
        if minimum <= c_count && c_count <= maximum {
            ok1_count += 1;
        }
        if d2_is_c(password, minimum, c) ^ d2_is_c(password, maximum, c) {
            ok2_count += 1;
        }
    }
    println!("ok1: {}", ok1_count);
    println!("ok2: {}", ok2_count);
    Ok(())
}

fn d3_is_tree(line: &str, i: usize) -> bool {
    match line.chars().nth(i) {
        Some('#') => true,
        Some('.') => false,
        _ => panic!("d3"),
    }
}

fn day3(input: &str) -> Result<()> {
    let mut line_cnt = 0;
    let mut index1 = vec![0, 0, 0, 0];
    let mut index2 = 0;
    let xs = vec![1, 3, 5, 7];
    let mut count1: Vec<i64> = vec![0, 0, 0, 0];
    let mut count2: i64 = 0;
    for line in input.lines() {
        for i in 0..4 {
            if d3_is_tree(line, index1[i]) {
                count1[i] += 1;
            }
            index1[i] = (index1[i] + xs[i]) % line.len();
        }
        if line_cnt % 2 == 0 {
            if d3_is_tree(line, index2) {
                count2 += 1;
            }
            index2 = (index2 + 1) % line.len();
        }
        line_cnt += 1
    }
    println!("count1: {}", count1[1]);
    println!("prod: {}", count1.iter().fold(count2, |a, b| a * b));
    Ok(())
}

fn day4(input: &str) -> Result<()> {
    let required: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();
    let mut valid = 0;
    let mut super_valid = 0;

    let passports: Vec<Vec<Vec<&str>>> = input
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .map(|f| f.split(':').collect())
                .collect()
        })
        .collect();
    for p in passports {
        let q: HashMap<&str, &str> = p.iter().map(|v| (v[0], v[1])).collect();
        if required.is_subset(&q.keys().cloned().collect()) {
            valid += 1;
            if d5_is_valid(&q)? {
                super_valid += 1;
            }
        }
    }
    println!("valid {}", valid);
    println!("super_valid {}", super_valid);
    Ok(())
}

fn d5_num_in(s: &str, a: i32, b: i32) -> bool {
    match s.parse() {
        Ok(x) => a <= x && x <= b,
        Err(_) => false,
    }
}

fn d5_is_valid(passport: &HashMap<&str, &str>) -> Result<bool> {
    if !d5_num_in(passport.get("byr").ok_or("d5")?, 1920, 2002) {
        return Ok(false);
    }
    if !d5_num_in(passport.get("iyr").ok_or("d5")?, 2010, 2020) {
        return Ok(false);
    }
    if !d5_num_in(passport.get("eyr").ok_or("d5")?, 2020, 2030) {
        return Ok(false);
    }
    let hgt = passport.get("hgt").ok_or("d5")?;
    if hgt.len() < 3 {
        return Ok(false);
    }
    match &hgt[hgt.len() - 2..] {
        "in" => {
            if !d5_num_in(&hgt[..hgt.len() - 2], 59, 76) {
                return Ok(false);
            }
        }
        "cm" => {
            if !d5_num_in(&hgt[..hgt.len() - 2], 150, 193) {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }
    let hcl = passport.get("hcl").ok_or("d5")?;
    if hcl.len() != 7 || hcl.chars().nth(0).ok_or("d5")? != '#' {
        return Ok(false);
    }
    if !hcl[1..].chars().all(|c| "0123456789abcdef".contains(c)) {
        return Ok(false);
    }
    let ecl = passport.get("ecl").ok_or("d5")?;
    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .find(|e| *e == ecl)
        .is_none()
    {
        return Ok(false);
    }
    let pid = passport.get("pid").ok_or("d5")?;
    if pid.len() != 9 {
        return Ok(false);
    }
    if !pid.chars().all(|c| c.is_ascii_digit()) {
        return Ok(false);
    }
    Ok(true)
}

fn day5(input: &str) -> Result<()> {
    let mut seats: Vec<i32> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'B' | 'R' => 1,
                    _ => 0,
                })
                .fold(0, |n, i| 2 * n + i)
        })
        .collect();
    let m = *seats.iter().max().ok_or("d5")?;
    seats.sort();
    for i in 1..seats.len() {
        if seats[i - 1] + 1 < seats[i] {
            println!("missing between {} and {}", seats[i - 1], seats[i]);
        }
    }
    println!("{}", m);
    Ok(())
}

fn day6(input: &str) -> Result<()> {
    let xss: Vec<Vec<HashSet<char>>> = input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.chars().collect()).collect())
        .collect();
    let part1: usize = xss
        .iter()
        .map(|xs| {
            xs.iter()
                .fold(HashSet::new(), |a, b| a.union(&b).cloned().collect())
                .len()
        })
        .sum();
    let part2: usize = xss
        .iter()
        .map(|xs| {
            xs.iter()
                .fold(('a'..='z').collect(), |a: HashSet<char>, b| {
                    a.intersection(&b).cloned().collect()
                })
                .len()
        })
        .sum();
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
    Ok(())
}

fn d7_add<Vertex: Eq + Hash, Arc>(graph: &mut HashMap<Vertex, Vec<Arc>>, source: Vertex, arc: Arc) {
    let mut old_arcs = match graph.remove(&source) {
        Some(old_arcs) => old_arcs,
        None => vec![],
    };
    old_arcs.push(arc);
    graph.insert(source, old_arcs);
}

fn d7_dfs<V: Eq, C>(
    graph: &HashMap<V, Vec<(i32, V)>>,
    cache: &mut HashMap<V, i32>,
    combine: &C,
    x: &V,
) -> i32
where
    V: Clone + Hash,
    C: Fn(&Vec<(i32, i32)>) -> i32,
{
    let empty = vec![];
    match cache.get(x) {
        Some(r) => *r,
        None => {
            let children: &Vec<(i32, V)> = match graph.get(x) {
                Some(children) => children,
                None => &empty,
            };
            let results: Vec<(i32, i32)> = children
                .iter()
                .map(|(w, y)| (*w, d7_dfs(graph, cache, combine, y)))
                .collect();
            let r = combine(&results);
            cache.insert(x.clone(), r);
            r
        }
    }
}

fn day7(input: &str) -> Result<()> {
    let (g, g_rev) = {
        let mut g: HashMap<(&str, &str), Vec<(i32, (&str, &str))>> = HashMap::new();
        let mut g_rev: HashMap<(&str, &str), Vec<(i32, (&str, &str))>> = HashMap::new();
        for line in input.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            let src: (&str, &str) = (words[0], words[1]);
            let mut i = 4;
            while i < words.len() && words[i] != "no" {
                let tgt: (&str, &str) = (words[i + 1], words[i + 2]);
                let cnt: i32 = words[i].parse()?;
                d7_add(&mut g, src, (cnt, tgt));
                d7_add(&mut g_rev, tgt, (0, src));
                i += 4;
            }
        }
        (g, g_rev)
    };
    let sg = ("shiny", "gold");
    let part1 = {
        let mut cache = HashMap::new();
        d7_dfs(&g_rev, &mut cache, &|_| 0, &sg);
        cache.len() - 1
    };
    let part2 = {
        let mut cache = HashMap::new();
        d7_dfs(
            &g,
            &mut cache,
            &|xs| xs.iter().fold(1, |total, (weight, c)| total + weight * c),
            &sg,
        ) - 1
    };
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn d8_execute(program: &Vec<(&str, i32)>) -> Result<(i32, i32)> {
    let mut ic: i32 = 0;
    let mut acc = 0;
    let mut seen = HashSet::new();
    let n = program.len() as i32;
    while 0 <= ic && ic < n {
        if seen.contains(&ic) {
            return Ok((ic, acc));
        }
        seen.insert(ic);
        let (op, v) = &program[ic as usize];
        match *op {
            "nop" => ic += 1,
            "jmp" => ic += v,
            "acc" => {
                acc += v;
                ic += 1
            }
            _ => panic!("unknown instruction"),
        }
    }
    Ok((ic, acc))
}

fn day8(input: &str) -> Result<()> {
    let mut program: Vec<(&str, i32)> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .map(|ws: Vec<&str>| (ws[0], ws[1].parse().unwrap()))
        .collect();
    let (_, acc1) = d8_execute(&program)?;
    let n = program.len() as i32;
    let mut i = 0;
    let acc2 = loop {
        match program[i as usize].0 {
            "jmp" => {
                program[i as usize] = ("nop", program[i as usize].1);
                let (m, acc2) = d8_execute(&program)?;
                if m == n {
                    break acc2;
                }
                program[i as usize] = ("jmp", program[i as usize].1)
            }
            "nop" => {
                program[i as usize] = ("jmp", program[i as usize].1);
                let (m, acc2) = d8_execute(&program)?;
                if m == n {
                    break acc2;
                }
                program[i as usize] = ("nop", program[i as usize].1)
            }
            _ => (),
        }
        i += 1;
    };
    println!("part1: {}", acc1);
    println!("part2: {}", acc2);
    Ok(())
}

fn d9_get(h: &HashMap<i64, i32>, x: i64) -> i32 {
    match h.get(&x) {
        Some(count) => *count,
        None => 0,
    }
}

fn d9_inc(h: &mut HashMap<i64, i32>, x: i64) {
    h.insert(x, d9_get(h, x) + 1);
}

fn d9_dec(h: &mut HashMap<i64, i32>, x: i64) {
    let v = d9_get(h, x) - 1;
    if v == 0 {
        h.remove(&x);
    } else {
        h.insert(x, v);
    }
}

/** The extra work to lower complexity is just for fun, and completely unneeded
for the given problem sizes. */
fn day9(input: &str) -> Result<()> {
    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let window = 25;
    let mut sums = HashMap::new();
    for j in 0..window {
        for i in 0..j {
            d9_inc(&mut sums, numbers[i] + numbers[j]);
        }
    }
    let magic = {
        // O(window * numbers.len())
        let mut k = window;
        loop {
            if d9_get(&sums, numbers[k]) == 0 {
                break numbers[k];
            }
            for i in k - window + 1..k {
                d9_dec(&mut sums, numbers[k - window] + numbers[i]);
                d9_inc(&mut sums, numbers[k] + numbers[i]);
            }
            k += 1;
        }
    };
    let weakness = {
        // O(numbers.len())
        let mut position_of_sum = HashMap::new();
        position_of_sum.insert(0, 0);
        let (i, j) = {
            let mut k = 0;
            let mut sum = 0;
            loop {
                match position_of_sum.get(&(sum - magic)) {
                    Some(i) => break (*i, k),
                    None => {
                        sum += numbers[k];
                        k += 1;
                        position_of_sum.insert(sum, k);
                    }
                }
            }
        };
        numbers[i..j].iter().min().ok_or("d9")? + numbers[i..j].iter().max().ok_or("d9")?
    };
    println!("part1: {}", magic);
    println!("part2: {}", weakness);
    Ok(())
}

fn day10(input: &str) -> Result<()> {
    let xs: Vec<i32> = {
        let mut ys: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        ys.push(0);
        ys.push(3 + *ys.iter().max().ok_or("d10")?);
        ys.sort_unstable();
        ys
    };
    let part1 = {
        let diffs: Vec<i32> = (1..xs.len()).map(|i| xs[i] - xs[i - 1]).collect();
        let c1 = diffs.iter().filter(|x| **x == 1).count();
        let c3 = diffs.iter().filter(|x| **x == 3).count();
        c1 * c3
    };
    let part2 = {
        let mut count: Vec<i64> = vec![1];
        for i in 1..xs.len() {
            let mut c = 0;
            let mut k = 1;
            while k <= i && xs[i] - xs[i - k] <= 3 {
                c += count[i - k];
                k += 1;
            }
            count.push(c);
        }
        count[count.len() - 1]
    };

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn d11_count_adjacent(map: &Vec<String>, x: usize, y: usize) -> i32 {
    let x = x as i32;
    let y = y as i32;
    let m = map.len() as i32;
    let n = map[0].len() as i32;
    let mut r = 0;
    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            if 0 <= i
                && i < m
                && 0 <= j
                && j < n
                && (i != x || j != y)
                && map[i as usize].as_bytes()[j as usize] == '#' as u8
            {
                r += 1;
            }
        }
    }
    r
}

fn d11_count_visible(map: &Vec<String>, x: usize, y: usize) -> i32 {
    let x = x as i32;
    let y = y as i32;
    let m = map.len() as i32;
    let n = map[0].len() as i32;
    let mut r = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            for k in 1.. {
                let xx = x + k * dx;
                let yy = y + k * dy;
                if !(0 <= xx && xx < m && 0 <= yy && yy < n) {
                    break;
                }
                match map[xx as usize].as_bytes()[yy as usize] as char {
                    'L' => break,
                    '#' => {
                        r += 1;
                        break;
                    }
                    _ => (),
                }
            }
        }
    }
    r
}

fn d11_solve<C>(map: Vec<String>, limit: i32, count: C) -> Result<i32>
where
    C: Fn(&Vec<String>, usize, usize) -> i32,
{
    let mut new_map = vec![];
    for i in 0..map.len() {
        let mut new_row = String::new();
        for j in 0..map[i].len() {
            let neighbors = count(&map, i, j);
            new_row.push(if map[i].as_bytes()[j] == 'L' as u8 && neighbors == 0 {
                '#'
            } else if map[i].as_bytes()[j] == '#' as u8 && neighbors >= limit {
                'L'
            } else {
                map[i].as_bytes()[j] as char
            });
        }
        new_map.push(new_row);
    }
    if new_map == map {
        let r: usize = map
            .iter()
            .map(|l| l.chars().filter(|c| *c == '#').count())
            .sum();
        Ok(r as i32)
    } else {
        d11_solve(new_map, limit, count)
    }
}

fn day11(input: &str) -> Result<()> {
    let map: Vec<String> = input.lines().map(|x| String::from(x)).collect();
    let part1 = d11_solve(map.clone(), 4, d11_count_adjacent)?;
    let part2 = d11_solve(map.clone(), 5, d11_count_visible)?;
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

#[derive(Clone, Debug)]
struct V2 {
    x: f64,
    y: f64,
}

impl V2 {
    fn rotate(&self, alpha: f64) -> Self {
        let x = alpha.cos() * self.x - alpha.sin() * self.y;
        let y = alpha.sin() * self.x + alpha.cos() * self.y;
        Self { x, y }
    }
    fn add(&self, other: &Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Self { x, y }
    }
    fn norm1(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }
    fn scale(&self, lambda: f64) -> Self {
        let x = lambda * self.x;
        let y = lambda * self.y;
        Self { x, y }
    }
}

fn d12_go(instructions: &Vec<(char, i32)>, part2: bool) -> f64 {
    let mut position = V2 { x: 0.0, y: 0.0 };
    let mut waypoint = if part2 {
        V2 { x: 10., y: 1. }
    } else {
        V2 { x: 1.0, y: 0.0 }
    };
    let absolute: HashMap<char, V2> = [
        ('N', V2 { x: 0., y: 1. }),
        ('S', V2 { x: 0., y: -1. }),
        ('E', V2 { x: 1., y: 0. }),
        ('W', V2 { x: -1., y: 0. }),
    ]
    .iter()
    .cloned()
    .collect();
    for (op, num) in instructions {
        let num = *num as f64;
        {
            let w = if part2 { &mut waypoint } else { &mut position };
            absolute.get(op).map(|v| *w = w.add(&v.scale(num)));
        }
        match op {
            'L' => waypoint = waypoint.rotate(num * PI / 180.),
            'R' => waypoint = waypoint.rotate(-num * PI / 180.),
            'F' => position = position.add(&waypoint.scale(num)),
            _ => (),
        }
        if false {
            println!("{:?}", position)
        };
    }
    position.norm1()
}

fn day12(input: &str) -> Result<()> {
    let instructions: Vec<(char, i32)> = input
        .split_whitespace()
        .map(|l| (l[..1].parse().unwrap(), l[1..].parse().unwrap()))
        .collect();
    let part1 = d12_go(&instructions, false);
    let part2 = d12_go(&instructions, true);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn day13(input: &str) -> Result<()> {
    let (start, constraints) = {
        let ls: Vec<&str> = input.lines().collect();
        let start: i64 = ls[0].parse()?;
        let constraints: Vec<(i64, i64)> = ls[1]
            .split(',')
            .enumerate()
            .map(|(i, x)| {
                if x == "x" {
                    None
                } else {
                    Some((i as i64, x.parse().unwrap()))
                }
            })
            .filter_map(|ix| ix)
            .collect();
        (start, constraints)
    };
    let part1 = {
        let (wait, m) = constraints
            .iter()
            .map(|(_, m)| (m - start % m, m))
            .min()
            .ok_or("d13")?;
        wait * m
    };
    let part2 = {
        let mut ans: i64 = 0;
        let mut lcm: i64 = 1;
        for (k, m) in constraints {
            while (ans + k) % m != 0 {
                ans += lcm;
            }
            lcm = lcm / gcd(lcm, m) * m;
        }
        ans
    };
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

#[derive(Debug)]
enum D14I {
    Mask(String),
    Write(i64, i64),
}

fn d14_solve(instructions: &Vec<D14I>, part2: bool) -> i64 {
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut m0: i64 = 0;
    let mut m1: i64 = 0;
    for instr in instructions {
        match instr {
            D14I::Mask(mstr) => {
                m0 = 0;
                m1 = 0;
                for c in mstr.chars() {
                    m0 = 2 * m0;
                    m1 = 2 * m1 + 1;
                    match c {
                        '0' => {
                            if !part2 {
                                m1 -= 1
                            }
                        }
                        '1' => m0 += 1,
                        'X' => {
                            if part2 {
                                m0 += 1;
                                m1 -= 1
                            }
                        }
                        _ => panic!("d14"),
                    }
                }
            }
            D14I::Write(addr, val) => {
                if part2 {
                    let mut m1c = m1;
                    loop {
                        memory.insert((addr | m0) & m1c, *val);
                        if m1c == (1 << 36) - 1 {
                            break;
                        }
                        m1c = (m1c + 1) | m1;
                    }
                } else {
                    memory.insert(*addr, (val | m0) & m1);
                }
            }
        }
    }
    memory.values().sum()
}

fn day14(input: &str) -> Result<()> {
    let instructions = {
        let mut instructions = vec![];
        for l in input.lines() {
            let ws: Vec<&str> = l.split(|c| "[] ".contains(c)).collect();
            if ws[0] == "mask" {
                instructions.push(D14I::Mask(String::from(ws[2])));
            } else {
                instructions.push(D14I::Write(ws[1].parse().unwrap(), ws[4].parse().unwrap()));
            }
        }
        instructions
    };
    let part1 = d14_solve(&instructions, false);
    let part2 = d14_solve(&instructions, true);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn d15_solve(xs: &Vec<i32>, limit: usize) -> usize {
    let mut last: HashMap<usize, usize> = xs[..xs.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, x)| (*x as usize, i))
        .collect();
    let mut now = xs[xs.len() - 1] as usize;
    for i in xs.len() - 1..limit - 1 {
        let nxt = match last.get(&now) {
            Some(j) => i - j,
            None => 0,
        };
        last.insert(now, i);
        now = nxt;
    }
    now
}

fn day15(input: &str) -> Result<()> {
    let xs: Vec<i32> = input
        .trim()
        .split(',')
        .map(|w| w.parse().unwrap())
        .collect();
    let part1 = d15_solve(&xs, 2020);
    let part2 = d15_solve(&xs, 30_000_000);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn d16_is_valid(ranges: &Vec<i32>, val: i32) -> bool {
    match ranges.binary_search(&val) {
        Ok(i) => i % 2 == 0,
        Err(i) => i % 2 != 0,
    }
}

fn day16(input: &str) -> Result<()> {
    let (fields, your_ticket, nearby_tickets) = {
        let sections: Vec<&str> = input.split("\n\n").collect();
        let fields: Vec<Vec<&str>> = sections[0]
            .lines()
            .map(|l| l.split(':').collect())
            .collect();
        let fields: Vec<(&str, Vec<&str>)> = fields
            .iter()
            .map(|v| (v[0], v[1].split("or").collect()))
            .collect();
        let fields: HashMap<&str, Vec<(i32, i32)>> = fields
            .iter()
            .map(|(f, rs)| {
                (
                    *f,
                    rs.iter()
                        .map(|r| {
                            let lh: Vec<&str> = r.split('-').collect();
                            (lh[0].trim().parse().unwrap(), lh[1].trim().parse().unwrap())
                        })
                        .collect(),
                )
            })
            .collect();
        let your_ticket: Vec<&str> = sections[1].lines().collect();
        let your_ticket: Vec<i32> = your_ticket[1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let nearby_tickets: Vec<Vec<i32>> = sections[2]
            .lines()
            .skip(1)
            .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();
        (fields, your_ticket, nearby_tickets)
    };
    let (part1, nearby_tickets) = {
        // playing here: fast validity check isn't needed
        let mut ends: Vec<(i32, i32)> = vec![];
        for lims in fields.values() {
            for (l, h) in lims {
                ends.push((l + 0, -1));
                ends.push((h + 1, 1));
            }
        }
        ends.sort();
        let mut valid_ranges: Vec<i32> = vec![];
        let mut count = 0;
        for (val, typ) in ends {
            let new_count = count - typ;
            if (count == 0) != (new_count == 0) {
                valid_ranges.push(val);
            }
            count = new_count;
        }
        if valid_ranges.len() % 2 != 0 {
            panic!("d16");
        }
        let mut ans = 0;
        let mut valid_tickets = vec![];
        for t in nearby_tickets {
            let mut bad = false;
            for v in &t {
                if !d16_is_valid(&valid_ranges, *v) {
                    ans += v;
                    bad = true;
                }
            }
            if !bad {
                valid_tickets.push(t);
            }
        }
        (ans, valid_tickets)
    };
    let part2 = {
        let n = nearby_tickets[0].len();
        let mut possibilities: Vec<HashSet<&str>> = vec![];
        for i in 0..n {
            let mut could_be: HashSet<&str> = fields.keys().map(|x| *x).collect();
            for t in &nearby_tickets {
                let x = t[i];
                could_be.retain(|&f| {
                    fields
                        .get(f)
                        .unwrap()
                        .iter()
                        .any(|(l, h)| *l <= x && x <= *h)
                });
            }
            possibilities.push(could_be);
        }
        // TODO: Would be fun to implement this similar to UP in SAT.
        let mut todo: VecDeque<&str> = VecDeque::new();
        for could_be in &possibilities {
            if could_be.len() == 1 {
                todo.push_back(could_be.iter().nth(0).unwrap());
            }
        }
        while let Some(f) = todo.pop_front() {
            for could_be in &mut possibilities {
                if could_be.len() > 1 {
                    could_be.remove(f);
                    if could_be.len() == 1 {
                        todo.push_back(could_be.iter().nth(0).unwrap());
                    }
                }
            }
        }
        let mut ans: i64 = 1;
        for i in 0..n {
            if possibilities[i]
                .iter()
                .nth(0)
                .unwrap()
                .starts_with("departure")
            {
                ans *= your_ticket[i] as i64;
            }
        }
        ans
    };
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn d17_start(init: &HashSet<(i8, i8)>, dimension: usize) -> HashSet<Vec<i8>> {
    init.iter()
        .map(|(x, y)| {
            let mut r = vec![0; dimension];
            r[0] = *x;
            r[1] = *y;
            r
        })
        .collect()
}

fn d17_step(prev: HashSet<Vec<i8>>, dimension: usize) -> HashSet<Vec<i8>> {
    let (min, max) = {
        let mut min = vec![i8::MAX; dimension];
        let mut max = vec![i8::MIN; dimension];
        for i in 0..dimension {
            for p in &prev {
                min[i] = min[i].min(p[i] - 1);
                max[i] = max[i].max(p[i] + 1);
            }
        }
        (min, max)
    };
    let mut result = HashSet::new();
    let mut p = min.clone();
    loop {
        let mut neighbors = 0;
        let mut q: Vec<i8> = vec![-1; dimension];
        loop {
            if q.iter().any(|x| *x != 0) {
                let pq: Vec<i8> = p.iter().zip(q.iter()).map(|(a, b)| a + b).collect();
                if prev.contains(&pq) {
                    neighbors += 1
                }
            }
            let mut i = 0;
            while i < dimension && q[i] == 1 {
                q[i] = -1;
                i += 1;
            }
            if i < dimension {
                q[i] += 1;
            } else {
                break;
            }
        }
        let active = prev.contains(&p) && (neighbors == 2 || neighbors == 3);
        let active = active || (!prev.contains(&p) && neighbors == 3);
        if active {
            result.insert(p.clone());
        }
        let mut i = 0;
        while i < dimension && p[i] == max[i] {
            p[i] = min[i];
            i += 1;
        }
        if i < dimension {
            p[i] += 1;
        } else {
            break;
        }
    }
    result
}

fn d17_solve(init: &HashSet<(i8, i8)>, dimension: usize) -> usize {
    let mut state = d17_start(init, dimension);
    for _ in 0..6 {
        state = d17_step(state, dimension);
    }
    state.len()
}

fn day17(input: &str) -> Result<()> {
    let active: HashSet<(i8, i8)> = {
        let mut result = HashSet::new();
        let mut x = 0;
        for line in input.lines() {
            let mut y = 0;
            for c in line.chars() {
                if c == '#' {
                    result.insert((x, y));
                }
                y += 1;
            }
            x += 1;
        }
        result
    };
    let part1 = d17_solve(&active, 3);
    let part2 = d17_solve(&active, 4);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum D18Tok {
    Open,
    Close,
    Add,
    Mul,
    Num(i64),
}

fn d18_atom(expr: &Vec<D18Tok>, i: usize, precedence: bool) -> (i64, usize) {
    match expr[i] {
        D18Tok::Open => {
            let (x, i) = d18_eval(expr, i + 1, precedence);
            (x, i + 1)
        }
        D18Tok::Num(x) => (x, i + 1),
        _ => panic!("d18"),
    }
}

fn d18_eval(expr: &Vec<D18Tok>, i: usize, precedence: bool) -> (i64, usize) {
    let mut p: i64 = 1;
    let (mut s, mut i) = d18_atom(expr, i, precedence);
    while expr[i] != D18Tok::Close {
        let (x, j) = d18_atom(expr, i + 1, precedence);
        match expr[i] {
            D18Tok::Add => s += x,
            D18Tok::Mul => {
                if precedence {
                    p *= s;
                    s = x;
                } else {
                    s *= x
                }
            }
            _ => panic!("d18"),
        }
        i = j;
    }
    (p * s, i)
}

fn day18(input: &str) -> Result<()> {
    let input = String::from("((") + &input.trim().replace("\n", ") + (") + "))";
    let input = input.replace("(", "( ").replace(")", " )");
    let input: Vec<D18Tok> = input
        .split_whitespace()
        .map(|t| match t {
            "(" => D18Tok::Open,
            ")" => D18Tok::Close,
            "+" => D18Tok::Add,
            "*" => D18Tok::Mul,
            x => D18Tok::Num(x.parse().unwrap()),
        })
        .collect();
    let (part1, _) = d18_eval(&input, 1, false);
    let (part2, _) = d18_eval(&input, 1, true);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

#[derive(Clone, Debug)]
enum D19Rule {
    N(Vec<usize>),
    T(char),
}

type D19I = usize;
type D19N = usize;
type D19Grammar = HashMap<D19N, Vec<D19Rule>>;
type D19Range = (D19I, D19I);
struct D19Result {
    known_yes: HashSet<D19N>,
    known_no: HashSet<D19N>,
}
type D19Cache = HashMap<D19Range, D19Result>;

fn d19_parse_rules(input: &str) -> D19Grammar {
    let mut result = D19Grammar::new();
    for line in input.lines() {
        let ws: Vec<&str> = line.split(':').collect();
        let lhs: D19N = ws[0].parse().unwrap();
        let mut rhs = vec![];
        for alt in ws[1].trim().split('|') {
            let alt = alt.trim();
            if alt.starts_with("\"") {
                rhs.push(D19Rule::T(alt.chars().nth(1).unwrap()));
            } else {
                rhs.push(D19Rule::N(
                    alt.split_whitespace().map(|n| n.parse().unwrap()).collect(),
                ));
            }
        }
        result.insert(lhs, rhs);
    }
    result
}

fn d19_grammar_add_raw(grammar: &mut D19Grammar, lhs: D19N, rhs: D19Rule) {
    if !grammar.contains_key(&lhs) {
        grammar.insert(lhs, vec![]);
    }
    let r: &mut Vec<D19Rule> = grammar.get_mut(&lhs).unwrap();
    r.push(rhs);
}

fn d19_grammar_add(grammar: &mut D19Grammar, last: &mut D19N, lhs: D19N, rhs: D19Rule) {
    match rhs {
        D19Rule::T(_) => d19_grammar_add_raw(grammar, lhs, rhs),
        D19Rule::N(ns) => {
            let mut l = lhs;
            for j in 2..ns.len() {
                *last = *last + 1;
                d19_grammar_add_raw(grammar, l, D19Rule::N(vec![ns[j - 2], *last]));
                l = *last;
            }
            d19_grammar_add_raw(
                grammar,
                l,
                D19Rule::N(ns.iter().rev().take(2).rev().cloned().collect()),
            );
        }
    }
}

fn d19_normalize(grammar: D19Grammar) -> D19Grammar {
    let mut last: D19N = *grammar.keys().max().unwrap();
    let mut result = D19Grammar::new();
    for (l, rs) in grammar {
        for r in rs {
            d19_grammar_add(&mut result, &mut last, l, r);
        }
    }
    result
}

fn d19_cyk(
    grammar: &D19Grammar,
    text: &str,
    cache: &mut D19Cache,
    nonterminal: D19N,
    range: D19Range,
) -> Result<bool> {
    {
        let range_cache = cache.get_mut(&range).ok_or("d19")?;
        if range_cache.known_yes.contains(&nonterminal) {
            return Ok(true);
        }
        if range_cache.known_no.contains(&nonterminal) {
            return Ok(false);
        }
    }
    let mut answer = false;
    for rule in grammar.get(&nonterminal).unwrap() {
        match rule {
            D19Rule::T(c) => {
                answer |= range.0 + 1 == range.1 && text.chars().nth(range.0).ok_or("d19")? == *c
            }
            D19Rule::N(ns) => {
                assert!(ns.len() == 1 || ns.len() == 2); // no epsilons! otherwise more work needed ...
                if ns.len() == 1 {
                    // TODO: This might not terminate, but it does on my data. I should fix it.
                    answer = answer || d19_cyk(grammar, text, cache, ns[0], range)?;
                } else {
                    for k in range.0 + 1..range.1 {
                        answer |= d19_cyk(grammar, text, cache, ns[0], (range.0, k))?
                            && d19_cyk(grammar, text, cache, ns[1], (k, range.1))?;
                    }
                }
            }
        }
    }
    {
        let range_cache = cache.get_mut(&range).ok_or("d19")?;
        if answer {
            range_cache.known_yes.insert(nonterminal);
        } else {
            range_cache.known_no.insert(nonterminal);
        }
    }
    Ok(answer)
}

fn d19_parse_text(rules: &D19Grammar, text: &str) -> bool {
    let mut cache: D19Cache = D19Cache::new();
    for i in 0..text.len() {
        for j in i + 1..=text.len() {
            cache.insert(
                (i, j),
                D19Result {
                    known_yes: HashSet::new(),
                    known_no: HashSet::new(),
                },
            );
        }
    }
    d19_cyk(rules, text, &mut cache, 0, (0, text.len())).unwrap()
}

fn day19(input: &str) -> Result<()> {
    let input: Vec<&str> = input.split("\n\n").collect();
    let rules = d19_parse_rules(input[0]);
    let rules = d19_normalize(rules);
    let data: Vec<&str> = input[1].lines().collect();
    let answer = data.iter().filter(|d| d19_parse_text(&rules, d)).count();
    println!("answer: {}", answer);
    Ok(())
}

type D20TileId = usize;
type D20Tile = u128;
type D20TileVariants = [u128; 8];

static D20N: usize = 10;

fn d20_parse_tile(tile: &[&str]) -> D20Tile {
    let mut r: D20Tile = 0;
    for i in 0..D20N {
        for j in 0..D20N {
            r *= 2;
            if tile[i][j..j + 1] == *"#" {
                r += 1
            }
        }
    }
    r
}

fn d20_rot(tile: D20Tile) -> D20Tile {
    let mut result = 0;
    for i in 0..D20N {
        for j in 0..D20N {
            if (tile >> (i * D20N + j)) & 1 != 0 {
                result |= 1 << ((D20N - j - 1) * D20N + i);
            }
        }
    }
    result
}

fn d20_flip(tile: D20Tile) -> D20Tile {
    let mut result = 0;
    for i in 0..D20N {
        for j in 0..D20N {
            if (tile >> (i * D20N + j)) & 1 != 0 {
                result |= 1 << (i * D20N + D20N - j - 1);
            }
        }
    }
    result
}

fn d20_get_variants(tile: D20Tile) -> D20TileVariants {
    let mut result: D20TileVariants = [0; 8];
    let mut i = 0;
    let mut last = tile;
    for _ in 0..4 {
        last = d20_rot(last);
        result[i] = last;
        i += 1;
    }
    last = d20_flip(last);
    for _ in 0..4 {
        result[i] = last;
        i += 1;
        last = d20_rot(last);
    }
    result
}

fn d20_bottom(tile: D20Tile) -> usize {
    (tile & ((1 << D20N) - 1)) as usize
}

fn d20_reconstruct(
    tiles: &Vec<Option<D20TileVariants>>,
    by_border: &Vec<HashSet<D20TileId>>,
    available: &mut HashSet<D20TileId>,
    image: &mut Vec<Vec<Option<(D20TileId, usize)>>>,
    row: usize,
    col: usize,
) -> bool {
    let m = image.len();
    if row == m {
        return true;
    }
    if col == m {
        return d20_reconstruct(tiles, by_border, available, image, row + 1, 0);
    }
    let top = if row == 0 {
        None
    } else {
        let above = image[row - 1][col].unwrap();
        Some(d20_bottom(tiles[above.0].unwrap()[above.1]))
    };
    let left = if col == 0 {
        None
    } else {
        let prev = image[row][col - 1].unwrap();
        // right of prev.1 is bottom of variant
        let variant = 7
            - (if prev.1 % 4 == 0 {
                prev.1 + 3
            } else {
                prev.1 - 1
            });
        Some(d20_bottom(tiles[prev.0].unwrap()[variant]))
    };
    // prefilter (not really necessary, but it is 5-6 times faster)
    let possible: Vec<D20TileId> = {
        match (top, left) {
            (None, None) => available.iter().copied().collect(),
            (Some(top), None) => by_border[top].intersection(&available).copied().collect(),
            (None, Some(left)) => by_border[left].intersection(&available).copied().collect(),
            (Some(top), Some(left)) => by_border[top]
                .intersection(&by_border[left])
                .copied()
                .collect::<HashSet<D20TileId>>()
                .intersection(&available)
                .copied()
                .collect(),
        }
    };
    //println!("at ({},{}) consider {:?}", row, col, possible);
    // recurse
    for tile_id in possible {
        available.remove(&tile_id);
        for variant in 0..8 {
            if let Some(top) = top {
                // top of variant is bottom of other
                let other = 7
                    - (if variant % 4 < 2 {
                        variant + 2
                    } else {
                        variant - 2
                    });
                if top != d20_bottom(tiles[tile_id].unwrap()[other]) {
                    continue;
                }
            }
            if let Some(left) = left {
                // left of variant is bottom of other
                let other = if variant % 4 < 3 {
                    variant + 1
                } else {
                    variant - 3
                };
                if left != d20_bottom(tiles[tile_id].unwrap()[other]) {
                    continue;
                }
            }
            image[row][col] = Some((tile_id, variant));
            if d20_reconstruct(tiles, by_border, available, image, row, col + 1) {
                return true;
            }
        }
        available.insert(tile_id);
    }
    return false;
}

fn d20_part2_check(map: &Vec<Vec<char>>) -> usize {
    let n = map.len();
    let monster: Vec<Vec<char>> = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .map(|l| l.chars().collect())
    .collect();
    let mx = monster.len();
    let my = monster[0].len();
    let mut ans = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if map[i][j] == '#' {
                ans[i][j] = true;
            }
        }
    }
    for i in mx..=n {
        for j in my..=n {
            let mut ok = true;
            for x in 0..mx {
                for y in 0..my {
                    ok &= monster[x][y] != '#' || map[i - mx + x][j - my + y] == '#';
                }
            }
            if ok {
                for x in 0..mx {
                    for y in 0..my {
                        if monster[x][y] == '#' {
                            ans[i - mx + x][j - my + y] = false;
                        }
                    }
                }
            }
        }
    }
    let cnt: usize = ans.iter().map(|l| l.iter().filter(|x| **x).count()).sum();
    cnt
}

fn d20_map_rot(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = map.len();
    let mut result = vec![vec!['?'; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[n - j - 1][i] = map[i][j];
        }
    }
    result
}

fn d20_map_flip(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = map.len();
    let mut result = vec![vec!['?'; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][n - j - 1] = map[i][j];
        }
    }
    result
}

fn d20_part2(
    tiles: &Vec<Option<D20TileVariants>>,
    image: &Vec<Vec<Option<(D20TileId, usize)>>>,
) -> usize {
    let n = image.len() * (D20N - 2);
    let mut map: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    for i in 0..image.len() {
        for j in 0..image.len() {
            let id_variant = image[i][j].unwrap();
            let t = tiles[id_variant.0].unwrap()[id_variant.1];
            for x in 1..D20N - 1 {
                for y in 1..D20N - 1 {
                    if (t >> ((D20N - x - 1) * D20N + D20N - y - 1)) & 1 != 0 {
                        map[i * (D20N - 2) + x - 1][j * (D20N - 2) + y - 1] = '#';
                    }
                }
            }
        }
    }
    let mut result = n * n;
    for _ in 0..4 {
        map = d20_map_rot(map);
        result = result.min(d20_part2_check(&map));
    }
    map = d20_map_flip(map);
    for _ in 0..4 {
        map = d20_map_rot(map);
        result = result.min(d20_part2_check(&map));
    }
    result
}

fn day20(input: &str) -> Result<()> {
    let tiles: Vec<(D20TileId, D20Tile)> = input
        .trim()
        .split("\n\n")
        .map(|t| {
            let ls: Vec<&str> = t.lines().collect();
            (
                ls[0]
                    .replace(":", " ")
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
                d20_parse_tile(&ls[1..]),
            )
        })
        .collect();
    let mut all_ids: HashSet<D20TileId> = tiles.iter().map(|(i, _)| i).copied().collect();
    let tiles: Vec<Option<D20TileVariants>> = {
        let mut result = vec![None; 1 << 16];
        for (id, tile) in tiles {
            result[id] = Some(d20_get_variants(tile));
        }
        result
    };
    let by_border = {
        let mut result: Vec<HashSet<D20TileId>> = (0..(1 << 16)).map(|_| HashSet::new()).collect();
        for i in &all_ids {
            for v in tiles[*i].unwrap().iter() {
                result[d20_bottom(*v)].insert(*i);
            }
        }
        result
    };
    let m = {
        let mut m = 0;
        while m * m < all_ids.len() {
            m += 1
        }
        m
    };
    let mut image: Vec<Vec<Option<(D20TileId, usize)>>> = vec![vec![None; m]; m];
    let found = d20_reconstruct(&tiles, &by_border, &mut all_ids, &mut image, 0, 0);
    assert!(found, "no solution?");
    let part1 = image[0][0].unwrap().0
        * image[0][m - 1].unwrap().0
        * image[m - 1][0].unwrap().0
        * image[m - 1][m - 1].unwrap().0;
    let part2 = d20_part2(&tiles, &image);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

fn day21(input: &str) -> Result<()> {
    let input: Vec<String> = input
        .lines()
        .map(|l| l.replace(")", "").replace(",", ""))
        .collect();
    let menu: Vec<Vec<HashSet<&str>>> = input
        .iter()
        .map(|l| {
            l.split("(contains")
                .map(|x| x.split_whitespace().collect())
                .collect()
        })
        .collect();
    let mut by_allergen: HashMap<&str, HashSet<&str>> = HashMap::new();
    for dish in &menu {
        let ingredients = &dish[0];
        let allergens = &dish[1];
        for a in allergens {
            match by_allergen.get_mut(a) {
                Some(i) => *i = i.intersection(ingredients).copied().collect(),
                None => {
                    by_allergen.insert(a, ingredients.clone());
                }
            }
        }
    }
    let bad = by_allergen
        .values()
        .fold(HashSet::new(), |acc, x| acc.union(x).copied().collect());
    let part1: usize = menu
        .iter()
        .map(|d| d[0].iter().filter(|i| !bad.contains(*i)).count())
        .sum();
    let mut changed = true;
    while changed {
        changed = false;
        let allergens: Vec<&str> = by_allergen.keys().copied().collect();
        for a in &allergens {
            let ws = by_allergen.get(a).unwrap();
            if ws.len() == 1 {
                let w = ws.iter().next().unwrap().clone();
                for b in &allergens {
                    if a == b {
                        continue;
                    }
                    changed |= by_allergen.get_mut(b).unwrap().remove(w);
                }
            }
        }
    }
    let part2 = {
        let mut allergens: Vec<&str> = by_allergen.keys().copied().collect();
        allergens.sort();
        let words: Vec<&str> = allergens
            .iter()
            .map(|a| by_allergen.get(a).unwrap().iter().next().unwrap())
            .copied()
            .collect();
        words.join(",")
    };
    println!("part1 {}", part1);
    println!("part2 {}", part2);
    Ok(())
}

fn d22_go(cards: &mut Vec<VecDeque<usize>>, recurse: bool) {
    let mut seen: HashSet<Vec<VecDeque<usize>>> = HashSet::new();
    while !cards[0].is_empty() && !cards[1].is_empty() {
        if seen.contains(cards) {
            cards[1].clear();
            return;
        }
        seen.insert(cards.clone());
        let c: [usize; 2] = [cards[0].pop_front().unwrap(), cards[1].pop_front().unwrap()];
        let winner = if recurse && c[0] <= cards[0].len() && c[1] <= cards[1].len() {
            let mut cards_copy: Vec<VecDeque<usize>> = vec![];
            for i in 0..2 {
                cards_copy.push(cards[i].iter().take(c[i]).copied().collect());
            }
            d22_go(&mut cards_copy, recurse);
            if cards_copy[0].is_empty() {
                1
            } else {
                0
            }
        } else if c[0] < c[1] {
            1
        } else {
            0
        };
        cards[winner].push_back(c[winner]);
        cards[winner].push_back(c[1 - winner]);
    }
}

fn d22_solve(cards: &Vec<Vec<usize>>, recurse: bool) -> usize {
    let mut cards_copy: Vec<VecDeque<usize>> =
        cards.iter().map(|h| h.iter().copied().collect()).collect();
    d22_go(&mut cards_copy, recurse);
    let mut ans: usize = 0;
    for hand in &cards_copy {
        for i in 0..hand.len() {
            ans += (hand.len() - i) * hand[i];
        }
    }
    ans
}

fn day22(input: &str) -> Result<()> {
    let cards: Vec<Vec<usize>> = input
        .trim()
        .split("\n\n")
        .map(|h| h.lines().skip(1).map(|c| c.parse().unwrap()).collect())
        .collect();
    let part1 = d22_solve(&cards, false);
    println!("part1: {}", part1);
    let part2 = d22_solve(&cards, true);
    println!("part2: {}", part2);
    Ok(())
}

fn d23_go(perm: &Vec<usize>, m: usize, n: usize) -> Vec<usize> {
    let mut next: Vec<usize> = vec![n; n];
    let mut prev: Vec<usize> = vec![n; n];
    {
        let mut pi: Vec<usize> = perm.iter().map(|x| x - 1).collect();
        for i in pi.len()..n {
            pi.push(i);
        }
        for i in 0..n {
            next[pi[i]] = pi[(i + 1) % n];
        }
        for i in 0..n {
            prev[next[i]] = i
        }
    }
    let mut c0 = perm[0] - 1;
    for _ in 0..m {
        let c1 = next[c0];
        let c2 = next[c1];
        let c3 = next[c2];
        let mut d = c0;
        while {
            d = (d + n - 1) % n;
            d == c1 || d == c2 || d == c3
        } {}
        next[c0] = next[c3];
        prev[next[c0]] = c0;
        prev[c1] = d;
        next[c3] = next[d];
        next[prev[c1]] = c1;
        prev[next[c3]] = c3;
        c0 = next[c0];
    }
    let mut ans: Vec<usize> = vec![];
    c0 = 0;
    for _ in 1..n {
        c0 = next[c0];
        ans.push(c0 + 1);
    }
    ans
}

fn day23(input: &str) -> Result<()> {
    let permutation: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| String::from(c).parse().unwrap())
        .collect();
    let part1 = d23_go(&permutation, 100, permutation.len());
    let part2 = d23_go(&permutation, 10_000_000, 1_000_000);
    println!(
        "part1: {}",
        part1
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join("")
    );
    println!("part2: {}", (part2[0] as u64) * (part2[1] as u64));
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let input = std::fs::read_to_string(&args.input_file)?;
    match args.day_number {
        1 => day1(&input),
        2 => day2(&input),
        3 => day3(&input),
        4 => day4(&input),
        5 => day5(&input),
        6 => day6(&input),
        7 => day7(&input),
        8 => day8(&input),
        9 => day9(&input),
        10 => day10(&input),
        11 => day11(&input),
        12 => day12(&input),
        13 => day13(&input),
        14 => day14(&input),
        15 => day15(&input),
        16 => day16(&input),
        17 => day17(&input),
        18 => day18(&input),
        19 => day19(&input),
        20 => day20(&input),
        21 => day21(&input),
        22 => day22(&input),
        23 => day23(&input),
        _ => {
            println!("unknown day ({})", args.day_number);
            Ok(())
        }
    }
}
