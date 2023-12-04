use std::collections::HashMap;

fn process(input: String) -> usize {

    input.lines().map(|line|{
        let (winning, having) = line.split(":").last().unwrap().split_once("|").unwrap();
        let winning = winning.trim().split_whitespace().collect::<Vec<&str>>();
        let having = having.trim().split_whitespace().collect::<Vec<&str>>();

        let count = winning.iter().filter(|&x| having.contains(x)).count();
        if count == 0 {
            0
        } else {
            let mut res = 1;
            for _ in 1..count {
                res *= 2;
            }
            res
        }
    }).sum()

}

fn main() {
    let input = include_str!("../../input/d4.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d4ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 13);
    }
}