use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let res = calc_pos(input);
    dbg!(res);
}

// if distance of node<->node > (1, 1), copy the action of prev node (action: (i16, i16))
// check for node 1by1 from front to rear ([node_pos: (i16, i16)])
fn calc_pos(input: &str) -> usize {
    let mut tail_pos_set: HashSet<(i16, i16)> = HashSet::new();
    let mut nodes_pos: [(i16, i16); 10] = [(0, 0); 10];
    // 1. if action_prev == None, and distance > 1, then go to former node_pos_prev, and save
    //    transaction as action_prev
    // 2. if action_prev == None, and distance <= 1, then do nth
    // 3. if action_prev != None, and distance > 1, then transfer action_prev
    // 4. if action_prev != None, and distance <= 1, then reset action_prev

    for line in input.lines() {
        let instruct: (&str, &str) = line.split_once(" ").unwrap();
        let steps = instruct.1.parse().unwrap();
        for _ in 0..steps {
            match instruct.0 {
                "R" => nodes_pos[0].0 += 1,
                "L" => nodes_pos[0].0 -= 1,
                "U" => nodes_pos[0].1 += 1,
                "D" => nodes_pos[0].1 -= 1,
                _ => unreachable!(),
            }

            for i in 1..nodes_pos.len() {
                nodes_pos[i] = follow(nodes_pos[i - 1], nodes_pos[i]);
            }

            tail_pos_set.insert(nodes_pos[9]); // Track final knot
        }
    }

    tail_pos_set.len()
}

fn follow(head: (i16, i16), tail: (i16, i16)) -> (i16, i16) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return tail; // Already adjacent
    }

    (
        tail.0 + dx.signum(), // Move one step in x
        tail.1 + dy.signum(), // Move one step in y
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        assert_eq!(calc_pos(input), 36);
    }
}
