use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let res = calc_pos(input);
    dbg!(res);
}

fn calc_pos(input: &str) -> usize {
    let mut tail_pos_set: HashSet<(i16, i16)> = HashSet::new();
    let mut tail_pos_curr: (i16, i16) = (0, 0);
    let mut head_pos_curr: (i16, i16) = (0, 0);
    let mut head_pos_prev: (i16, i16) = (0, 0);

    for line in input.lines() {
        let action: (&str, &str) = line
            .split_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .map(|arr: [&str; 2]| (arr[0], arr[1]))
            .unwrap();
        for _ in 0..action.1.parse().unwrap() {
            match action.0 {
                "R" => head_pos_curr.0 += 1,
                "L" => head_pos_curr.0 -= 1,
                "U" => head_pos_curr.1 += 1,
                "D" => head_pos_curr.1 -= 1,
                _ => (),
            }
            if (tail_pos_curr.0 - head_pos_curr.0).abs() >= 2
                || (tail_pos_curr.1 - head_pos_curr.1).abs() >= 2
            {
                tail_pos_curr = head_pos_prev;
            }
            if !tail_pos_set.contains(&tail_pos_curr) {
                tail_pos_set.insert(tail_pos_curr);
            }
            head_pos_prev = head_pos_curr;
            dbg!(tail_pos_curr);
        }
    }

    tail_pos_set.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(calc_pos(input), 13);
    }
}
