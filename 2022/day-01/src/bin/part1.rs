fn main() {
    let input = include_str!("../input1.txt");
    let res = get_max(input);
    println!("{res}");
}

fn get_max(input: &str) -> u32 {
    let mut temp: u32 = 0;
    let mut res: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if temp > res {
                res = temp;
            }
            temp = 0;
            continue;
        }
        temp += line.parse::<u32>().expect("not number");
        dbg!(line, temp);
    }
    if temp > res { temp } else { res }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(get_max(input), 24000);
    }
}
