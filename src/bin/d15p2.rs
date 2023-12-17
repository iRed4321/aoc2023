use std::collections::HashMap;

use itertools::Itertools;

fn get_hash(x: &str) -> usize {
    let mut curr = 0;

    for c in x.chars() {
        curr += c as usize;
        curr *= 17;
        curr = curr % 256;
    }

    curr
}


fn process(input: String) -> usize {

    let mut map : HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    input.trim().split(',').for_each(|x|{

        if let Some ((label, value)) = x.split_once('='){
            let hash = get_hash(label);
            let value = value.parse::<usize>().unwrap();

            map.get_mut(&hash).map(|v|{
                
                v.iter_mut().find(|(l, _)| l == label).map(|(_, v)| *v = value).unwrap_or_else(||{
                    v.push((label.to_owned(), value));
                });

            }).unwrap_or_else(||{
                map.insert(hash, vec![(label.to_owned(), value)]);
            });

        } else {
            let label = x[..x.len()-1].to_owned();
            let hash = get_hash(&label);

            map.get_mut(&hash).map(|v|{
                v.retain(|(l, _)| l != &label);
            }).unwrap_or_else(||());
        }

    });

    map.iter().map(|(box_nb, lenses)|{

        lenses.iter().enumerate().map(|(idx, (_, focal))|{

            (1+box_nb) * (idx+1) * focal

        }).sum::<usize>()

    }).sum()

}

fn main() {
    let input = include_str!("../../input/d15.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d15ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 145);
    }
}