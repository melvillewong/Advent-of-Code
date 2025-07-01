use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let res = calc_prior_sum(input);
    dbg!(res);
}

fn calc_prior_sum(input: &str) -> u16 {
    let mut sum: u16 = 0;

    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let shared_char = find_shared_char(
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        );
        sum += char_to_int(shared_char.unwrap());
    }

    sum
}

fn find_shared_char(a: &str, b: &str, c: &str) -> Option<char> {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();
    let set_c: HashSet<char> = c.chars().collect();

    set_a
        .intersection(&set_b)
        .copied()
        .find(|c| set_c.contains(c))
}

fn char_to_int(c: char) -> u16 {
    if !c.is_alphabetic() {
        panic!("Not Alphabetic char");
    }
    if c.is_ascii_lowercase() {
        return c as u16 - 'a' as u16 + 1;
    }
    c as u16 - 'A' as u16 + 27
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_prior_sum() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(calc_prior_sum(input), 70);
    }
}
