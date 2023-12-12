fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

type Record = (Conditions, GroupSizes, String);

type Conditions = Vec<Condition>;

#[derive(Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

type GroupSizes = Vec<u8>;

struct Records {
    records: Vec<Record>,
}

fn parse(lines: &[String]) -> Records {
    let records = lines
        .iter()
        .map(|line| {
            let (conditions, group_sizes) = line.split_once(' ').unwrap();

            let conditions = conditions
                .chars()
                .map(|c| match c {
                    '?' => Condition::Unknown,
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    _ => unreachable!()
                })
                .collect();

            let group_sizes = group_sizes
                .split(',')
                .map(|g| {
                    g.parse().unwrap()
                })
                .collect();

            (conditions, group_sizes, line.clone())
        })
        .collect();

    Records { records }
}

fn conditions_group_sizes(conditions: &Conditions) -> Vec<usize> {
    let mut buffer = 0;
    let mut group_sizes = vec![];

    for condition in conditions {
        match condition {
            Condition::Operational => {
                if buffer > 0 {
                    group_sizes.push(buffer);
                    buffer = 0;
                }
            },
            Condition::Damaged => {
                buffer += 1;
            },
            Condition::Unknown => unreachable!(),
        }
    }

    if buffer > 0 {
        group_sizes.push(buffer);
    }

    group_sizes
}

fn conditions_to_string(conditions: &Conditions) -> String {
    let mut output = String::new();

    for condition in conditions {
        match condition {
            Condition::Operational => output += ".",
            Condition::Damaged => output += "#",
            Condition::Unknown => output += "?",
        }
    }

    output
}

fn is_valid_arrangement(conditions: &Conditions, expected_group_sizes: &GroupSizes) -> bool {
    println!("is valid: {}", conditions_to_string(conditions));

    let actual_group_sizes = conditions_group_sizes(conditions);
    println!("actual_group_sizes: {actual_group_sizes:?}");

    if actual_group_sizes.len() != expected_group_sizes.len() {
        return false;
    }

    for (expected, actual) in expected_group_sizes.iter().zip(actual_group_sizes) {
        if (*expected as usize) != actual {
            println!("false");
            return false;
        }
    }

    println!("true");
    true
}

fn backtrack_arangements(conditions: &Conditions, group_sizes: &GroupSizes) -> u32 {
    match conditions.iter().position(|condition| matches!(condition, Condition::Unknown)) {
        // If there are no unknown springs
        None => {
            u32::from(is_valid_arrangement(conditions, group_sizes))
        },
        Some(unknown_index) => {
            let mut cloned = conditions.clone();
            let mut valid = 0;

            // assign operational
            cloned[unknown_index] = Condition::Operational;

            // if is_valid_arrangement(&cloned, group_sizes) {
                valid += backtrack_arangements(&cloned, group_sizes);
            // }

            // assign damaged
            cloned[unknown_index] = Condition::Damaged;

            // if is_valid_arrangement(&cloned, group_sizes) {
                valid += backtrack_arangements(&cloned, group_sizes);
            // }

            valid
        },
    }
}

fn part1(records: &Records) -> u32 {
    records.records.iter()
        .map(|(conditions, group_sizes, line)| {
            println!("line: {line}");
            let arrangement_count = backtrack_arangements(conditions, group_sizes);
            println!("arrangements: {arrangement_count}");

            arrangement_count
        })
        .sum()
}

fn part2(records: &Records) -> u32 {
    records.records.iter()
        .map(|(conditions, group_sizes, line)| {
            println!("line: {line}");

            let group_sizes = vec![group_sizes.clone(); 5].concat();
            let mut conditions_extended = conditions.clone();
            for i in 0..4 {
                conditions_extended.push(Condition::Unknown);
                conditions_extended = [conditions_extended, conditions.clone()].concat();
            }

            let mut last = Condition::Operational;
            let mut conditions_extended: Vec<_> = conditions_extended.into_iter()
                .filter(|condition| {
                    match condition {
                        Condition::Operational if matches!(last, Condition::Operational) => false,
                        condition => {
                            last = condition.clone();
                            true
                        },
                    }
                })
                .collect();

            if matches!(conditions_extended.iter().last().unwrap(), Condition::Operational) {
                conditions_extended.pop();
            }

            println!("{} {group_sizes:?}", conditions_to_string(&conditions_extended));

            let operational_count = conditions_extended.iter().filter(|condition| matches!(condition, Condition::Operational)).count();
            let required_operational_count = group_sizes.len() - 1;
            println!("current operational_count: {operational_count}, required splits: {required_operational_count}, remaining: {}", (required_operational_count as i32) - (operational_count as i32));
            // let arrangement_count = backtrack_arangements(conditions, group_sizes);
            // println!("arrangements: {arrangement_count}");

            println!();

            0
        })
        .sum()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    // println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
