fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let game = line.rsplit_once(": ").expect("should be a game");

            let game_number = game
                .0
                .chars()
                .skip_while(|ch| !ch.is_ascii_digit())
                .take_while(|ch| ch.is_ascii_digit())
                .fold(None, |acc, ch| {
                    ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
                })
                .expect("should be a game number");

            let hands: Vec<&str> = game.1.split(';').collect();

            let result: u32 = hands
                .iter()
                .map(|hand| {
                    let draws: Vec<&str> = hand.trim().split(',').collect();

                    let cube_number: u32 = draws
                        .iter()
                        .map(|draw| {
                            let value = draw
                                .split_ascii_whitespace()
                                .next()
                                .expect("should be there")
                                .parse::<u32>()
                                .expect("should be a digit");

                            let color = draw
                                .split_ascii_whitespace()
                                .last()
                                .expect("should be there");

                            if (color == "red" && value > 12)
                                || (color == "blue" && value > 14)
                                || (color == "green" && value > 13)
                            {
                                1
                            } else {
                                0
                            }
                        })
                        .sum();
                    cube_number
                })
                .sum();
            dbg!(result);
            if result > 0 {
                0
            } else {
                game_number
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
        assert_eq!(part1(input), 8);
    }
}
