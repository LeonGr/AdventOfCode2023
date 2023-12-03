fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse(lines: &[String]) -> Vec<Game> {
    lines
        .iter()
        .map(|line| {
            let (id, game) = line.split_once(": ").unwrap();
            let id: u32 = id.split_once(" ").unwrap().1.parse().unwrap();

            let sets: Vec<Set> = game
                .split("; ")
                .map(|set| {
                    let mut new_set = Set::new();

                    set.split(", ").for_each(|cube| {
                        let (count, color) = cube.split_once(" ").unwrap();
                        let count: u32 = count.parse().unwrap();

                        match color {
                            "red" => new_set.red = count,
                            "green" => new_set.green = count,
                            "blue" => new_set.blue = count,
                            _ => unreachable!(),
                        }
                    });

                    new_set
                })
                .collect();

            Game { id, sets }
        })
        .collect()
}

fn is_possible(game: &Game, available: &Set) -> bool {
    game.sets.iter().all(|set| {
        set.red <= available.red && set.green <= available.green && set.blue <= available.blue
    })
}

fn part1(games: &[Game]) -> u32 {
    println!("{:#?}", games);

    let available = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .iter()
        .filter_map(|game| {
            if is_possible(game, &available) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let minimum = game.sets.iter().fold(Set::new(), |acc, set| Set {
                red: u32::max(acc.red, set.red),
                green: u32::max(acc.green, set.green),
                blue: u32::max(acc.blue, set.blue),
            });

            minimum.red * minimum.green * minimum.blue
        })
        .sum()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
