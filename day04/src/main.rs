use std::{str::FromStr, ops::BitAnd};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u8, combinator::map,
    multi::separated_list0, sequence::preceded, sequence::separated_pair, IResult,
};

use bitmaps::Bitmap;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[derive(Debug)]
struct ScratchCard {
    winning: Bitmap<193>,
    have: Bitmap<193>,
}

impl FromStr for ScratchCard {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parsed: IResult<&str, ScratchCard> = map(
            separated_pair(
                preceded(alt((tag("Card   "), tag("Card  "), tag("Card "))), u8),
                alt((tag(":  "), tag(": "))),
                separated_pair(
                    separated_list0(alt((tag("  "), tag(" "))), u8),
                    alt((tag(" |  "), tag(" | "))),
                    separated_list0(alt((tag("  "), tag(" "))), u8),
                ),
            ),
            |(_, (winning_numbers, have_numbers))| {
                let mut winning = Bitmap::new();
                let mut have = Bitmap::new();

                for wins in winning_numbers {
                    winning.set((wins - 1).into(), true);
                }

                for has in have_numbers {
                    have.set((has - 1).into(), true);
                }

                ScratchCard { winning, have }
            }
        )(input);

        match parsed {
            Ok((_, scratchcard)) => Ok(scratchcard),
            Err(err) => Err(err.to_string()),
        }
    }
}

fn parse(lines: &[String]) -> Vec<ScratchCard> {
    lines
        .iter()
        .map(|line| ScratchCard::from_str(line).unwrap())
        .collect()
}

fn main() {
    let lines = read_input();
    let scratchcards = parse(&lines);

    let winning_counts: Vec<usize> = scratchcards
        .iter()
        .map(|scratchcard| scratchcard.have.bitand(scratchcard.winning).len())
        .collect();

    let part1 = winning_counts.iter().fold(0, |acc, winning_count| {
        if *winning_count == 0 {
            acc
        } else {
            acc + 2u32.pow((*winning_count as u32) - 1)
        }
    });

    let mut card_counts = vec![1; winning_counts.len()];

    for (index, winning_count) in winning_counts.iter().enumerate() {
        let increment = card_counts[index];

        for i in 1..=*winning_count {
            card_counts[index + i] += increment;
        }
    }

    let part2: usize = card_counts.iter().sum();

    println!("part1: {part1}");
    println!("part2: {part2}");
}
