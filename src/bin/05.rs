use std::collections::VecDeque;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

fn process_line(line: &String) -> u32 {
    0
}

fn process_line2(line: &String) -> u32 {
    0
}

// 1  1
// 5  2
// 9  3
// 13 4

fn loc_to_bin(loc: usize) -> usize {
    loc / 4 + 1
}

fn load_map(data: &Vec<String>) -> Vec<VecDeque<char>> {
    let mut map: Vec<VecDeque<char>> = Vec::with_capacity(10);
    for _ in 0..map.capacity() {
        map.push(VecDeque::new());
    }
    for line in data.iter() {
        for (loc, c) in line.chars().enumerate() {
            match c {
                'A'..='Z' => {
                    let bin = loc_to_bin(loc);
                    map[bin].push_front(c);
                }
                _ => {}
            }
        }
    }
    //println!("{:?}", map);
    map
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace().skip(1).step_by(2);
        Ok(Move {
            count: iter.next().unwrap().parse()?,
            from: iter.next().unwrap().parse()?,
            to: iter.next().unwrap().parse()?,
        })
    }
}

fn parse_moves(data: &Vec<String>) -> Vec<Move> {
    let moves: Vec<Move> = data.iter().map(|line| line.parse().unwrap()).collect();
    //println!("{:?}", moves);
    moves
}

fn process_moves(map: &mut Vec<VecDeque<char>>, moves: &Vec<Move>) {
    let _ = moves
        .iter()
        .inspect(|m| {
            // This moves blocks
            let split = map[m.from].len() - m.count;
            let mut vals = map[m.from].split_off(split);
            map[m.to].append(&mut vals);
            //for _ in 0..m.count {
                //let val = map[m.from].pop_back().unwrap();
                //map[m.to].push_back(val);
            //}
        })
        .count();
}

fn main() {
    let input: Vec<String> = io::stdin().lines().map(|val| val.unwrap()).collect();
    run(input);
}

fn run(mut input: Vec<String>) {
    let mut unparsed_map: Vec<String> = Vec::new();
    let mut count: usize = 0;

    for line in input.iter() {
        count = count + 1;
        if line.trim().len() != 0 {
            unparsed_map.push(line.clone());
        } else {
            break;
        }
    }

    // This is gross and not efficient.
    let mut map = load_map(&unparsed_map);
    let moves = parse_moves(&input.split_off(count));
    process_moves(&mut map, &moves);
    let _ = map
        .iter()
        .skip(1)
        .inspect(|val| print!("{}", val.back().unwrap()))
        .count();
    println!("");
}

#[test]
fn test_loc_to_bin() {
    assert_eq!(loc_to_bin(1), 1);
    assert_eq!(loc_to_bin(5), 2);
    assert_eq!(loc_to_bin(9), 3);
    assert_eq!(loc_to_bin(13), 4);
}

#[test]
fn test() {
    const example: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        " move 1 from 2 to 1\n",
        " move 3 from 1 to 3\n",
        " move 2 from 2 to 1\n",
        " move 1 from 1 to 2\n",
    );
    let input: Vec<String> = example.lines().map(|val| val.into()).collect();
    run(input);
}
