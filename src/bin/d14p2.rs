use core::slice;
use std::collections::{HashSet, HashMap};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter};

use ndarray::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use strum::{EnumIter, IntoEnumIterator, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Field{
    None,
    Bloc,
    Move,
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::None => write!(f, "."),
            Field::Bloc => write!(f, "#"),
            Field::Move => write!(f, "O"),
        }
    }
}

fn pretty_print(input: &Array2<Field>) {
    for line in input.rows() {
        for c in line {
            print!("{:?}", c);
        }
        println!();
    }
}

#[derive(EnumIter, Display, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction{
    North,
    West,
    South,
    East,
}

impl Direction {
    fn axis(&self) -> Axis {
        match self {
            Direction::North | Direction::South => Axis(1),
            Direction::West | Direction::East => Axis(0),
        }
    }
}

fn process(input: String) -> usize {

    let input : Vec<Vec<_>> = input.lines().map(|line|
        line.chars().map(|c| match c{
            '.' => Field::None,
            '#' => Field::Bloc,
            'O' => Field::Move,
            _ => unreachable!(),
        
        }).collect()
    ).collect();

    let mut input2 = Array2::from_shape_vec((input.len(), input[0].len()), input.clone().into_iter().flatten().collect()).unwrap();

    let timer = std::time::Instant::now();

    let mut last_fall_on = 0;

    for i in 0..1_000_000_000 {

        for direction in Direction::iter() {

            input2.axis_iter_mut(direction.axis()).into_iter().for_each(|mut lane|{

                match direction {
                    Direction::North | Direction::West => (),
                    Direction::South | Direction::East => lane.invert_axis(Axis(0)),
                }
                
                last_fall_on = 0;

                for index_in in 0..lane.len(){
                    match lane[index_in] {
                        Field::None => (),
                        Field::Bloc => last_fall_on = index_in + 1,
                        Field::Move => {
                            lane[index_in] = Field::None;
                            lane[last_fall_on] = Field::Move;
                            last_fall_on += 1;
                        },
                    };
                }

            });
            
        }

        if i % 100_000 == 0 {
            println!("{}: {:?}", i, timer.elapsed());
        }

    }
    
    pretty_print(&input2);

    let height = input.len();

    input2.rows().into_iter().enumerate().map(|(row, line)|{
        line.iter().filter(|c| **c == Field::Move).count() * (height - row)
    }).sum()
    
}

fn main() {
    let input = include_str!("../../input/d14.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d14ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 69);
    }
}