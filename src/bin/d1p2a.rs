use std::collections::HashMap;

fn process(input: String) -> i32 {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    input
        .lines()
        .map(|line| {

            let mut first = 0;
            let mut last = 0;
            for i in 0..line.len() {

                let line_cut = line.split_at(i);
                
                for (key, value) in &numbers {
                    if first==0 && (line_cut.0.starts_with(key) || line_cut.0.starts_with(value.to_string().as_str())) {
                        first = *value;
                    }
                    if last==0 && line_cut.1.ends_with(key) || line_cut.1.ends_with(value.to_string().as_str()) {
                        last = *value;
                    }
                }
            }

            // for i in (0..line.len()).rev() {
                //         if line_cut_end.ends_with(key) || line_cut_end.ends_with(value.to_string().as_str()) {
                //             last = *value;
                //             break;
                //         }

            //     let line_cut_end = line.split_at(i).0;
                
            //     for (key, value) in &numbers {
            //     }
            // }

            format!("{}{}", last, first).to_owned().parse::<i32>().unwrap()
        })
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = include_str!("../../input/d1.txt");

    let res = process(input.to_owned());

    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = 
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
    
        assert_eq!(process(input.to_owned()), 281);
    }
}