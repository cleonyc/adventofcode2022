#![feature(extend_one)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() -> anyhow::Result<()> {
    println!("solution 1");
    solution_1()?;
    println!("solution 2 ");
    solution_2()?;
    Ok(())
}
fn solution_2() -> anyhow::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut bufreader = BufReader::new(&mut input_file);
    let mut input = String::new();
    bufreader.read_to_string(&mut input)?;
    let mut elf_vals: Vec<usize> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cals| cals.parse::<usize>().unwrap_or(0))
                .sum::<usize>()
        })
        .collect();
    elf_vals.sort();
    let part1 = elf_vals.last().unwrap();
    println!("part 1: {part1}");
    let part2: usize = elf_vals.iter().rev().take(3).sum();
    println!("part 2: {part2}");
    Ok(())
}
fn solution_1() -> anyhow::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let bufreader = BufReader::new(&mut input_file);
    let mut elf_vals: Vec<usize> = vec![];
    let mut i = 0;
    for line in bufreader.lines() {
        let line = line?;
        if line.is_empty() {
            i += 1;
            elf_vals.extend_one(0);
            continue;
        }
        elf_vals[i] += line.parse::<usize>()?;
    }
    elf_vals.sort();
    let part1 = elf_vals.last().unwrap();
    println!("part 1: {part1}");
    let part2: usize = elf_vals.iter().rev().take(3).sum();
    println!("part 2: {part2}");
    Ok(())
}
