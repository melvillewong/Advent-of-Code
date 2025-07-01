use std::{cmp::Ordering, collections::VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let res = find_top_crates(input);
    dbg!(res);
}

fn find_top_crates(input: &str) -> String {
    let mut res = String::new();
    let mut vec: [VecDeque<char>; 9] = [const { VecDeque::new() }; 9];
    let mut is_table = true;
    for line in input.lines() {
        if is_table {
            if check_table(line) {
                parse_into_vec(&mut vec, line);
            } else {
                // dbg!(&vec);
                is_table = false;
            }
        } else if !line.is_empty() {
            parse_and_move_crate(&mut vec, line);
        }
    }
    for vd in &mut vec {
        if !vd.is_empty() {
            res.push(vd.pop_front().unwrap());
        }
    }
    res
}

fn parse_and_move_crate(vec: &mut [VecDeque<char>], line: &str) {
    let arr: [u8; 3] = line
        .split_whitespace()
        .filter_map(|s| s.parse::<u8>().ok())
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    for _ in 0..arr[0] {
        // dbg!(&vec, arr[0], arr[1] as usize - 1, arr[2] as usize - 1);
        let pop = vec[arr[1] as usize - 1].pop_front().unwrap();
        vec[arr[2] as usize - 1].push_front(pop);
    }
}

fn parse_into_vec(vec: &mut [VecDeque<char>], line: &str) {
    let mut col: usize = 0;
    let mut recent_fst_ws: Option<usize> = None;
    for (i, c) in line.chars().enumerate() {
        if c.is_whitespace() && recent_fst_ws.is_none() {
            recent_fst_ws = Some(i);
        }
        if !c.is_whitespace() {
            if recent_fst_ws.is_some() {
                col = match (i - recent_fst_ws.unwrap()).cmp(&1) {
                    Ordering::Greater => {
                        col + (i - recent_fst_ws.unwrap()) / 4 + (i - recent_fst_ws.unwrap()) % 4
                    }
                    Ordering::Equal => col + 1,
                    Ordering::Less => col,
                };
                recent_fst_ws = None;
            } else if c.is_alphabetic() {
                vec[col].push_back(c);
            }
        }
    }
}

fn check_table(line: &str) -> bool {
    !line.starts_with(" 1")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(find_top_crates(input), "CMZ");
    }
}
