fn main() {
    let input = include_str!("../input.txt");
    let res = calc_signal(input);
    dbg!(res);
}

fn calc_signal(input: &str) -> i32 {
    let mut cycle = 0;
    let mut signal = 1;
    let mut target_cycle = [20, 60, 100, 140, 180, 220].iter().peekable();
    let mut signal_strength = Vec::<i32>::new();
    for line in input.lines() {
        let (instr, value) = line.split_once(" ").unwrap_or((line, "0"));
        let cycles_per_instr = if instr == "addx" { 2 } else { 1 };

        for _ in 0..cycles_per_instr {
            cycle += 1;
            if target_cycle.peek().is_some_and(|&&c| cycle == c) {
                signal_strength.push(target_cycle.next().unwrap() * signal);
            }
        }

        if instr == "addx" {
            signal += value.parse::<i32>().unwrap();
        }
    }
    signal_strength.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(calc_signal(input), 13140);
    }
}
