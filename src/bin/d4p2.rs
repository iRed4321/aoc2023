fn process(input: String) -> usize {

    let mut cards = input.lines()
    .map(|card|{
        let (winning, having) = card.split(":").last().unwrap().split_once("|").unwrap();
        let winning = winning.trim().split_whitespace().collect::<Vec<&str>>();
        let having = having.trim().split_whitespace().collect::<Vec<&str>>();

        (winning, having, 1)
    }).collect::<Vec<_>>();

    for i in 0..cards.len() {
        let (winning, having, currcount) = cards.get(i).unwrap();
        let countwin = winning.iter().filter(|&x| having.contains(x)).count();
        
        let currcount = currcount.to_owned();

        for j in i+1..=i+countwin {
            if let Some((_, _, countofnext)) = cards.get_mut(j){
                *countofnext += currcount;
            }
        }

    }
    
    cards.iter().map(|(_, _, count)| count).sum()
}

fn main() {
    let input = include_str!("../../input/d4.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d4ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 30);
    }
}