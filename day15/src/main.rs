use std::collections::HashMap;

fn read_input() -> String {
    let input = include_str!("../input");
    input.trim().to_string()
}

fn hash(label: &str) -> u32 {
    label
        .chars()
        .fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn part1(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

fn part2(input: &str) -> u32 {
    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    for step in input.split(',') {
        let (label, focal_length) = if let Some((label, focal_length)) = step.split_once('=') {
            (label, Some(focal_length.parse().unwrap()))
        } else if let Some((label, _)) = step.split_once('-') {
            (label, None)
        } else {
            unreachable!()
        };

        let relevant_box = boxes.entry(hash(label));

        if let Some(focal_length) = focal_length {
            relevant_box
                .and_modify(|lenses| {
                    let index = lenses
                        .iter()
                        .position(|&(entry_label, _)| entry_label == label);

                    if let Some(index) = index {
                        lenses[index] = (label, focal_length);
                    } else {
                        lenses.push((label, focal_length));
                    }
                })
                .or_insert(vec![(label, focal_length)]);
        } else {
            relevant_box
                .and_modify(|lenses| {
                    let index = lenses
                        .iter()
                        .position(|&(entry_label, _)| entry_label == label);

                    if let Some(index) = index {
                        lenses.remove(index);
                    }
                });
        }
    }

    boxes
        .iter()
        .map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_index, (_, focal_length))| {
                    (box_index + 1) * (u32::try_from(lens_index).unwrap() + 1) * focal_length
                })
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let input = read_input();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
