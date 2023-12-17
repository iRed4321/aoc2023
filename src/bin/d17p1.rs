use itertools::Itertools;
use ndarray::{Array2, ArrayView2};
use pathfinding::{prelude::dijkstra, directed::dijkstra};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction{
    North,
    West,
    South,
    East,
}

impl Direction {
    fn get(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
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
    x: i32,
    y: i32,
    just_did: (Direction, u32),
}

impl Pos{
    fn successors(&self, map: &ArrayView2<u8>) -> Vec<(Pos, usize)> {
        let &Pos{x, y, just_did} = self;
        let times = just_did.1;
        let last_dir = just_did.0;
        
        //get up, right, down and left positions
        Direction::iter().filter(|dir|{
            if dir == &last_dir.opposite(){
                false
            } else if times == 3 && dir == &last_dir{
                false
            } else {
                let dir_shift = dir.get();
                let new_pos_x = x + dir_shift.0;
                let new_pos_y = y + dir_shift.1;
                map.shape()[0] as i32 > new_pos_x && new_pos_x >= 0 && map.shape()[1] as i32 > new_pos_y && new_pos_y >= 0
            }
        }).map(|dir| {
            let dir_shift = dir.get();
            let new_pos_x = x + dir_shift.0;
            let new_pos_y = y + dir_shift.1;
            let dir_weight = map[[new_pos_x as usize, new_pos_y as usize]];
            let new_pos = Pos{x: new_pos_x, y: new_pos_y, just_did: (dir, if dir == last_dir { times + 1 } else { 1 })};

            (new_pos, dir_weight as usize)
        }).collect_vec()
    }

    fn is_final(&self, map: &ArrayView2<u8>) -> bool {
        let &Pos{x, y, ..} = self;
        x == map.shape()[0] as i32 - 1 && y == map.shape()[1] as i32 - 1
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

    // let result = dijkstra(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
    let result = dijkstra(&Pos{x: 0, y: 0, just_did: (Direction::East, 0)}, |p| p.successors(&origin_grid.view()), |p| p.is_final(&origin_grid.view()));
    
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
        assert_eq!(res, 102);
    }
}