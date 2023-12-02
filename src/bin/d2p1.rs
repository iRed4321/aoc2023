fn process(input: String) -> usize {

    input.lines().enumerate().filter_map(|(i, line)| {

        for hand in line.split(":").last().unwrap().split(";") {
            for cube in hand.split(",") {
                let (nb, color) = cube.trim().split_once(" ").unwrap();
                let nb = nb.parse::<usize>().unwrap();
                match color {
                    "red" => if nb > 12 { return None; },
                    "green" => if nb > 13 { return None; },
                    "blue" => if nb > 14 { return None; },
                    _ => panic!("unknown color: {}", color),
                }
            }
        }
        
        Some(i+1)
    })
    .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = include_str!("../../input/d2.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d2ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 8);
    }
}