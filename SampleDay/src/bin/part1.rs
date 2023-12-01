fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let gnomes = input.split("\n\n");

    let mut held_calories: Vec<u32> = gnomes
        .map(|calories| {
            calories
                .split('\n')
                .flat_map(|num| num.parse::<u32>())
                .sum()
        })
        .collect();

    held_calories.sort_by(|a, b| b.cmp(a));
    held_calories[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./testInput.txt");
        assert_eq!(part1(input), "142");
    }
}
