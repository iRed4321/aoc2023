use std::collections::HashMap;

use itertools::Itertools;


#[derive(Debug, Clone, Eq, Ord)]
struct Game{
    hand: String,
    pub bid: usize
}

impl PartialEq for Game{
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Game{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let card_self: HashMap<char, usize> = self.hand.chars().sorted().group_by(|&c| c).into_iter().map(|(c, group)| (c, group.count())).collect();
        let card_other: HashMap<char, usize> = other.hand.chars().sorted().group_by(|&c| c).into_iter().map(|(c, group)| (c, group.count())).collect();

        println!("{:?} {:?}", self.hand, other.hand);

        //Five of a kind
        if card_self.values().any(|&v| v == 5) && card_other.values().any(|&v| v == 5){
            println!("A");
            return self.cmp_cards(&other).into();
        }

        if card_self.values().any(|&v| v == 5){
            println!("B");
            return Some(std::cmp::Ordering::Greater);
        }

        if card_other.values().any(|&v| v == 5){
            println!("C");
            return Some(std::cmp::Ordering::Less);
        }

        //Four of a kind
        if card_self.values().any(|&v| v == 4) && card_other.values().any(|&v| v == 4){
            println!("D");
            return self.cmp_cards(&other).into();
        }

        if card_self.values().any(|&v| v == 4){
            println!("E");
            return Some(std::cmp::Ordering::Greater);
        }

        if card_other.values().any(|&v| v == 4){
            println!("F");
            return Some(std::cmp::Ordering::Less);
        }

        //Full house

        if let Some(key3elems) = card_self.iter().find_map(|(k, v)| if *v == 3 { Some(k) } else { None }){
            if card_self.iter().any(|(k, v)| k != key3elems && *v == 2){
                if let Some(key3elemsother) = card_other.iter().find_map(|(k, v)| if *v == 3 { Some(k) } else { None }){
                    if card_other.iter().any(|(k, v)| k != key3elemsother && *v == 2){
                        println!("G");
                        return self.cmp_cards(&other).into();
                    }
                }
            }
        }

        if let Some(key3elems) = card_self.iter().find_map(|(k, v)| if *v == 3 { Some(k) } else { None }){
            if card_self.iter().any(|(k, v)| k != key3elems && *v == 2){
                println!("H");
                return Some(std::cmp::Ordering::Greater);
            }
        }

        if let Some(key3elems) = card_other.iter().find_map(|(k, v)| if *v == 3 { Some(k) } else { None }){
            if card_other.iter().any(|(k, v)| k != key3elems && *v == 2){
                println!("I");
                return Some(std::cmp::Ordering::Less);
            }
        }

        //Three of a kind
        if card_self.values().any(|&v| v == 3) && card_other.values().any(|&v| v == 3){
            println!("J");
            return self.cmp_cards(&other).into();
        }

        if card_self.values().any(|&v| v == 3){
            println!("K");
            return Some(std::cmp::Ordering::Greater);
        }

        if card_other.values().any(|&v| v == 3){
            println!("L");
            return Some(std::cmp::Ordering::Less);
        }

        //Two pair
        if card_self.values().filter(|&v| *v == 2).count() == 2 && card_other.values().filter(|&v| *v == 2).count() == 2{
            println!("M");
            return self.cmp_cards(&other).into();
        }

        if card_self.values().filter(|&v| *v == 2).count() == 2{
            println!("N");
            return Some(std::cmp::Ordering::Greater);
        }

        if card_other.values().filter(|&v| *v == 2).count() == 2{
            println!("O");
            return Some(std::cmp::Ordering::Less);
        }

        //One pair
        if card_self.values().any(|&v| v == 2) && card_other.values().any(|&v| v == 2){
            println!("P");
            return self.cmp_cards(&other).into();
        }

        if card_self.values().any(|&v| v == 2){
            println!("Q");
            return Some(std::cmp::Ordering::Greater);
        }

        if card_other.values().any(|&v| v == 2){
            println!("R");
            return Some(std::cmp::Ordering::Less);
        }

        //High card
        println!("S");
        self.cmp_cards(&other).into()

    }
}

impl Game{
    fn new(hand: &str, bid: &str) -> Self{

        let hand = hand.to_owned();
        let bid = bid.parse::<usize>().unwrap();

        Self{
            hand,
            bid
        }
    }

    fn cmp_cards(&self, other: &Self) -> std::cmp::Ordering{
        let cards_self: Vec<char> = self.hand.chars().map(|c|{
            match c{
                'T' => 'A',
                'J' => 'B',
                'Q' => 'C',
                'K' => 'D',
                'A' => 'E',
                _ => c
            }
        }).collect();
        let cards_other: Vec<char> = other.hand.chars().map(|c|{
            match c{
                'T' => 'A',
                'J' => 'B',
                'Q' => 'C',
                'K' => 'D',
                'A' => 'E',
                _ => c
            }
        }).collect();

        for i in 0..cards_self.len(){
            if cards_self[i] > cards_other[i]{
                println!("T");
                return std::cmp::Ordering::Greater
            }else if cards_self[i] < cards_other[i]{
                println!("U");
                return std::cmp::Ordering::Less
            }
        }

        println!("V");
        std::cmp::Ordering::Equal
    }
}

impl From<(&str, &str)> for Game{
    fn from((hand, bid): (&str, &str)) -> Self {
        Self::new(hand, bid)
    }
}

fn process(input: String) -> usize {

    input.lines().map(|line|{
        line.split_whitespace().collect_tuple::<(&str, &str)>().unwrap().into()
    }).sorted().enumerate().map(|(index, game): (usize, Game)|{
        game.bid * (index + 1)
    }).sum()
    
}

fn main() {
    let input = include_str!("../../input/d7.txt");
    println!("{}", process(input.to_owned()));
    // 248192278
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d7ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 6440);
    }
}