use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator};
fn process(input: String) -> usize {

    let mut input = input.split("\n\n").map(|x| {
        x.lines()
        .filter(|x| !x.contains(":"))
        .map(|x| 
            x.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let seeds = input.remove(0).first().unwrap().chunks(2)
    .map(|x|(x[0], x[0] + x[1]))
    .collect::<Vec<_>>();

    let input: Vec<Vec<_>> = input.iter().map(|x| {
        x.iter().map(|x|(x[1], x[0], x[2])).collect()
    }).collect();

    seeds.par_iter().map(|seed| {

        (seed.0..seed.1).into_par_iter().map(|s| {
            let mut last = s.to_owned();

            for input in input.iter(){

                last = input.iter().find_map(|(curr, next, length)| {
                    if last >= *curr && last < curr.checked_add(*length).unwrap_or(u32::MAX) {
                        Some(next + (last - curr))
                    } else {
                        None
                    }
                }).unwrap_or(last);
            }
            
            last
        })

    }).flatten().min().unwrap() as usize


}

fn main() {
    let input = include_str!("../../input/d5 copy.txt");
    println!("{}", process(input.to_owned()));
    //not 593974860
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d5ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 46);
    }
}