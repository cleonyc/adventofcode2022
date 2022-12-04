#![feature(iter_array_chunks)]
use std::collections::{HashSet, HashMap};
use std::io::Read;
use std::{fs::File, io::BufReader};

fn main() -> anyhow::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut bufreader = BufReader::new(&mut input_file);
    let mut input = String::new();
    bufreader.read_to_string(&mut input)?;
    let part1 = input
        .split('\n')
        .map(|line| {
            let compartment_strs = line.split_at(line.len() / 2);
            let (mut first_comp, second_comp): (Vec<char>, Vec<char>) = (
                compartment_strs.0.chars().collect(),
                compartment_strs.1.chars().collect(),
            );
            first_comp.retain(|char| second_comp.contains(char));
            first_comp.sort();
            first_comp.dedup();
            first_comp
                .iter()
                .map(to_priority)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("part 1: {part1}");
    let part2 = input
        .split('\n')
        .array_chunks()
        .map(|group: [&str; 3]| {
            let mut items_in_rucksacks: Vec<char> = group
                .map(|rucksack| {
                    let mut chars: Vec<char> = rucksack.chars().collect();
                    chars.sort();
                    chars.dedup();
                    chars
                })
                .iter_mut()
                .reduce(|cur, prev| {
                    cur.append(prev);
                    cur
                })
                .unwrap().clone();
            items_in_rucksacks.sort();
            let mut cool_map: HashMap<char, usize> = HashMap::new();
            let item = items_in_rucksacks.iter().reduce(|prev, cur| {
                if !cool_map.contains_key(prev) {
                    cool_map.insert(*prev, 1);
                }
                cool_map.insert(*cur, if cool_map.contains_key(cur) {cool_map[cur] + 1} else {1});
                if cool_map[prev] > cool_map[cur] {prev} else {cur}
            }).unwrap();
            to_priority(item)
        })
        .sum::<usize>();
    println!("part 2: {part2}");
    Ok(())
}
fn to_priority(item: &char) -> usize {
    if item.is_lowercase() {
        *item as usize - 0x60
    } else {
        *item as usize - 0x26
    }
}
