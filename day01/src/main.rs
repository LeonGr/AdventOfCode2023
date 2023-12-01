fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn get_digits_as_chars(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().filter(|x| x.is_numeric()).collect())
        .collect()
}

fn combine(first_digit: u32, second_digit: u32) -> u32 {
    first_digit * 10 + second_digit
}

fn part1(lines: &[Vec<char>]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let first = line.first().unwrap().to_string();
            let last = line.last().unwrap().to_string();

            combine(
                first.parse().expect("should be a digit"),
                last.parse().expect("should be a digit"),
            )
        })
        .sum()
}

fn part2(lines: &[String]) -> u32 {
    lines.iter().map(|line| {
        let first: u32 = get_first_value(line);
        let last: u32 = get_last_value(line);

        combine(first, last)
    }).sum()
}

fn get_first_value(line: &str) -> u32 {
    for i in 0..line.len() {
        let single_char = &line[i..=i];

        match single_char {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                return single_char.parse().unwrap();
            }
            _ => (),
        };

        for j in 3..=5 {
            if i + j > line.len() {
                break;
            }

            let slice = &line[i..(i + j)];
            match slice {
                "one" | "two" | "three" | "four" | "five" | "six" | "seven" | "eight" | "nine" => {
                    return spelled_to_digit(slice);
                }
                _ => (),
            }
        }
    }

    unreachable!()
}

fn get_last_value(line: &str) -> u32 {
    for i in (0..line.len()).rev() {
        let single_char = &line[i..=i];

        match single_char {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                return single_char.parse().unwrap();
            }
            _ => (),
        };

        for j in 2..5 {
            if j > i {
                break;
            }

            let slice = &line[(i - j)..=i];
            match slice {
                "one" | "two" | "three" | "four" | "five" | "six" | "seven" | "eight" | "nine" => {
                    return spelled_to_digit(slice);
                }
                _ => (),
            }
        }
    }

    unreachable!()
}

fn spelled_to_digit(spelled: &str) -> u32 {
    match spelled {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

fn main() {
    let lines = read_input();
    let parsed = get_digits_as_chars(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&lines));
}
