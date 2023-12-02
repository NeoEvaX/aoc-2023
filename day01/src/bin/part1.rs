fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();

    let mut total_calibration = 0;
    for line in lines {
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
    fn test_part1() {
        let input = include_str!("./testInput.txt");
        assert_eq!(part1(input), 142);
    }
}
