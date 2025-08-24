use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    let opt = find_fewest(input);
    if let Some(res) = opt {
        println!("Shortest path length: {res}");
    } else {
        println!("No path found");
    }
}

struct Init((usize, usize), (usize, usize), Vec<Vec<char>>);

fn find_fewest(input: &str) -> Option<usize> {
    let Init(start, end, mut grid) = find_start_end_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();

    // normalise 'S' and 'E'
    grid[start.0][start.1] = 'a';
    grid[end.0][end.1] = 'z';

    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();

    // Init elem to expand
    queue.push_back((start.0, start.1, 0));
    visited[start.0][start.1] = true;

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((r, c, dist)) = queue.pop_front() {
        let curr = grid[r][c];

        // base
        if (r, c) == end {
            return Some(dist);
        }

        // for each directions
        for (dr, dc) in directions.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            // if out of bounds
            if (nr < 0 || nr as usize >= rows) || (nc < 0 || nc as usize >= cols) {
                continue;
            }

            let (ur, uc) = (nr as usize, nc as usize);

            // if not one higher elevation
            if (ur, uc) != end && (grid[ur][uc] as u8) > (curr as u8 + 1) {
                continue;
            }

            if !visited[ur][uc] {
                visited[ur][uc] = true;
                queue.push_back((ur, uc, dist + 1));
            }
        }
    }

    None
}

fn find_start_end_grid(input: &str) -> Init {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, vec)| {
            vec.iter()
                .enumerate()
                .find_map(|(c, &ch)| if ch == 'S' { Some((r, c)) } else { None })
        })
        .expect("No start found");

    let end = grid
        .iter()
        .enumerate()
        .find_map(|(r, vec)| {
            vec.iter()
                .enumerate()
                .find_map(|(c, &ch)| if ch == 'E' { Some((r, c)) } else { None })
        })
        .expect("No end found");

    Init(start, end, grid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(find_fewest(input), Some(31));
    }
}
