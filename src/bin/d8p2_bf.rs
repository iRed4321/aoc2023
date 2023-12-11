use std::{hash::Hash, collections::HashMap};

use itertools::Itertools;


fn process(input: String) -> usize {

    let (path, map) = input.split("\n\n").collect_tuple().unwrap();

    let map = map.lines().map(|x| {
        let key = x.split_once(" = ").unwrap();
        let (left, right) = key.1.split_once(", ").unwrap();
        let left = left[1..].to_owned();
        let right = right[..3].to_owned();
        (key.0.to_owned(), (left, right))
    }).collect::<HashMap<_, _>>();

    let mut count = 0;
    let mut curr = map.keys().cloned().filter(|x| x.ends_with("A"));

    for (step, direction) in path.chars().cycle().enumerate() {

        if curr.all(|x| x.ends_with("Z")) {
            count = step;
            break;
        }

        curr.map(|x| match direction {
            'L' => map[&x].0.as_str(),
            'R' => map[&x].1.as_str(),
            _ => panic!("Impossible!")
        }).filter(|x| true);
    }

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
        assert_eq!(res, 6);
    }
}