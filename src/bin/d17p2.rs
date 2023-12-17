use itertools::Itertools;
use ndarray::{Array2, ArrayView2};
use pathfinding::prelude::dijkstra;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction{
    North,
    West,
    South,
    East,
}

impl Direction {

    fn shift(&self, (posx, posy): (usize,usize)) -> Option<(usize, usize)> {
        match self {
            Direction::South => Some((posx, posy + 1)),
            Direction::East => Some((posx + 1, posy)),
            Direction::North if posy != 0 => Some((posx, posy - 1)),
            Direction::West if posx != 0 => Some((posx - 1, posy)),
            _ => None
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos{
    x: usize,
    y: usize,
    last_dir: Direction,
    times: usize
}

impl Pos{
    fn successors(&self, map: &ArrayView2<u8>) -> Vec<(Pos, usize)> {
        let &Pos{x, y, last_dir, times} = self;
        
        Direction::iter().filter_map(|dir|{
            if dir == last_dir.opposite(){
                None
            } else if dir == last_dir && times == 10{
                None
            } else if dir != last_dir && times < 4{
                None
            } else {
                let Some((new_pos_x, new_pos_y)) = dir.shift((x, y)) else {
                    return None;
                };
                if let Some(dir_weight) = map.get((new_pos_x, new_pos_y)){
                    Some((Pos{
                        x: new_pos_x,
                        y: new_pos_y,
                        last_dir: dir, 
                        times: if dir == last_dir { times + 1 } else { 1 }
                    }, *dir_weight as usize))
                } else {
                    None
                }
            }
        }).collect_vec()
    }

    fn is_final(&self, map: &ArrayView2<u8>) -> bool {
        let &Pos{x, y, times, ..} = self;
        x == map.shape()[0] - 1 && y == map.shape()[1] - 1 && times >= 4
    }

}

fn process(input: String) -> usize {

    let input = input
        .lines()
        .map(|l| l.chars().map(|digit| digit.to_digit(10).unwrap() as u8).collect_vec())
        .collect_vec();

    let nb_rows = input.len() as isize;
    let nb_cols = input[0].len() as isize;

    let origin_grid = Array2::from_shape_vec((nb_rows as usize, nb_cols as usize), input.into_iter().flatten().collect_vec()).unwrap();

    let result = dijkstra(
        &Pos{x: 0, y: 0, last_dir: Direction::South, times:0},
        |p| p.successors(&origin_grid.view()),
        |p| p.is_final(&origin_grid.view()));

    result.unwrap().1
}

fn main() {
    let input = include_str!("../../input/d17.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d17ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 94);
    }
}