use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, alphanumeric1},
    multi::{separated_list1, many1},
    sequence::{tuple, delimited},
    IResult, branch::alt,
};

fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

enum Direction {
    Left,
    Right,
}

type Directions = Vec<Direction>;

#[derive(Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

type Nodes = Vec<Node>;

fn directions(input: &str) -> IResult<&str, Directions> {
    let (extra, directions) =
        many1(
            alt((tag("L"),
                 tag("R"))))(input)?;

    let directions = directions.iter()
        .map(|&direction| {
            match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!("Only directions are L and R"),
            }
        })
        .collect();

    Ok((extra, directions))
}

fn nodes(input: &str) -> IResult<&str, Nodes> {
    let (extra, nodes) =
        separated_list1(
            newline,
            tuple(
                (alphanumeric1,
                 tag(" = "),
                 delimited(
                     tag("("),
                     tuple(
                         (alphanumeric1, tag(", "), alphanumeric1)),
                     tag(")"))))
                       )(input)?;

    let nodes = nodes.iter()
        .map(|&(id, _, (left, _, right))| {
            Node {
                id: id.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            }
        })
        .collect();

    Ok((extra, nodes))
}

fn whole_input(input: &str) -> IResult<&str, (Directions, Nodes)> {
    let (extra, (directions, _, nodes)) =
        tuple((directions, many1(newline), nodes))(input)?;

    Ok((extra, (directions, nodes)))
}

fn parse(input: &str) -> (Directions, Nodes) {
    match whole_input(input) {
        Ok((_, result)) => result,
        Err(err) => {
            println!("error: {err:#?}");
            panic!();
        }
    }
}

fn part1(directions: &Directions, nodes: &Nodes) -> usize {
    let node_map: HashMap<String, Node> = nodes.iter()
        .map(|node| {
            (node.id.clone(), node.clone())
        })
        .collect();

    let dir_len = directions.len();

    let mut current_id = "AAA";
    let mut steps = 0;
    while current_id != "ZZZ" {
        let current_direction = &directions[steps % dir_len];
        let current_node = node_map.get(current_id).unwrap_or_else(|| panic!("Every node should exist, {current_id} doesn't"));

        current_id =
            match current_direction {
                Direction::Left => {
                    &current_node.left
                },
                Direction::Right => {
                    &current_node.right
                },
            };

        steps += 1;
    }

    steps
}

fn count_steps(node_map: &HashMap<String, Node>, directions: &Directions, start: &str) -> usize {
    let dir_len = directions.len();

    let mut current_id = start;
    let mut steps = 0;
    while !current_id.ends_with('Z') {
        let current_direction = &directions[steps % dir_len];
        let current_node = node_map.get(current_id).expect("Every node should exist");

        current_id =
            match current_direction {
                Direction::Left => {
                    &current_node.left
                },
                Direction::Right => {
                    &current_node.right
                },
            };

        steps += 1;
    }

    steps
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b > 0 {
        let temp = a;
        a = b;
        b = temp % a;
    }

    a
}

fn lcm_two(a: usize, b: usize) -> usize {
    (a * b).div_euclid(gcd(a, b))
}

fn lcm(numbers: &[usize]) -> usize {
    let a: Vec<usize> = numbers.to_vec();
    a.into_iter().reduce(|acc, e| {
        lcm_two(acc, e)
    }).unwrap()
}

fn part2(directions: &Directions, nodes: &Nodes) -> usize {
    let node_map: HashMap<String, Node> = nodes.iter()
        .map(|node| {
            (node.id.clone(), node.clone())
        })
        .collect();

    let start_points: Vec<String> = nodes.iter()
        .filter_map(|node| {
            if node.id.ends_with('A') {
                Some(node.id.clone())
            } else {
                None
            }
        })
        .collect();

    println!("start points: {start_points:?}");

    let steps: Vec<usize> =
        start_points.iter()
            .map(|start_point| {
                count_steps(&node_map, directions, start_point)
            })
            .collect();

    lcm(&steps)
}

fn main() {
    let lines = read_input();
    let (directions, nodes) = parse(&lines);

    println!("part1: {}", part1(&directions, &nodes));
    println!("part2: {}", part2(&directions, &nodes));
}
