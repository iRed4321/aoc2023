use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ndarray::prelude::*;

fn pretty_print(input: &Vec<Vec<char>>) {
    for line in input {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field{
    None,
    Bloc,
    Move,
}

fn process(input: String) -> usize {

    let mut input : Vec<Vec<_>> = input.lines().map(|line|
        line.chars().map(|c| match c{
            '.' => Field::None,
            '#' => Field::Bloc,
            'O' => Field::Move,
            _ => unreachable!(),
        
        }).collect()
    ).collect();

    let mut input2 = Array2::from_shape_vec((input.len(), input[0].len()), input.clone().into_iter().flatten().collect()).unwrap();
    println!("{:?}", input2);

    // let mut set = HashSet::new();
    // let mut hasher = DefaultHasher::new();

    // let timer = std::time::Instant::now();

    // for _ in 0..1000000000_usize{
        
            //fall north
            for col in 0..input[0].len() {
                let mut last_fall_on = 0;
                for row in 0..input.len() {
                    match input[row][col] {
                        Field::None => (),
                        Field::Bloc => last_fall_on = row+1,
                        Field::Move => {
                            input[row][col] = Field::None;
                            input[last_fall_on][col] = Field::Move;
                            last_fall_on += 1;
                        },
                    }
                }
            }

            // equivalent north with ndarray

            for mut col in input2.columns_mut() {
                let mut last_fall_on = 0;
                for (index, mut elem) in col.iter_mut().enumerate() {
                    match elem {
                        Field::None => (),
                        Field::Bloc => last_fall_on = index+1,
                        Field::Move => {
                            elem = &mut Field::None;
                            col[last_fall_on] = Field::Move;
                            last_fall_on += 1;
                        },
                    }
                }
            }
        
            // //fall west
            // for row in 0..input.len() {
            //     let mut last_fall_on = 0;
            //     for col in 0..input.len() {
            //         match input[row][col] {
            //             Field::None => (),
            //             Field::Bloc => last_fall_on = col+1,
            //             Field::Move => {
            //                 input[row][col] = Field::None;
            //                 input[row][last_fall_on] = Field::Move;
            //                 last_fall_on += 1;
            //             },
            //         }
            //     }
            // }
        
            // //fall south
            // for col in 0..input[0].len() {
            //     let mut last_fall_on = input.len()-1;
            //     for row in (0..input.len()).rev() {
            //         match input[row][col] {
            //             Field::None => (),
            //             Field::Bloc => last_fall_on = row.checked_sub(1).unwrap_or(0),
            //             Field::Move => {
            //                 input[row][col] = Field::None;
            //                 input[last_fall_on][col] = Field::Move;
            //                 if last_fall_on > 0 {
            //                     last_fall_on -= 1;
            //                 }
            //             },
            //         }
            //     }
            // }
        
            // //fall east
            // for row in 0..input.len() {
            //     let mut last_fall_on = input[0].len()-1;
            //     for col in (0..input[0].len()).rev() {
            //         match input[row][col] {
            //             Field::None => (),
            //             Field::Bloc => last_fall_on = col.checked_sub(1).unwrap_or(0),
            //             Field::Move => {
            //                 input[row][col] = Field::None;
            //                 input[row][last_fall_on] = Field::Move;
            //                 if last_fall_on > 0 {
            //                     last_fall_on -= 1;
            //                 }
            //             },
            //         }
            //     }
            // }

            // input.hash(&mut hasher);
            // let hash = hasher.finish();

            // if set.contains(&hash) {
            //     break;
            // } else {
            //     set.insert(hash);

            //     // if set.len() % 1_000_000 == 0 {
            //     //     println!("time: {:?}", timer.elapsed());
            //     //     println!("set len: {}", set.len());
            //     // }
            // }

    // }

    // println!("{:?}", pretty_print(&input));

    let height = input.len();

    input.iter().enumerate().map(|(row, line)|{
        println!("height-row: {}", height - row);
        line.iter().filter(|c| **c == Field::Move).count() * (height - row)
    }).sum()
    
}

fn main() {
    let input = include_str!("../../input/d14ex.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d14ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 136);
    }
}