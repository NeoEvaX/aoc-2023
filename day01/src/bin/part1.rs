fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|character| character.to_digit(10));

            let first_digit = digits.next().expect("should be a number");

            match digits.last() {
                Some(num) => first_digit * 10 + num,
                None => first_digit * 10 + first_digit,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./testInput.txt");
        assert_eq!(part1(input), 142);
    }
}
