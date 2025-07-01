use std::{collections::HashMap, iter::Peekable, str::Lines};

type LineIter<'a> = Peekable<Lines<'a>>;

fn main() {
    let input = include_str!("../input.txt");
    let res = calc_total_freed_size(input);
    dbg!(res);
}

fn calc_total_freed_size(input: &str) -> u32 {
    let mut lines: LineIter = input.lines().peekable();
    let dir_size_hm = find_all_dirs_size(&mut lines);
    let needed_space = 30000000 - (70000000 - dir_size_hm["/"]);
    // (diff, size)
    let mut res: (u32, u32) = (dir_size_hm["/"] - needed_space, dir_size_hm["/"]);
    for size in dir_size_hm.values() {
        if *size > needed_space && *size - needed_space < res.0 {
            res = (size - needed_space, *size);
        }
    }
    res.1
}

fn find_all_dirs_size(lines: &mut LineIter) -> HashMap<String, u32> {
    let mut hm: HashMap<String, u32> = HashMap::new();
    parse_com_line(lines, &mut hm);
    hm
}

fn parse_com_line(lines: &mut LineIter, hm: &mut HashMap<String, u32>) {
    let mut layer: Vec<String> = vec![];
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            let mut parts = line[2..].split_whitespace();
            let (args1, args2) = (parts.next(), parts.next());
            command_exec(lines, hm, &mut layer, args1, args2);
        }
    }
}

fn command_exec(
    lines: &mut LineIter,
    hm: &mut HashMap<String, u32>,
    layer: &mut Vec<String>,
    args1: Option<&str>,
    args2: Option<&str>,
) {
    match args1.expect("At least one args for command_exec") {
        "cd" => com_cd(layer, args2.expect("dir should come with cd")),
        "ls" => com_ls(hm, layer, lines),
        _ => panic!("Unexpected command"),
    }
}

fn com_cd(layer: &mut Vec<String>, dir: &str) {
    match dir {
        ".." => {
            layer.pop();
        }
        "/" => layer.clear(),
        d => layer.push(d.to_string()),
    }
}

fn com_ls(hm: &mut HashMap<String, u32>, layer: &mut [String], lines: &mut LineIter) {
    let mut visible_size: u32 = 0;
    while lines.peek().is_some() && !lines.peek().unwrap().starts_with('$') {
        if let Some(line) = lines.next() {
            let mut parts = line.split_whitespace().peekable();
            if parts
                .peek()
                .expect("Expect space between listed item")
                .parse::<u32>()
                .is_ok()
            {
                if let Some(size) = parts.next() {
                    visible_size += size.parse::<u32>().unwrap();
                }
            }
        }
    }
    for i in 0..=layer.len() {
        let path = format!("/{}", layer[..i].join("/"));
        hm.entry(path)
            .and_modify(|size| *size += visible_size)
            .or_insert(visible_size);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(calc_total_freed_size(input), 24933642);
    }
}
