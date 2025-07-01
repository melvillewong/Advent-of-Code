fn main() {
    let input = include_str!("../input.txt");
    let res = find_max_trees(input);
    dbg!(res);
}

fn find_max_trees(input: &str) -> usize {
    let mut metric: Vec<Vec<u8>> = vec![];
    populate(input, &mut metric);
    let mut max: usize = 0;
    for i in 1..metric.len() - 1 {
        for j in 1..metric[i].len() - 1 {
            let curr = calc_tree_max((i, j), &metric);
            max = if max > curr { max } else { curr };
        }
    }
    max
}

fn populate(input: &str, vec: &mut Vec<Vec<u8>>) {
    for line in input.lines() {
        vec.push(line.bytes().map(|b| b - b'0').collect());
    }
}

fn calc_tree_max(pos: (usize, usize), vec: &[Vec<u8>]) -> usize {
    let tree = vec[pos.0][pos.1];

    // Top
    let mut top = 0;
    for i in (0..pos.0).rev() {
        top += 1;
        if vec[i][pos.1] >= tree {
            break;
        }
    }

    // Bottom
    let mut bottom = 0;
    for i in pos.0 + 1..vec.len() {
        bottom += 1;
        if vec[i][pos.1] >= tree {
            break;
        }
    }

    // Left
    let mut left = 0;
    for i in (0..pos.1).rev() {
        left += 1;
        if vec[pos.0][i] >= tree {
            break;
        }
    }

    // Right
    let mut right = 0;
    for i in pos.1 + 1..vec[0].len() {
        right += 1;
        if vec[pos.0][i] >= tree {
            break;
        }
    }

    top * bottom * left * right
}

// NOTE - Overcomplicated (should only consider next tree < height -> continue, next tree >= height
// -> count and break)
// fn calc_tree_max(pos: (usize, usize), vec: &[Vec<u8>]) -> usize {
//     let tree = vec[pos.0][pos.1];
//     let mut curr_max_height = 0;
//     // can see between tree with curr_max_height and consideration tree
//     // update tree with curr_max_height if tree after has higher max_height
//     let mut top = 0;
//     for t in vec[..pos.0].iter().rev() {
//         // t >= tree
//         if t[pos.1] >= tree {
//             top += 1;
//             break;
//         }
//         // t > curr
//         else if t[pos.1] > curr_max_height {
//             curr_max_height = t[pos.1];
//             top += 1;
//         }
//         // t == curr
//         else if t[pos.1] == curr_max_height {
//             top += 1;
//         }
//     }
//     curr_max_height = 0;
//
//     let mut bottom = 0;
//     for t in vec[pos.0 + 1..].iter() {
//         if t[pos.1] >= tree {
//             bottom += 1;
//             break;
//         }
//         // t > curr
//         else if t[pos.1] > curr_max_height {
//             curr_max_height = t[pos.1];
//             bottom += 1;
//         }
//         // t == curr
//         else if t[pos.1] == curr_max_height {
//             bottom += 1;
//         }
//     }
//     curr_max_height = 0;
//
//     let mut left = 0;
//     for &t in vec[pos.0][..pos.1].iter().rev() {
//         if t >= tree {
//             left += 1;
//             break;
//         }
//         // t > curr
//         else if t > curr_max_height {
//             curr_max_height = t;
//             left += 1;
//         }
//         // t == curr
//         else if t == curr_max_height {
//             left += 1;
//         }
//     }
//     curr_max_height = 0;
//
//     let mut right = 0;
//     for &t in vec[pos.0][pos.1 + 1..].iter() {
//         if t >= tree {
//             right += 1;
//             break;
//         }
//         // t > curr
//         else if t > curr_max_height {
//             curr_max_height = t;
//             right += 1;
//         }
//         // t == curr
//         else if t == curr_max_height {
//             right += 1;
//         }
//     }
//     top * bottom * left * right
// }

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
        assert_eq!(find_max_trees(input), 8);
    }
}
