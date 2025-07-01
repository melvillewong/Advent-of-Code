fn main() {
    let input = include_str!("../input.txt");
    let res = process(input);
    dbg!(res);
}

fn process(input: &str) {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(process(input), 21);
    }
}
