fn main() {
    let input = include_str!("../input.txt");
    let res = calc_pos(input);
    dbg!(res);
}

fn calc_pos(input: &str) -> usize {
    let mut ctr: usize = 0;

    for line in input.lines() {
        let parts: [&str; 2] = line
            .split_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        dbg!(parts);
    }

    ctr
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
