fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

type History = Vec<i32>;
type Report = Vec<History>;

fn parse(lines: &[String]) -> Report {
    lines.iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(history: &History) -> i32 {
    let mut endings = vec![];
    let mut steps = history.clone();

    let mut only_zero = false;
    while !only_zero {
        only_zero = true;
        let last = *steps.last().unwrap();
        endings.push(last);

        let mut previous = steps[0];
        (1..steps.len()).for_each(|i| {
            let current = steps[i];
            if current != 0 {
                only_zero = false;
            }
            steps[i - 1] = current - previous;
            previous = current;
        });

        steps.pop();
    }

    endings.iter().sum()
}

fn part1(report: &Report) -> i32 {
    report.iter().map(extrapolate).sum()
}

fn extrapolate_start(history: &History) -> i32 {
    let history_reverse = history.iter().rev().copied().collect();
    extrapolate(&history_reverse)
}

fn part2(report: &Report) -> i32 {
    report.iter().map(extrapolate_start).sum()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
