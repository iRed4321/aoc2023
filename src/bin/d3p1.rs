use std::collections::HashMap;

fn process(input: String) -> usize {

    let tab = input.lines().enumerate().map(|(i, line)| {
        line.chars().enumerate().map(move |(j, c)| {
            ((i, j), c)
        })
    }).flatten().collect::<HashMap<(usize,usize), char>>();

    let mut nums : HashMap<(usize,usize), usize> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let mut curr_num = String::new();
        for (index, c) in line.chars().enumerate(){
            if c.is_digit(10) {
                curr_num.push(c);
            }
            if !curr_num.is_empty() && (!c.is_digit(10) || index == line.len() - 1) {
                nums.insert((i, index - curr_num.len()), curr_num.parse::<usize>().unwrap());
                curr_num = String::new();
            }
        }
    }

    nums.iter().filter_map(|((i,j), num)| {
        let mut res = None;
        'outer: for x in (*i as i32 - 1)..=(*i as i32 + 1) {
            for y in (*j as i32 - 1)..=(*j as i32 + num.to_string().len() as i32) {
                let Some(c) = tab.get(&(x as usize, y as usize)) else {continue;};
                if !c.is_digit(10) && c != &'.' {
                    res = Some(*num);
                    break 'outer;
                }
            }
        }
        res
    }).sum()

}

fn main() {
    let input = include_str!("../../input/d3.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d3ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 4361);
    }
}