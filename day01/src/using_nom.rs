use nom::{
    branch::alt, character::complete::one_of, multi::many0, IResult, bytes::complete::tag, combinator::peek,
};
use fancy_regex::Regex;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn get_digits(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter()
        .map(|line| line.chars().filter(|x| x.is_numeric()).collect()).collect()
}

fn part1(lines: &[Vec<char>]) -> u32 {
    lines.iter()
        .map(|line| {
            let first = line.first().expect("first digit should be present");
            let last = line.last().expect("last digit should be present");

            let combined = format!("{first}{last}");
            let value: u32 = combined.parse().expect("Expected first and last digit to form a number");
            value
        })
        .sum()
}

fn part2(lines: &[String]) -> u32 {
    lines.iter()
        .map(|line| {
            println!();
            println!("line: {line}");
            let nom_value = get_value(line);
            let regex_value = get_value_regex(line);
            assert!(nom_value == regex_value);
            regex_value
        })
        .sum()
}

fn to_digit(value: &str) -> u32 {
    match value {
        "one" | "two" | "three" | "four" | "five" | "six" | "seven" | "eight" | "nine" => spelled_to_digit(value),
        x => str_to_digit(x),
    }
}

fn get_value_regex(line: &str) -> u32 {
    let regex = Regex::new("(?=(one|two|three|four|five|six|seven|eight|nine|[0-9]))").unwrap();

    let captures: Vec<&str> = regex.captures_iter(line).flat_map(|caps| {
        caps
            .unwrap()
            .iter().filter_map(|m| {
                match m {
                    Some(m) => {
                        match m.as_str() {
                            "" => None,
                            s => Some(s)
                        }
                    }
                    None => None,
                }
            }).collect::<Vec<_>>()
    }).collect();
    println!("{:?}", captures);

    let first = captures.first().unwrap();
    let last = captures.last().unwrap();

    let first_digit = to_digit(first);
    let last_digit = to_digit(last);

    let combined = format!("{first_digit}{last_digit}");
    let value: u32 = combined.parse().expect("Expected first and last digit to form a number");

    value
}

#[derive(Debug)]
enum Value {
    Spelled(String),
    Digit(char),
    Other,
}

fn letter(input: &str) -> IResult<&str, Value> {
    let (matched, _) = one_of("abcdefghijklmnopqrstuvwxyz")(input)?;
    Ok((matched, Value::Other))
}

fn digit(input: &str) -> IResult<&str, Value> {
    let (matched, digit) = one_of("123456789")(input)?;
    Ok((matched, Value::Digit(digit)))
}

fn spelled(input: &str) -> IResult<&str, Value> {
    let (matched, spelled) = alt((
            peek(tag("one")),
            peek(tag("two")),
            peek(tag("three")),
            peek(tag("four")),
            peek(tag("five")),
            peek(tag("six")),
            peek(tag("seven")),
            peek(tag("eight")),
            peek(tag("nine"))
        ))(input)?;
    Ok((matched, Value::Spelled(spelled.to_string())))
}

fn numbers(input: &str) -> IResult<&str, Vec<Value>> {
    many0(alt((spelled, digit, letter)))(input)
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

fn str_to_digit(value: &str) -> u32 {
    value.to_string().parse().unwrap()
}

fn not_is_other(value: &Value) -> bool {
    !matches!(value, Value::Other)
}

fn values_from_string(input: &str) -> (String, String) {
    use Value::{Digit, Other};

    match numbers(input) {
        Ok((_, found)) => {
            let x: Vec<&Value> = found.iter().filter(|&x| not_is_other(x)).collect();
            println!("found: {:?}", x);

            let values: Vec<String> = found.iter()
                .filter_map(|x| {
                    match x {
                        Digit(x) => {
                            Some(str_to_digit(x.to_string().as_str()).to_string())
                        },
                        Value::Spelled(x) => {
                            let value = spelled_to_digit(&x);

                            Some(value.to_string())

                        },
                        Other => None,
                    }
                }).collect();

            let first = values.first().unwrap();
            let last = values.last().unwrap();
            println!("first: {first}, last: {last}");
            (first.to_string(), last.to_string())
        }
        Err(err) => {
            println!("{}", err);
            unreachable!()
        },
    }
}

fn get_value(line: &str) -> u32 {
    let (first, last) = values_from_string(line);
    let combined = format!("{first}{last}");
    let value: u32 = combined.parse().expect("Expected first and last digit to form a number");
    value
}

fn main() {
    let lines = read_input();
    let parsed = get_digits(&lines);

    println!("part2: {}", part2(&lines));
    println!("part1: {}", part1(&parsed));
}

