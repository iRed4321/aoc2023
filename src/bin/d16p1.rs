use std::{fmt::{Display, self}, collections::HashSet};

use itertools::Itertools;
use ndarray::Array2;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
    Splitted(Box<(Direction, Direction)>),
}

impl From<(Direction, Direction)> for Direction {
    fn from((a, b): (Direction, Direction)) -> Self {
        Self::Splitted(Box::new((a, b)))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Mirror {
    Backslash,
    Forwardslash,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Splitter {
    Vertical,
    Horizontal,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum TileObject {
    Mirror(Mirror),
    Splitter(Splitter),
}

impl Display for TileObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileObject::Mirror(Mirror::Backslash) => write!(f, "\\"),
            TileObject::Mirror(Mirror::Forwardslash) => write!(f, "/"),
            TileObject::Splitter(Splitter::Vertical) => write!(f, "|"),
            TileObject::Splitter(Splitter::Horizontal) => write!(f, "-"),
        }
    }
}

fn pretty_print(grid: &Array2<(Option<TileObject>, HashSet<Direction>)>) {
    for row in grid.outer_iter() {
        for tile in row {
            print!("{}", tile.0.as_ref().map(|t| t.to_string()).unwrap_or_else(|| ".".to_owned()));
        }
        println!();
    }
    println!();
    for row in grid.outer_iter() {
        for tile in row {
            print!("{}", if tile.1.is_empty() { "." } else { "X" });
        }
        println!();
    }
    println!();
}

fn process(input: String) -> usize {

    let input = input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut grid = Array2::from_shape_vec((input.len(), input[0].len()), input.into_iter().flatten().map(|c| (match c {
        '\\' => Some(TileObject::Mirror(Mirror::Backslash)),
        '/' => Some(TileObject::Mirror(Mirror::Forwardslash)),
        '|' => Some(TileObject::Splitter(Splitter::Vertical)),
        '-' => Some(TileObject::Splitter(Splitter::Horizontal)),
        _ => None,
    }, HashSet::<Direction>::new())).collect_vec()).unwrap();

    let mut beams = vec![(Direction::Down, (0_isize, 3_isize))];

    let mut last_grid = grid.to_owned();

    let nb_rows = grid.nrows() as isize;
    let nb_cols = grid.ncols() as isize;

    loop {

        beams = beams.iter()
        .filter_map(|(dir, (y, x))| {
            let (y, x) = (*y, *x);

            if y < 0 || x < 0 || y >= nb_rows || x >= nb_cols {
                None
            } else if last_grid.get((y as usize, x as usize)).unwrap().1.contains(dir) {
                None
            } else {

                grid.get_mut((y as usize, x as usize)).unwrap().1.insert(dir.clone());

                let tile = last_grid.get((y as usize, x as usize)).unwrap();
    
                let dir: Direction = if let Some(tile) = &tile.0 {
    
                    match (tile, dir) {
                        (_, Direction::Splitted(_)) => panic!("Beam splitted twice"),
                        (TileObject::Mirror(Mirror::Backslash), Direction::Right) => Direction::Down,
                        (TileObject::Mirror(Mirror::Backslash), Direction::Down) => Direction::Right,
                        (TileObject::Mirror(Mirror::Backslash), Direction::Left) => Direction::Up,
                        (TileObject::Mirror(Mirror::Backslash), Direction::Up) => Direction::Left,
                        (TileObject::Mirror(Mirror::Forwardslash), Direction::Right) => Direction::Up,
                        (TileObject::Mirror(Mirror::Forwardslash), Direction::Down) => Direction::Left,
                        (TileObject::Mirror(Mirror::Forwardslash), Direction::Left) => Direction::Down,
                        (TileObject::Mirror(Mirror::Forwardslash), Direction::Up) => Direction::Right,
                        (TileObject::Splitter(Splitter::Vertical), Direction::Right) => (Direction::Down, Direction::Up).into(),
                        (TileObject::Splitter(Splitter::Vertical), Direction::Down) => Direction::Down,
                        (TileObject::Splitter(Splitter::Vertical), Direction::Left) => (Direction::Up, Direction::Down).into(),
                        (TileObject::Splitter(Splitter::Vertical), Direction::Up) => Direction::Up,
                        (TileObject::Splitter(Splitter::Horizontal), Direction::Right) => Direction::Right,
                        (TileObject::Splitter(Splitter::Horizontal), Direction::Down) => (Direction::Left, Direction::Right).into(),
                        (TileObject::Splitter(Splitter::Horizontal), Direction::Left) => Direction::Left,
                        (TileObject::Splitter(Splitter::Horizontal), Direction::Up) => (Direction::Right, Direction::Left).into(),
                    }
                } else {
                    dir.to_owned()
                };

                Some(match dir {
                    Direction::Right => vec![(dir, (y, x + 1))],
                    Direction::Down => vec![(dir, (y + 1, x))],
                    Direction::Left => vec![(dir, (y, x - 1))],
                    Direction::Up => vec![(dir, (y - 1, x))],
                    Direction::Splitted(dirs) => {
                        vec![
                            (match dirs.0 {
                                Direction::Right => (dirs.0, (y, x + 1)),
                                Direction::Down => (dirs.0, (y + 1, x)),
                                Direction::Left => (dirs.0, (y, x - 1)),
                                Direction::Up => (dirs.0, (y - 1, x)),
                                Direction::Splitted(_) => panic!("Beam splitted twice"),
                            }),
                            (match dirs.1 {
                                Direction::Right => (dirs.1, (y, x + 1)),
                                Direction::Down => (dirs.1, (y + 1, x)),
                                Direction::Left => (dirs.1, (y, x - 1)),
                                Direction::Up => (dirs.1, (y - 1, x)),
                                Direction::Splitted(_) => panic!("Beam splitted twice"),
                            }),
                        ]
                    }
                })

            }

        })
        .flatten() 
        .collect_vec();

        if beams.is_empty() {
            break;
        }

        last_grid = grid.to_owned();

    }

    pretty_print(&grid);

    grid.iter().filter(|(_, beamspassed)| !beamspassed.is_empty()).count()
    
}

fn main() {
    let input = include_str!("../../input/d16.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d16ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 46);
    }
}