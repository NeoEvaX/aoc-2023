fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            line = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");
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
    fn test_part2() {
        let input = include_str!("./testInput2.txt");
        assert_eq!(part2(input), 281);
    }
}
