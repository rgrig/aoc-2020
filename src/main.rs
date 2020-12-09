use error_chain::error_chain;
use std::collections::{HashMap, HashSet};
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
        _ => {
            println!("unknown day ({})", args.day_number);
            Ok(())
        }
    }
}
