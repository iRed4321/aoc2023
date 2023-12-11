use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

#[derive(Debug)]
struct StartingPattern {
    pub value: String,
    pub curr_pat: String,
    pub steps_done: usize,
    pub ends: HashMap<String, usize>,
}

fn process(input: String) -> usize {
    let (path, map) = input.split("\n\n").collect_tuple().unwrap();

    let map = map
        .lines()
        .map(|x| {
            let key = x.split_once(" = ").unwrap();
            let (left, right) = key.1.split_once(", ").unwrap();
            let left = left[1..].to_owned();
            let right = right[..3].to_owned();
            (key.0, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut count = 0;
    let mut starts = map
        .keys()
        .filter_map(|x| {
            if x.ends_with("A") {
                Some(StartingPattern {
                    value: x.to_string(),
                    curr_pat: x.to_string(),
                    steps_done: 0,
                    ends: HashMap::new(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    starts.iter_mut().for_each(|start| {
        for direction in path.chars().cycle() {
            if start.ends.get(&start.curr_pat).is_some(){
                break;
            }

            if start.curr_pat.ends_with("Z") {
                start.ends.insert(start.curr_pat.to_owned(), start.steps_done);
            }

            start.curr_pat = match direction {
                'L' => map[&start.curr_pat.as_str()].0.to_owned(),
                'R' => map[&start.curr_pat.as_str()].1.to_owned(),
                _ => panic!("Impossible!"),
            };

            start.steps_done += 1;
        }
    });

    println!("{:#?}", starts);

    count
}

fn main() {
    let input = include_str!("../../input/d8.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d8exb.txt");

        let res = process(input.to_owned());
        // assert_eq!(res, 6);
    }
}
