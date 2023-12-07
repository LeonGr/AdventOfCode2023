use std::{cmp::Ordering, fmt, str::FromStr};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[repr(u8)]
#[derive(Clone)]
enum Card {
    Number(u8),
    JackJoker = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(10) => write!(f, "T"),
            Self::Number(d) => write!(f, "{d}"),
            Self::JackJoker => write!(f, "J"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
            Self::Ace => write!(f, "A"),
        }
    }
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::Number(n) => *n,
            _ => unsafe { *(self as *const Self).cast::<u8>() },
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card = match s {
            "A" => Card::Ace,
            "K" => Card::King,
            "Q" => Card::Queen,
            "J" => Card::JackJoker,
            "T" => Card::Number(10),
            d => Card::Number(d.parse().unwrap()),
        };

        Ok(card)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for Card {}

const CARD_OPTIONS: usize = 13;

#[repr(u8)]
#[derive(Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cards = format!("{:?}", self.cards);
        write!(f, "hand: {}, bid: {}", cards, self.bid)
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut occurrences = [0; CARD_OPTIONS];

        let max_occurences = self.cards.iter().fold(0, |acc, card| {
            let value = card.value();
            let card_index = (value - 2) as usize;
            occurrences[card_index] += 1;

            let count = occurrences[card_index];

            acc.max(count)
        });

        match max_occurences {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            1 => HandType::HighCard,
            3 => {
                if occurrences.contains(&2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if occurrences
                    .iter()
                    .filter(|&occurrences| *occurrences == 2)
                    .count()
                    == 2
                {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => unreachable!("Max occurrences should be in [1; 5]"),
        }
    }

    fn has_joker(&self) -> bool {
        self.cards.contains(&Card::JackJoker)
    }

    fn get_joker_type(&self) -> HandType {
        if self.has_joker() {
            let mut occurrences = [0; CARD_OPTIONS];
            let joker_position: usize = 9;

            let max_occurences = self.cards.iter().fold(0, |acc, card| {
                let value = card.value();
                let card_index = (value - 2) as usize;
                occurrences[card_index] += 1;

                let count = occurrences[card_index];

                acc.max(count)
            });

            match max_occurences {
                4 | 5 => HandType::FiveOfAKind,
                1 => HandType::OnePair,
                3 => {
                    if occurrences[joker_position] == 3 {
                        if occurrences.contains(&2) {
                            HandType::FiveOfAKind
                        } else {
                            HandType::FourOfAKind
                        }
                    } else if occurrences[joker_position] == 2 {
                        HandType::FiveOfAKind
                    } else if occurrences[joker_position] == 1 {
                        HandType::FourOfAKind
                    } else {
                        unreachable!();
                    }
                }
                2 => {
                    if occurrences
                        .iter()
                        .filter(|&occurrences| *occurrences == 2)
                        .count()
                        == 2
                    {
                        if occurrences[joker_position] == 2 {
                            HandType::FourOfAKind
                        } else {
                            HandType::FullHouse
                        }
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
                _ => unreachable!("Max occurrences should be in [1; 5]"),
            }
        } else {
            self.get_type()
        }
    }
}

fn cmp_with_jokers(first: &Hand, second: &Hand) -> std::cmp::Ordering {
    let first_highest_possible = first.get_joker_type();
    let second_highest_possible = second.get_joker_type();

    if first_highest_possible.value() == second_highest_possible.value() {
        for (card_first, card_second) in first.cards.iter().zip(&second.cards) {
            let first_value = match card_first.value() {
                11 => 1,
                d => d,
            };

            let second_value = match card_second.value() {
                11 => 1,
                d => d,
            };

            match first_value.cmp(&second_value) {
                Ordering::Equal => (),
                other => return other
            }
        }

        unreachable!()
    } else {
        first_highest_possible
            .value()
            .cmp(&second_highest_possible.value())
    }
}

fn order_with_jokers(hands: &mut [Hand]) {
    hands.sort_by(cmp_with_jokers);
}

impl HandType {
    fn value(&self) -> u8 {
        unsafe { *(self as *const Self).cast::<u8>() }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type().value().cmp(&other.get_type().value()) {
            Ordering::Equal => {
                for (card_self, card_other) in self.cards.iter().zip(&other.cards) {
                    match card_self.value().cmp(&card_other.value()) {
                        Ordering::Equal => (),
                        other => return other
                    }
                }

                unreachable!("Hand types are equal, at least one card must be higher");
            }
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Eq for Hand {}

fn parse(lines: &[String]) -> Vec<Hand> {
    lines
        .iter()
        .map(|line| {
            let (cards_str, bid) = line.split_once(' ').unwrap();

            let cards: [Card; 5] = cards_str
                .chars()
                .map(|c| Card::from_str(&c.to_string()).unwrap())
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();
            let bid = bid.parse().unwrap();

            Hand { cards, bid }
        })
        .collect()
}

fn part1(hands: &mut [Hand]) -> u32 {
    hands.sort();

    hands.iter().enumerate().fold(0, |acc, (rank, hand)| {
        let rank: u32 = (rank + 1).try_into().unwrap();
        let hand_value = rank * hand.bid;
        acc + hand_value
    })
}

fn part2(hands: &mut [Hand]) -> u32 {
    order_with_jokers(hands);

    hands.iter().enumerate().fold(0, |acc, (rank, hand)| {
        let rank: u32 = (rank + 1).try_into().unwrap();
        let hand_value = rank * hand.bid;
        acc + hand_value
    })
}

fn main() {
    let lines = read_input();
    let mut hands = parse(&lines);

    println!("part1: {}", part1(&mut hands));
    println!("part2: {}", part2(&mut hands));
}
