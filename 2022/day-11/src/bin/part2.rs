use eval::{eval, to_value};

fn main() {
    let input = include_str!("../input.txt");
    let res = calc_top2_monkeys(input);
    dbg!(res);
}

#[derive(Debug, PartialEq, Clone)]
struct Actions {
    r#true: Option<u8>,
    r#false: Option<u8>,
}

#[derive(Debug, PartialEq, Clone)]
struct Instr {
    op: String,
    test: u8,
    actions: Actions,
}

fn calc_top2_monkeys(input: &str) -> u64 {
    // Var: a & b & c
    let mut inspect_times: Vec<u32>;
    let mut items: Vec<Vec<u32>>;
    let mut instrs: Vec<Option<Instr>>;

    // Const rounds
    const ROUNDS: u16 = 10000;

    // Store each monkey and its instr in Vec
    let split_monkey = input.split("\r\n\r\n").collect::<Vec<_>>();
    let monkey_amount = split_monkey.len();

    // Init 1: Initial items
    let item_instrs = input
        .lines()
        .map(|l| l.trim_start())
        .filter(|l| l.starts_with("Starting items: "))
        .map(|l| &l["Starting items: ".len()..])
        .collect::<Vec<&str>>();
    items = vec![vec![]; monkey_amount];
    init_items(item_instrs, &mut items);
    inspect_times = vec![0; monkey_amount];

    // Init 2: Store monkeys' instrs
    instrs = vec![None; monkey_amount];
    for (i, &monkey) in split_monkey.iter().enumerate() {
        let instr = Instr::parse_instr(monkey);
        instrs[i] = Some(instr);
    }

    // Process 1
    for i in 1..=ROUNDS {
        update_items_pos(&instrs, &mut items, &mut inspect_times);
        if [
            1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000,
        ]
        .contains(&i)
        {
            println!("{} = {:?}", i, &inspect_times);
        }
    }

    multiply_top2(&mut inspect_times)
}

fn init_items(instr: Vec<&str>, items: &mut [Vec<u32>]) {
    for (i, &ins) in instr.iter().enumerate() {
        let vec = ins
            .split(", ")
            .map(|x| x.parse().expect("Init items parsing failed"))
            .collect::<Vec<u32>>();
        items[i] = vec;
    }
}

impl Instr {
    fn parse_instr(monkey_instr: &str) -> Instr {
        let mut op = String::new();
        let mut test = 0;
        let mut actions = Actions {
            r#true: None,
            r#false: None,
        };

        let mut lines = monkey_instr
            .lines()
            .map(|l| l.trim_start())
            .filter(|l| !(l.starts_with("Monkey ") || l.starts_with("Starting items: ")));

        // find op
        if let Some(target) = lines.next() {
            let prefix = "Operation: new = old ";
            if !target.starts_with(prefix) {
                panic!("unexpected op prefix: {target}");
            }
            let mut target_vec = target[prefix.len()..]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            if target_vec[1] == "old" {
                target_vec[0] = "**".to_string();
                target_vec[1] = 2.to_string();
            }
            op = target_vec.concat();
        }

        // find test
        if let Some(target) = lines.next() {
            let prefix = "Test: divisible by ";
            if !target.starts_with(prefix) {
                panic!("unexpected test prefix");
            }
            test = target[prefix.len()..].parse().expect("Fail to parse test");
        }

        // find actions.true
        if let Some(target) = lines.next() {
            if target.starts_with("If true: throw to monkey ") {
                actions.r#true = Some(
                    target
                        .chars()
                        .last()
                        .expect("Fail to find last ch in test")
                        .to_digit(10)
                        .expect("Fail to parse last ch in test") as u8,
                );
            }
        }

        // find actions.false
        if let Some(target) = lines.next() {
            if target.starts_with("If false: throw to monkey ") {
                actions.r#false = Some(
                    target
                        .chars()
                        .last()
                        .expect("Fail to find last ch in test")
                        .to_digit(10)
                        .expect("Fail to parse last ch in test") as u8,
                )
            }
        }

        Instr { op, test, actions }
    }
}

fn update_items_pos(instrs: &[Option<Instr>], items: &mut [Vec<u32>], inspect_times: &mut [u32]) {
    let modulus: u64 = instrs
        .iter()
        .filter_map(|instr| instr.as_ref().map(|i| i.test as u64))
        .product();
    for monkey_idx in 0..items.len() {
        let mut current_items = std::mem::take(&mut items[monkey_idx]);

        for item in current_items.drain(..) {
            if let Some(instr) = &instrs[monkey_idx] {
                let eval_val = eval(&(item.to_string() + &instr.op[..])[..])
                    .unwrap_or_else(|_| to_value((item as u64).pow(2)))
                    .as_u64()
                    .expect("Failed to convert to u64");
                let calc = eval_val % modulus;
                match calc % instr.test as u64 {
                    0 => {
                        if let Some(i) = instr.actions.r#true {
                            items[i as usize].push(calc as u32);
                        }
                    }
                    _ => {
                        if let Some(i) = instr.actions.r#false {
                            items[i as usize].push(calc as u32);
                        }
                    }
                }
                inspect_times[monkey_idx] += 1;
            }
        }
        // dbg!(&items, &inspect_times);
    }
}

fn multiply_top2(vec: &mut [u32]) -> u64 {
    vec.sort();
    let max_two = &vec[vec.len() - 2..];
    let multiplied: u64 = max_two
        .iter()
        .map(|x| *x as u64)
        .reduce(|acc, e| acc * e)
        .expect("Error multiply_top2");
    multiplied
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3\r
\r
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0\r
\r
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3\r
\r
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(calc_top2_monkeys(input), 2713310158);
    }
}
