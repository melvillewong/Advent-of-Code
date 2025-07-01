use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let res = find_pos_for_4diff(input);
    dbg!(res);
}

fn find_pos_for_4diff(input: &str) -> usize {
    let mut res = 0;
    for (i, win) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
        let mut uniq = HashSet::new();
        if win.iter().all(|x| uniq.insert(x)) {
            res = i + 4;
            return res;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_pos_for_4diff(input), 7)
    }

    #[test]
    fn test_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_pos_for_4diff(input), 5)
    }

    #[test]
    fn test_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_pos_for_4diff(input), 6)
    }

    #[test]
    fn test_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_pos_for_4diff(input), 10)
    }

    #[test]
    fn test_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_pos_for_4diff(input), 11)
    }
}
