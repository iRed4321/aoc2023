use std::collections::HashMap;
use ndarray::prelude::*;
use strum::{EnumIter, IntoEnumIterator, Display};

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
            '.' => Some(false),
            '#' => None,
            'O' => Some(true),
            _ => unreachable!(),
        
        }).collect()
    ).collect();

    let mut input = Array2::from_shape_vec((input.len(), input[0].len()), input.clone().into_iter().flatten().collect()).unwrap();
    let mut last_fall_on = 0;
    let mut cache = HashMap::new();
    let mut i = 0;
    
    while i < 1_000_000_000 {

        for direction in Direction::iter() {

            input.axis_iter_mut(direction.axis()).into_iter().for_each(|mut lane|{

                match direction {
                    Direction::North | Direction::West => (),
                    Direction::South | Direction::East => lane.invert_axis(Axis(0)),
                }
                
                last_fall_on = 0;
    
                for index_in in 0..lane.len(){
                    last_fall_on = match lane[index_in] {
                        Some(false) => continue,
                        None => index_in + 1,
                        Some(true) => {
                            lane[index_in] = Some(false);
                            lane[last_fall_on] = Some(true);
                            last_fall_on + 1
                        },
                    };
                }
    
            });
            
        }

        if let Some(&prev) = cache.get(&input) {
            let cycle = i - prev;
            let remaining = (1_000_000_000 - i) % cycle;
            i = 1_000_000_000 - remaining;
            cache.clear();
        } else {
            cache.insert(input.clone(), i);
        }
        
        i += 1;

    }
    
    input.rows().into_iter().enumerate().map(|(row, line)|{
        line.iter().filter(|c| **c == Some(true)).count() * (input.dim().0 - row)
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
        assert_eq!(res, 64);
    }
}