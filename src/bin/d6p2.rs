use itertools::Itertools;


fn process(input: String) -> usize {

    let (race_time, race_dist) = input.lines().filter_map(|line|{
        line.split(":").last()?.replace(" ", "").parse::<usize>().ok()
    }).collect_tuple().unwrap();

    (0..=race_time).filter(|speed| {
        speed * (race_time - speed) > race_dist
    }).count()
    
}

fn main() {
    let input = include_str!("../../input/d6.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d6ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 71503);
    }
}