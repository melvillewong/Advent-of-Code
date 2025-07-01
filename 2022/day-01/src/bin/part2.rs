fn main() {
    let input = include_str!("../input1.txt");
    let res = get_three_max(input);
    println!("{:?}", res.iter().sum::<u32>());
}

fn get_three_max(input: &str) -> [u32; 3] {
    let mut temp: u32 = 0;
    let mut res: [u32; 3] = [0; 3];

    for line in input.lines() {
        if line.is_empty() {
            for (i, v) in res.iter().enumerate() {
                if temp > *v {
                    res[i] = temp;
                    break;
                }
            }
            temp = 0;
            continue;
        }
        temp += line.parse::<u32>().expect("not number");
        dbg!(line, temp);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_three_max() {
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
        assert_eq!(get_three_max(input).iter().sum::<u32>(), 45000);
    }
}
