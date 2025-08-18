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

/*
* Var:
*   a. Vec<u16> - Store inspect times for each (for final calc)
*   b. [Vec<u16>] - Store current items' worry level for each (for process)
*   c. Struct - Store each target & Test with action
*       - target: String (e.g. "** 2")
*       - Test: i32
*       - Actions: Enum - True(u8), False(u8)
*
* Init:
*   1. Initial items before 1st round
*   2. Read a. for dequeuing each item for processing
*       - If Struct is None -> Read and write struct
*       - If Struct is Some -> Skip reading until '\n' to next monkey
*   3. Initial inspect_times
*
* Process:
*   1. Loop input 20 times
*   2. Loop for each Monkey until EOF
*/

fn calc_top2_monkeys(input: &str) -> u32 {
    // Var: a & b & c
    let mut inspect_times: Vec<u16>;
    let mut items: Vec<Vec<u16>>;
    let mut instrs: Vec<Option<Instr>>;

    // Const rounds
    const ROUNDS: u8 = 20;

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
    for _ in 0..ROUNDS {
        update_items_pos(&instrs, &mut items, &mut inspect_times);
    }

    multiply_top2(&mut inspect_times)
}

fn init_items(instr: Vec<&str>, items: &mut [Vec<u16>]) {
    for (i, &ins) in instr.iter().enumerate() {
        let vec = ins
            .split(", ")
            .map(|x| x.parse().expect("Init items parsing failed"))
            .collect::<Vec<u16>>();
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

fn update_items_pos(instrs: &[Option<Instr>], items: &mut [Vec<u16>], inspect_times: &mut [u16]) {
    for monkey_idx in 0..items.len() {
        let mut current_items = std::mem::take(&mut items[monkey_idx]);

        for item in current_items.drain(..) {
            if let Some(instr) = &instrs[monkey_idx] {
                let eval_val = eval(&(item.to_string() + &instr.op[..])[..])
                    .unwrap_or(to_value((item as u32).pow(2)))
                    .as_u64()
                    .expect("Failed to convert to i64");
                let calc = (eval_val / 3) as u16;
                match calc % instr.test as u16 {
                    0 => {
                        if let Some(i) = instr.actions.r#true {
                            items[i as usize].push(calc);
                        }
                    }
                    _ => {
                        if let Some(i) = instr.actions.r#false {
                            items[i as usize].push(calc);
                        }
                    }
                }
                inspect_times[monkey_idx] += 1;
            }
        }
    }
}

fn multiply_top2(vec: &mut [u16]) -> u32 {
    vec.sort();
    let max_two = &vec[vec.len() - 2..];
    let multiplied: u32 = max_two
        .iter()
        .map(|&x| x as u32)
        .reduce(|acc, e| acc * e)
        .expect("Error multiply_top2");
    multiplied
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_items_test() {
        let vec = vec!["79, 98", "54, 65, 75, 74", "79, 60, 97", "74"];
        let mut items: Vec<Vec<u16>> = Vec::new();
        let items_ans: Vec<Vec<u16>> = vec![
            vec![79, 98],
            vec![54, 65, 75, 74],
            vec![79, 60, 97],
            vec![74],
        ];
        init_items(vec, &mut items);
        assert_eq!(items, items_ans);
    }

    #[test]
    fn parse_instr_test() {
        let str = "Monkey 0:
  Starting items: 79, 98
  target: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";
        let instr = Instr {
            op: "*19".into(),
            test: 23,
            actions: Actions {
                r#true: Some(2),
                r#false: Some(3),
            },
        };
        assert_eq!(Instr::parse_instr(str), instr);
    }

    #[test]
    fn update_items_pos_test() {
        let instrs = vec![
            Some(Instr {
                op: "*19".into(),
                test: 23,
                actions: Actions {
                    r#true: Some(2),
                    r#false: Some(3),
                },
            }),
            Some(Instr {
                op: "+6".into(),
                test: 19,
                actions: Actions {
                    r#true: Some(2),
                    r#false: Some(0),
                },
            }),
            Some(Instr {
                op: "**2".into(),
                test: 13,
                actions: Actions {
                    r#true: Some(1),
                    r#false: Some(3),
                },
            }),
            Some(Instr {
                op: "+3".into(),
                test: 17,
                actions: Actions {
                    r#true: Some(0),
                    r#false: Some(1),
                },
            }),
        ];

        let mut items: Vec<Vec<u16>> = vec![
            vec![79, 98],
            vec![54, 65, 75, 74],
            vec![79, 60, 97],
            vec![74],
        ];
        let mut inspect_times = vec![0; 4];
        let items_ans: Vec<Vec<u16>> = vec![
            vec![20, 23, 27, 26],
            vec![2080, 25, 167, 207, 401, 1046],
            vec![],
            vec![],
        ];
        let inspect_times_ans = vec![2, 4, 3, 5];

        update_items_pos(&instrs, &mut items, &mut inspect_times);

        assert_eq!(items, items_ans);
        assert_eq!(inspect_times, inspect_times_ans);
    }

    #[test]
    fn multiply_top2_test() {
        let mut input = vec![5, 2, 9, 7, 1];

        assert_eq!(multiply_top2(&mut input), 63);
    }

    #[test]
    fn case() {
        let input = "Monkey 0:
  Starting items: 79, 98
  target: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  target: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  target: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  target: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(calc_top2_monkeys(input), 10605);
    }
}
