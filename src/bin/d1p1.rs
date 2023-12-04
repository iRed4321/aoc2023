fn main() {
    let input = include_str!("../../input/d1.txt");

    let res = input
        .lines()
        .map(|line| {
            let only_digits = line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>();
            let first = only_digits.chars().next().unwrap();
            let last = only_digits.chars().last().unwrap();

            format!("{}{}", first, last).to_owned().parse::<i32>().unwrap()
        }
        )
        .sum::<i32>();

    println!("{}", res);
}
