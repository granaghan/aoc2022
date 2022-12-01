use std::io::{self, BufRead};

fn main() {
    let mut data: Vec<u32> = vec![0];
    for line in io::stdin().lock().lines() {
        if let Ok(cal) = line.unwrap().parse::<u32>() {
            *data.last_mut().unwrap() += cal;
        } else {
            data.push(0);
        }
    }
    data.sort();
    println!("{}", data.last_mut().unwrap());
    println!("{}", data[data.len() - 3..].iter().sum::<u32>());
}
