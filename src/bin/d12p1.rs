use itertools::Itertools;
use rayon::vec;

fn is_valid(s: &str, numbers: &Vec<usize>) -> bool {
    
    let dammaged = s
    .split('.')
    .map(|p| p.len())
    .filter(|&l| l > 0)
    .collect_vec();

    dammaged == *numbers
}

fn process(input: String) -> usize {

    input.lines().map(|l|{
        let line = l.split_whitespace().collect_vec();
        let springs = line[0];
        let numbers = line[1].split(',')
        .map(|n| {
            let n = n.parse::<usize>().unwrap();
            vec![n, n, n, n, n]
        }
        )
        .flatten()
        .collect_vec();

        let mut possibilities: Vec<String> = vec![String::new()];

        for c in springs.chars() {
            if c != '?' {
                possibilities.iter_mut().for_each(|p| p.push(c));
            } else {
                possibilities = possibilities.iter().map(|p| {
                    let mut p1 = p.clone();
                    let mut p2 = p.clone();
                    p1.push('.');
                    p2.push('#');
                    vec![p1, p2]
                }).flatten().collect_vec();
            }
        }

        possibilities.iter().filter(|p| is_valid(p, &numbers)).count()
        
    }).sum()
}

fn main() {
    let input = include_str!("../../input/d12.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d12ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 21);
    }
}