use itertools::Itertools;

fn process(input: String) -> i32 {

    input.lines().map(|line|{
        let mut line = line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

        let mut versions = vec![line.clone()];

        while versions.last().unwrap().iter().any(|x| x != &0) {
            line = line.iter().tuple_windows().map(|(a, b)| b-a).collect();
            versions.push(line.clone());
        }

        let mut last = 0;
        for line in 
            versions.iter_mut()
            .rev(){

                last = line.first().unwrap() - last;
        }
        
        last
        
    }).sum()
    
}

fn main() {
    let input = include_str!("../../input/d9.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d9ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 2);
    }
}