fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
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

            let mut green = 1;
            let mut red = 1;
            let mut blue = 1;

            hands.iter().for_each(|hand| {
                let draws: Vec<&str> = hand.trim().split(',').collect();

                draws.iter().for_each(|draw| {
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

                    if color == "red" && value > red {
                        red = value;
                    } else if color == "blue" && value > blue {
                        blue = value;
                    } else if color == "green" && value > green {
                        green = value;
                    }
                });
            });
            let result = green * red * blue;
            dbg!(result);
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("./testInput.txt");
        assert_eq!(part2(input), 2286);
    }
}
