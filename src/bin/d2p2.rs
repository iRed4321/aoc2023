fn process(input: String) -> usize {

    input.lines().map(|line| {

        let (mut red, mut green, mut blue) = (0, 0, 0);
        
        for hand in line.split(":").last().unwrap().split(";") {
            for cube in hand.split(",") {
                let (nb, color) = cube.trim().split_once(" ").unwrap();
                let nb = nb.parse::<usize>().unwrap();
                match color {
                    "red" => if nb > red { red = nb },
                    "green" => if nb > green { green = nb },
                    "blue" => if nb > blue { blue = nb },
                    _ => panic!("unknown color: {}", color),
                }
            }
        }
        
        red * green * blue
    })
    .sum()
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
        assert_eq!(res, 2286);
    }
}