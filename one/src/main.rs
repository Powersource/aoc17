use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("in.txt").unwrap();
    let mut input = BufReader::new(file);
    let mut in_string = String::new();
    input.read_to_string(&mut in_string).unwrap();
    println!("Answer: {}", solve(&in_string));
    println!("Answer: {}", solve2(&in_string));
}

fn solve(input: &str) -> i32 {
    let input = input.as_bytes();
    let mut sum: i32 = 0;
    if &input[0] == input.last().unwrap() {
        sum += i32::from(input[0]) - '0' as i32
    };
    input.windows(2).for_each(|a| if a[0] == a[1] {
        sum += i32::from(a[0]) - '0' as i32
    });
    sum
}

fn solve2(input: &str) -> i32 {
    let input = input.as_bytes();
    let (a, b) = input.split_at(input.len() / 2);
    let mut sum: i32 = 0;
    a.iter().zip(b).for_each(|tup| if tup.0 == tup.1 {
        sum += (i32::from(*tup.0) - '0' as i32) * 2
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::{solve, solve2};

    #[test]
    fn one_two() {
        assert_eq!(solve("1122"), 3);
    }

    #[test]
    fn one() {
        assert_eq!(solve("1111"), 4);
    }

    #[test]
    fn none() {
        assert_eq!(solve("1234"), 0);
    }

    #[test]
    fn nines() {
        assert_eq!(solve("91212129"), 9);
    }

    #[test]
    fn one_two_2() {
        assert_eq!(solve2("1212"), 6);
    }

    #[test]
    fn mirror() {
        assert_eq!(solve2("1221"), 0);
    }

    #[test]
    fn counting() {
        assert_eq!(solve2("123425"), 4);
    }

    #[test]
    fn same() {
        assert_eq!(solve2("123123"), 12);
    }

    #[test]
    fn ones() {
        assert_eq!(solve2("12131415"), 4);
    }
}