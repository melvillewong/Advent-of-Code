fn main() {
    let input = include_str!("../input.txt");
    let res = find_overlap(input);
    dbg!(res);
}

fn find_overlap(input: &str) -> u16 {
    let mut pair: u16 = 0;

    for line in input.lines() {
        let ranges: Vec<&str> = line.split_terminator(&['-', ','][..]).collect();
        if parse_and_compare(ranges) {
            pair += 1
        }
    }

    pair
}

fn parse_and_compare(ranges: Vec<&str>) -> bool {
    let parsed_ranges: Vec<u16> = ranges.iter().map(|s| s.parse().unwrap()).collect();
    let head_to_tail = parsed_ranges[0] <= parsed_ranges[3];
    let tail_to_head = parsed_ranges[1] >= parsed_ranges[2];
    head_to_tail && tail_to_head
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(find_overlap(input), 4);
    }
}
