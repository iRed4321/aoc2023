use itertools::Itertools;

fn pretty_print(input: &Vec<Vec<char>>) {
    for line in input {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn process(input: String) -> usize {

    let mut input : Vec<Vec<_>> = input.lines().map(|line|
        line.chars().map(|c| c).collect()
    ).collect();

    for col in 0..input[0].len() {
        let mut last_fall_on = 0;
        for row in 0..input.len() {
            match input[row][col] {
                '.' => (),
                '#' => last_fall_on = row+1,
                'O' => {
                    input[row][col] = '.';
                    input[last_fall_on][col] = 'O';
                    last_fall_on += 1;
                },
                _ => unreachable!(),
            }
        }
    }

    println!("{:?}", pretty_print(&input));

    let height = input.len();

    input.iter().enumerate().map(|(row, line)|{
        println!("height-row: {}", height - row);
        line.iter().filter(|c| **c == 'O').count() * (height - row)
    }).sum()
    
}

fn main() {
    let input = include_str!("../../input/d14.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d14ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 136);
    }
}