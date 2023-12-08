
fn process(input: String) -> usize {

    let mut input = input.split("\n\n").map(|x| {
        x
        .lines()
        .filter(|x| !x.contains(":"))
        .map(|x| 
            x.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let seeds = input.remove(0).first().unwrap().to_owned();

    let input = input.iter().map(|x| {
        x.iter().map(|x|(x[1], x[0], x[2])).collect::<Vec<(usize, usize, usize)>>()
    }).collect::<Vec<_>>();

    let mut all_last: Vec<usize> = vec![];

    for seed in seeds.iter() {
        let mut last = seed.to_owned();
        for input in input.iter(){
            for (curr, next, length) in input.iter(){
                if last >= *curr && last <= *curr + *length {
                    last = next + (last - curr);
                    break;
                }
            }
        }
        all_last.push(last)
    }

    *all_last.iter().min().unwrap()

}

fn main() {
    let input = include_str!("../../input/d5.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d5ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 35);
    }
}