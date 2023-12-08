use itertools::Itertools;


fn process(input: String) -> usize {

    let (races_times, races_dist) = input.lines().map(|line|{
        line.split(":").last().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<usize>>()
    }).collect_tuple().unwrap();

    races_times.iter().enumerate().map(|(index, time)|{
        let dist = races_dist[index];

        (0..=*time).filter(|speed| {
            speed * (time - speed) > dist
        }).count()

    }).product()
    
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
        assert_eq!(res, 288);
    }
}