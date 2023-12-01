fn process(input: String) -> usize {

    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    input.lines().map(|line| {
        let (mut first, mut last) = (None, None);

        'outer: for i in 0..=line.len() {

            let (start, end) = (line.split_at(i).1 ,line.split_at(line.len() - i).0);
            
            for (key, value) in numbers.iter().enumerate() {
                let key = key+1;

                if first.is_none() && (start.starts_with(&key.to_string()) || start.starts_with(value)) {
                    first = Some(key);
                }
                if last.is_none() && (end.ends_with(&key.to_string()) || end.ends_with(value)) {
                    last = Some(key);
                }
                if first.is_some() && last.is_some() {
                    break 'outer;
                }
            }
        }

        first.unwrap() * 10 + last.unwrap()
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
        assert_eq!(res, 281);
    }
}