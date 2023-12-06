use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, char, newline, space1, i32},
    multi::{separated_list0, separated_list1, many1},
    sequence::{preceded, tuple},
    IResult,
};

fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

fn time(input: &str) -> IResult<&str, Vec<i32>> {
    let (extra, times) = preceded(tuple((tag("Time:"), space1)), separated_list0(space1, i32))(input)?;

    Ok((extra, times))
}

fn records(input: &str) -> IResult<&str, Vec<i32>> {
    let (extra, records) = preceded(tuple((tag("Distance:"), space1)), separated_list0(space1, i32))(input)?;

    Ok((extra, records))
}

fn whole_input(input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    let (extra, (times, _, records)) =
        tuple((time, newline, records))(input)?;

    Ok((extra, (times, records)))
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    match whole_input(input) {
        Ok((_, result)) => result,
        Err(err) => {
            println!("error: {err:#?}");
            panic!();
        }
    }
}

fn get_winning_possibility_count(time: &i32, record: &i32) -> i32 {
    (0..=*time).fold(0, |acc, hold_time| {
        if i32::max(hold_time * ((*time) - hold_time), 0) > *record {
            // println!("hold_time: {}", hold_time);
            acc + 1
        } else {
            acc
        }
    })
}

fn part1(times: &[i32], records: &[i32]) -> i32 {
    times.iter().zip(records)
        .map(|(time, record)| {
            get_winning_possibility_count(time, record)
        })
        .product()
}

fn to_single_number(parts: &[i32]) -> i32 {
    parts.iter().rev()
        .fold((0, 1), |(total, index), part| {
            let new_total = total + index * part;
            let new_index = index * (10i32.pow(part.to_string().len() as u32));

            (new_total, new_index)
        }).0
}

fn part2(times: &[i32], records: &[i32]) -> i32 {
    let time = to_single_number(times);
    let record = to_single_number(records);

    let a: f64 = 1.0;
    let b: f64 = (-time).into();
    let c: f64 = record.into();

    let d = b.powi(2) - 4.0 * a * c;

    let x1 = (-b + d.sqrt()) / (2.0 * a);
    let x2 = (-b - d.sqrt()) / (2.0 * a);

    (x1.ceil() - x2.ceil().abs()) as i32
}

fn main() {
    let lines = read_input();
    let (times, records) = parse(&lines);

    println!("part1: {}", part1(&times, &records));
    println!("part2: {}", part2(&times, &records));
}
