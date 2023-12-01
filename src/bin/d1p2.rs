
fn process(input: String) -> i32 {

    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    input
        .lines()
        .map(|line| {

            let (mut first, mut last) = (None, None);

            for i in 1..line.len() {

                let line_cut = line.split_at(i);
                
                for (key, value) in numbers.iter().enumerate() {
                    let key = key+ 1;
                    let key_str = key.to_string();

                    if first.is_none() && (line_cut.0.starts_with(&key_str) || line_cut.0.starts_with(value)) {
                        first = Some(key);
                    }
                    if last.is_none() && (line_cut.1.ends_with(&key_str) || line_cut.1.ends_with(value)) {
                        last = Some(key);
                    }
                }

                if first.is_some() && last.is_some() {
                    break;
                }
            }

            dbg!(first);
            dbg!(last);

            format!("{}{}", last.unwrap(), first.unwrap()).to_owned().parse::<i32>().unwrap()
        })
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = include_str!("../../input/d1.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d1ex.txt");
    
        let res = process(input.to_owned());
        // assert_eq!(res, 281);
    }
}