use std::{collections::HashMap, cmp::Ordering};

use itertools::Itertools;


#[derive(Debug, Clone, Eq, Ord)]
struct Game{
    hand: String,
    rank: Rank,
    pub bid: usize
}

impl PartialEq for Game{
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Copy)]
enum Rank{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl PartialOrd for Game{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.rank.cmp(&other.rank){
            Ordering::Equal => self.cmp_cards(&other),
            x => x
        }.into()
    }
}

impl Game{
    fn new(hand: &str, bid: &str) -> Self{
        Self {
            hand: hand.to_owned(),
            rank: Game::get_rank(&hand.to_owned()),
            bid: bid.parse().unwrap()
        }
    }

    fn get_rank(hand: &String) -> Rank{

        let self_jokers = hand.chars().filter(|&c| c == 'J').count();

        let mut self_rank = Rank::HighCard;

        for char in hand.to_owned().chars(){
            if char == 'J'{
                continue;
            }

            let mut jokers = self_jokers;
            let new_self = hand.chars().map(|c| 
                match c {
                    'J' if jokers > 0 => {
                        jokers -= 1;
                        char
                    },
                    _ => c
                }
            ).collect::<String>();

            let curr_rank = Game::get_rank(&new_self);

            if curr_rank > self_rank{
                self_rank = curr_rank;
            }
        }

        let card: HashMap<char, usize> = hand.chars().sorted().group_by(|&c| c).into_iter().map(|(c, group)| (c, group.count())).collect();

        if card.values().any(|&v| v == 5){
            Rank::FiveOfAKind
        }else if card.values().any(|&v| v == 4){
            Rank::FourOfAKind
        }else if let Some(key3elems) = card.iter().find_map(|(k, v)| if *v == 3 { Some(k) } else { None }){
            if card.iter().any(|(k, v)| k != key3elems && *v == 2){
                Rank::FullHouse
            }else{
                Rank::ThreeOfAKind
            }
        }else if card.values().filter(|&v| *v == 2).count() == 2{
            Rank::TwoPair
        }else if card.values().any(|&v| v == 2){
            Rank::OnePair
        }else{
            Rank::HighCard
        }
    }

    fn cmp_cards(&self, other: &Self) -> std::cmp::Ordering{
        let cards_self: Vec<char> = self.hand.chars().map(|c|{
            match c{
                'T' => 'A',
                'J' => '1',
                'Q' => 'C',
                'K' => 'D',
                'A' => 'E',
                _ => c
            }
        }).collect();
        let cards_other: Vec<char> = other.hand.chars().map(|c|{
            match c{
                'T' => 'A',
                'J' => '1',
                'Q' => 'C',
                'K' => 'D',
                'A' => 'E',
                _ => c
            }
        }).collect();

        for i in 0..cards_self.len(){
            if cards_self[i] > cards_other[i]{
                return std::cmp::Ordering::Greater
            }else if cards_self[i] < cards_other[i]{
                return std::cmp::Ordering::Less
            }
        }

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
    }).sorted()
    .enumerate().map(|(index, game): (usize, Game)|{
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
        assert_eq!(res, 5905);
    }
}