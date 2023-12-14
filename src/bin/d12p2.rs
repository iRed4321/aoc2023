use itertools::Itertools;

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
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

        let mut numbers = vec![numbers.clone(), numbers.clone(), numbers.clone(), numbers.clone(), numbers].into_iter().flatten().collect_vec();
        let mut springs = vec![springs, "?", springs, "?", springs, "?", springs, "?", springs].into_iter().collect::<String>();
        
        // if springs.starts_with("????."){
            
        //     springs.remove(0);
        //     springs.remove(0);
        //     springs.remove(0);
        //     springs.remove(0);
        //     *numbers.first_mut().unwrap() = 1;
        // }

        println!("springs: {}", springs);
        println!("numbers: {:?}", numbers);
        
        let should_total_springs = numbers.iter().sum::<usize>();
        let curr_total_springs = springs.chars().filter(|&c| c == '#').count();

        println!("should: {}", should_total_springs);
        println!("current: {}", curr_total_springs);



        
        let to_change = springs.chars()
        .enumerate()
        .filter(|(_,c)| *c == '?')
        .collect_vec();

        let mut possibilities: Vec<String> = to_change.iter()
        .combinations(should_total_springs - curr_total_springs)
        .filter(|comb| {
            let mut valid = true;
            for (i, c) in springs.chars().enumerate() {
                if comb.iter().any(|&x| x.0 == i) && c == '#' {
                    valid = false;
                    break;
                }
            }
            valid
        })
        .map(|comb| {
            let mut new_spring = String::new();
            for (i, c) in springs.chars().enumerate() {
                if comb.iter().any(|&x| x.0 == i) {
                    new_spring.push('#')
                } else {
                    new_spring.push(match c {
                        '?' => '.',
                        c => c
                    });
                }
            }
            new_spring
        })
        .collect_vec();


        println!("possibilities: {:?}", possibilities.len());

        let pos = possibilities.iter().filter(|p| is_valid(p, &numbers)).count();

        println!("finished: {}\n", pos);
        
        pos

    }).sum()
}

fn main() {
    let input = include_str!("../../input/d12ex.txt");
    println("Hello world !");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d12ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 525152);
    }
}