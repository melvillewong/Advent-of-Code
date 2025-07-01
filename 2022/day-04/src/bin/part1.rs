fn main() {
    let input = include_str!("../input.txt");
    let res = find_fully_overlap(input);
    dbg!(res);
}

fn find_fully_overlap(input: &str) -> u16 {
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
    let overlap_fst = parsed_ranges[2] <= parsed_ranges[0] && parsed_ranges[1] <= parsed_ranges[3];
    let overlap_scd = parsed_ranges[0] <= parsed_ranges[2] && parsed_ranges[3] <= parsed_ranges[1];
    overlap_fst || overlap_scd
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
        assert_eq!(find_fully_overlap(input), 2);
    }
}
