use core::panic;

fn main() {
    let input = include_str!("../input.txt");
    let res = calc_score(input);
    dbg!(res);
}

fn calc_score(input: &str) -> u16 {
    let mut score: u16 = 0;
    for line in input.lines() {
        let cpu = &line[..1];
        let player = &line[line.len() - 1..];

        score += match player {
            "X" => match cpu {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => panic!("Wrong input"),
            },
            "Y" => match cpu {
                "A" => 1 + 3,
                "B" => 2 + 3,
                "C" => 3 + 3,
                _ => panic!("Wrong input"),
            },
            "Z" => match cpu {
                "A" => 2 + 6,
                "B" => 3 + 6,
                "C" => 1 + 6,
                _ => panic!("Wrong input"),
            },
            _ => panic!("Wrong input"),
        }
    }
    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_score() {
        let input = "A Y
B X
C Z";
        assert_eq!(calc_score(input), 12);
    }
}
