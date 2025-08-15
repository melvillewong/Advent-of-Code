fn main() {
    let input = include_str!("../input.txt");
    let res = draw_crt(input);
    println!("{}", res);
}

/*
* Sprite = A 3-pixels wide painter, change pos by addx after 2 cycles
* CRT cell's pixel can be [lit(#) or dark(.)] depends on whether sprite is overlapping it (any
*   pixel of its body)
*/

/*
* 1. Cycle 1 == CRT cell pixel 1
* 2. Keep track of addx POS, save POS±1 (3 pos)
* 3. Check if curr cycle == 1 of the 3 saved sprite pos
*   - True (overlap) = draw "#"
*   - False = draw "."
* 4. Jump to \n if cycle is multiple of 40 (\n and mod 40)
*/

fn draw_crt(input: &str) -> String {
    let mut crt = String::new();
    const MAX_CYCLE: i32 = 240;
    let mut cycle = 0;
    let mut sprite = 1;

    'outer: for line in input.lines() {
        let (instr, value) = line.split_once(" ").unwrap_or((line, "0"));
        let cycles_per_instr = if instr == "addx" { 2 } else { 1 };

        '_inner: for _ in 0..cycles_per_instr {
            if cycle % 40 == 0 && cycle != 0 {
                cycle %= 40;
                crt.push('\n');
            } else if cycle == MAX_CYCLE {
                break 'outer;
            }
            if is_overlap(cycle, sprite) {
                crt.push('#');
            } else {
                crt.push('.');
            }
            cycle += 1;
        }

        if instr == "addx" {
            sprite += value.parse::<i32>().unwrap();
        }
    }

    crt
}

fn is_overlap(cycle: i32, sprite: i32) -> bool {
    cycle == sprite - 1 || cycle == sprite || cycle == sprite + 1
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
        let ans = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(draw_crt(input), ans);
    }
}
