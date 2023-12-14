use itertools::Itertools;

fn match_lines(pattern: &str, i: usize, shift: usize) -> usize {
    let line1 = pattern.lines().nth(i - shift).unwrap();
    let line2 = pattern.lines().nth(i + 1 + shift).unwrap();
    line1.chars().zip(line2.chars()).filter(|(a, b)| a != b).count()
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn find_pattern(pattern: &str) -> Option<usize>{

    let mut fixed = false;

    for (i, _) in pattern.lines().enumerate(){

        for shift in 0.. {
        
            if i < shift || i + 1 + shift >= pattern.lines().count() {
                if fixed {
                    return Some(i);
                }
                break;
            }
            

            match match_lines(&pattern, i, shift) {
                0 => (),
                0 | 1 if !fixed => fixed = true,
                _ => {
                    fixed = false;
                    break;
                },
            }
        }

    }

    None

}

fn process(input: String) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .map(|(num, pattern)| {

            if let Some(found) = find_pattern(pattern) {
                return (found + 1)*100;
            }

            let pattern = transpose(
                &pattern
                    .lines()
                    .map(|l| l.chars().collect_vec())
                    .collect_vec(),
            ).iter().map(|l| l.iter().collect::<String>()).join("\n");

            if let Some(found) = find_pattern(&pattern) {
                return found + 1;
            }

            panic!("not found in {}", num);

        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/d13.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d13ex.txt");

        let res = process(input.to_owned());
        assert_eq!(res, 400);
    }
}
