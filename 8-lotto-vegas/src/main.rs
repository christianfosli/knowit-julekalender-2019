use std::collections::HashMap;
use std::fs;

fn main() {
    let wheels = fs::read_to_string("wheels.txt").expect("error reading file");
    let ops = parse_ops(wheels);
    let mut max = 0;
    for mut coins in 1..11 {
        let mut counts = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut wheel = last_digit(coins);
        loop {
            let op = &ops.get(&wheel).unwrap()[counts[wheel as usize] % 4];
            if op == "STOPP" {
                if coins > max {
                    max = coins;
                }
                break;
            } else if op == "ROTERPAR" {
                for i in 0..counts.len() {
                    if i % 2 == 0 {
                        counts[i] += 1;
                    }
                }
            } else if op == "ROTERODDE" {
                for i in 0..counts.len() {
                    if i % 2 != 0 {
                        counts[i] += 1;
                    }
                }
            } else if op == "ROTERALLE" {
                for i in 0..counts.len() {
                    counts[i] += 1;
                }
            } else {
                coins = perform_op(coins, op);
            }
            counts[wheel as usize] += 1;
            wheel = last_digit(coins);
        }
    }
    println!("{}", max);
}

fn perform_op(coins: i32, op: &str) -> i32 {
    match op {
        "PLUSS4" => coins + 4,
        "PLUSS101" => coins + 101,
        "MINUS9" => coins - 9,
        "MINUS1" => coins - 1,
        "REVERSERSIFFER" => {
            let negative = coins < 0;
            let coins: &str = &format!("{}", coins);
            let rev: String = coins.chars().rev().collect();
            if negative {
                let n: i32 = rev[..rev.len() - 1]
                    .parse()
                    .expect("error converting back to num");
                return -n;
            }
            rev.parse().expect("error converting back to number")
        }
        "OPP7" => {
            let mut c = coins;
            loop {
                c += 1;
                let s: &str = &format!("{}", c);
                if s.chars().last().expect("error getting last char") == '7' {
                    break;
                }
            }
            c
        }
        "GANGEMSD" => {
            let msd = {
                format!("{}", coins)
                    .chars()
                    .find(|n| n.is_digit(10))
                    .and_then(|a| a.to_digit(10))
                    .expect("No num found")
            };
            coins * msd as i32
        }
        "DELEMSD" => {
            let msd = {
                format!("{}", coins)
                    .chars()
                    .find(|n| n.is_digit(10))
                    .and_then(|a| a.to_digit(10))
                    .expect("No num found")
            };
            // TODO: check if this is eucledian division
            coins / msd as i32
        }
        "PLUSS1TILPAR" => {
            let s_orig: &str = &format!("{}", coins);
            let s_new: String = s_orig
                .chars()
                .map(|c| {
                    if !c.is_digit(10) {
                        return c.to_string();
                    }
                    let mut d = c.to_digit(10).expect("unable to convert back to digit");
                    if d % 2 == 0 {
                        d += 1;
                    }
                    format!("{}", d)
                })
                .collect();
            s_new.parse().expect("Unable to convert back to num")
        }
        "TREKK1FRAODDE" => {
            let s_orig: &str = &format!("{}", coins);
            let s_new: String = s_orig
                .chars()
                .map(|c| {
                    if !c.is_digit(10) {
                        return c.to_string();
                    }
                    let mut d = c.to_digit(10).expect("unable to convert back to digit");
                    if d % 2 != 0 {
                        d -= 1;
                    }
                    format!("{}", d)
                })
                .collect();
            s_new.parse().expect("Unable to convert back to num")
        }
        _ => panic!("Unexpected operation"),
    }
}

fn last_digit(n: i32) -> u32 {
    let s: &str = &format!("{}", n);
    s.chars()
        .last()
        .expect("cant get last char")
        .to_digit(10)
        .expect("cant convert to number")
}

fn parse_ops(wheels: String) -> HashMap<u32, Vec<String>> {
    let mut ops_all_wheels: HashMap<u32, Vec<String>> = HashMap::new();
    for l in wheels.lines() {
        let (wheel, ops) = l.split_at(7);
        let wheel: u32 = wheel
            .chars()
            .find(|a| a.is_digit(10))
            .and_then(|a| a.to_digit(10))
            .expect("No num found");
        let ops: Vec<String> = ops
            .trim()
            .to_owned()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        ops_all_wheels.insert(wheel, ops);
    }
    ops_all_wheels
}
