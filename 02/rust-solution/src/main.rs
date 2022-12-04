use std::{fs::File, io::{BufReader, Read}, str::FromStr, cmp::Ordering, ops::{Add, Sub}};

use strum::{EnumString, FromRepr};

#[derive(EnumString, PartialEq, Clone, Eq, FromRepr, Debug)]
#[repr(u8)]
enum RPS {
    #[strum(serialize = "X", serialize = "A")]
    Rock = 1,
    #[strum(serialize = "Y", serialize = "B")]
    Paper = 2,
    #[strum(serialize = "Z", serialize = "C")]
    Scissors = 3,
}
impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self {
            RPS::Rock => match other {
                RPS::Rock => Ordering::Equal,
                RPS::Paper => Ordering::Less,
                RPS::Scissors => Ordering::Greater,
            }
            ,
            RPS::Paper => match other {
                RPS::Rock => Ordering::Greater,
                RPS::Paper => Ordering::Equal,
                RPS::Scissors => Ordering::Less,
            },
            RPS::Scissors => match other {
                RPS::Rock => Ordering::Less,
                RPS::Paper => Ordering::Greater,
                RPS::Scissors => Ordering::Equal,
            },
        })
    }
}
impl RPS {
    fn points(&self, other: Self) -> Outcome {
        match self.partial_cmp(&other).unwrap() {
            Ordering::Less => Outcome::Lose,
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Win,
        }
    }
}
impl Add<isize> for RPS {
    type Output = Self;

    fn add(self, rhs: isize) -> Self {
        Self::from_repr(((self as isize + rhs - 1) % 3 + 1).try_into().unwrap()).unwrap()
    }
}
impl Sub<isize> for RPS {
    type Output = Self;

    fn sub(self, rhs: isize) -> Self {
        let thing = self as isize - rhs - 1;
        Self::from_repr((if thing == -1 {3} else {thing % 3 + 1}).try_into().unwrap()).unwrap()
    }
}
#[derive(EnumString, PartialEq, Clone, Debug)]
enum Outcome {
      #[strum(serialize = "Z")]
      Win = 6,
      #[strum(serialize = "Y")]
      Draw = 3,
      #[strum(serialize = "X")]
      Lose = 0
}
impl Outcome {
    fn needed(&self, round: RPS) -> RPS {
        match *self {
            Outcome::Win => round + 1,
            Outcome::Draw => round,
            Outcome::Lose => round - 1,
        }
    }
}
fn main() -> anyhow::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut bufreader = BufReader::new(&mut input_file);
    let mut input = String::new();
    bufreader.read_to_string(&mut input)?;
    let part1: Vec<usize> = input.split('\n').map(|round| {
        let moves = round.split(' ').map(|a| RPS::from_str(a).unwrap()).collect::<Vec<RPS>>();
        moves[1] as usize + moves[1].points(moves[0].clone()) as usize
    }).collect();
    println!("part 1: {}", part1.iter().sum::<usize>());
    let part2: Vec<usize> = input.split('\n').map(|round| {
        let split = round.split(' ').collect::<Vec<&str>>();
        let (elf_move, outcome) = ( RPS::from_str(split[0]).unwrap(), Outcome::from_str(split[1]).unwrap());
        outcome.needed(elf_move) as usize + outcome as usize
    }).collect();
    println!("part 2: {}", part2.iter().sum::<usize>());
    Ok(())
}
