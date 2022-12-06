use std::collections::HashSet;
use std::io;

fn char_to_val(c: &char) -> u32 {
    match c {
        'a'..='z' => *c as u32 - 'a' as u32 + 1,
        'A'..='Z' => *c as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

fn process_line(line: &String) -> u32 {
    let (first, last) = line.split_at(line.len() / 2);
    let mut a = HashSet::new();
    let mut b = HashSet::new();
    for c in first.chars() {
        a.insert(c);
    }
    for c in last.chars() {
        b.insert(c);
    }
    let mut common = a.intersection(&b);
    char_to_val(common.next().unwrap())
}

fn process_group(group: &[String]) -> u32 {
    let mut a = HashSet::new();
    let mut b = HashSet::new();
    let mut c = HashSet::new();
    for v in group[0].chars() {
        a.insert(v);
    }
    for v in group[1].chars() {
        b.insert(v);
    }
    for v in group[2].chars() {
        c.insert(v);
    }
    let common: HashSet<_> = a.intersection(&b).cloned().collect();
    let mut common2 = c.intersection(&common);
    char_to_val(common2.next().unwrap())
}

fn main() {
    let input: Vec<String> = io::stdin().lines().map(|val| val.unwrap()).collect();
    let res: u32 = input.iter().map(|v| process_line(v)).sum();
    println!("{}", res);
    let res2: u32 = input.chunks(3).map(|v| process_group(v)).sum();
    println!("{}", res2);
}
