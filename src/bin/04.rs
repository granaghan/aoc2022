use std::io;
use std::ops::RangeInclusive;

// Range::contains can't take a Range :(
fn is_contained(r1: RangeInclusive<u32>, r2: RangeInclusive<u32>) -> bool {
    (r1.start() <= r2.start() && r1.end() >= r2.end())
        || (r2.start() <= r1.start() && r2.end() >= r1.end())
}

// Range::contains can't take a Range :(
fn is_contained2(r1: RangeInclusive<u32>, r2: RangeInclusive<u32>) -> bool {
    for i in r2 {
        if r1.contains(&i) {
            return true;
        }
    }
    false
}

fn process_line(line: &String) -> u32 {
    let (first, second) = line.split_once(',').unwrap();
    let (s1, e1) = first.split_once('-').unwrap();
    let (s2, e2) = second.split_once('-').unwrap();
    let r1 = RangeInclusive::new(s1.parse::<u32>().unwrap(), e1.parse::<u32>().unwrap());
    let r2 = RangeInclusive::new(s2.parse::<u32>().unwrap(), e2.parse::<u32>().unwrap());
    if is_contained(r1, r2) {
        1
    } else {
        0
    }
}

fn process_line2(line: &String) -> u32 {
    let (first, second) = line.split_once(',').unwrap();
    let (s1, e1) = first.split_once('-').unwrap();
    let (s2, e2) = second.split_once('-').unwrap();
    let r1 = RangeInclusive::new(s1.parse::<u32>().unwrap(), e1.parse::<u32>().unwrap());
    let r2 = RangeInclusive::new(s2.parse::<u32>().unwrap(), e2.parse::<u32>().unwrap());
    if is_contained2(r1, r2) {
        1
    } else {
        0
    }
}

fn main() {
    let input: Vec<String> = io::stdin().lines().map(|val| val.unwrap()).collect();
    let sol1: u32 = input.iter().map(|v| process_line(v)).sum();
    println!("{}", sol1);
    let sol2: u32 = input.iter().map(|v| process_line2(v)).sum();
    println!("{}", sol2);
}
