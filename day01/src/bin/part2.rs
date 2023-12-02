fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();

    let mut total_calibration = 0;

    for line in lines {
        let mut line = line.to_string();
        line = line.replace("one", "one1one");
        line = line.replace("two", "two2two");
        line = line.replace("three", "three3three");
        line = line.replace("four", "four4four");
        line = line.replace("five", "five5five");
        line = line.replace("six", "six6six");
        line = line.replace("seven", "seven7seven");
        line = line.replace("eight", "eight8eight");
        line = line.replace("nine", "nine9nine");
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let calibration = digits[0] * 10 + digits.last().unwrap();
        total_calibration += calibration;
    }

    total_calibration
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
