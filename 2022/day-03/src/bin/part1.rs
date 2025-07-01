fn main() {
    let input = include_str!("../input.txt");
    let res = calc_prior_sum(input);
    dbg!(res);
}

fn calc_prior_sum(input: &str) -> u16 {
    let mut sum: u16 = 0;

    for line in input.lines() {
        let comp_len = line.len() / 2;
        let fst_comp: Vec<char> = line[..comp_len].chars().collect();
        for c in line[comp_len..].chars() {
            if fst_comp.contains(&c) {
                sum += char_to_int(c);
                break;
            }
        }
    }

    sum
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
        assert_eq!(calc_prior_sum(input), 157);
    }
}
