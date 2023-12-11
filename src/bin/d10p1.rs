use std::{hash::Hash, collections::HashMap, thread::AccessError, error::Error};

#[derive(Debug, Clone, PartialEq, Eq)]
enum RelPos {
    Above,
    AtRight,
    Below,
    AtLeft,
    Same,
}

impl RelPos {
    fn invert(&self) -> Self {
        match self {
            RelPos::Above => RelPos::Below,
            RelPos::AtRight => RelPos::AtLeft,
            RelPos::Below => RelPos::Above,
            RelPos::AtLeft => RelPos::AtRight,
            RelPos::Same => RelPos::Same,
        }
    }
}

impl From<RelPos> for (i16, i16) {
    fn from(pos: RelPos) -> Self {
        match pos {
            RelPos::Above => (0, -1),
            RelPos::AtRight => (1, 0),
            RelPos::Below => (0, 1),
            RelPos::AtLeft => (-1, 0),
            RelPos::Same => (0, 0),
        }
    }
}

#[derive(Debug, Clone)]
enum Pipe{
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Animal
}

impl Pipe {
    fn can_connect(&self) -> Vec<RelPos> {
        match self {
            Pipe::Horizontal => vec![RelPos::AtLeft, RelPos::AtRight],
            Pipe::Vertical => vec![RelPos::Above, RelPos::Below],
            Pipe::TopLeft => vec![RelPos::Below, RelPos::AtRight],
            Pipe::TopRight => vec![RelPos::Below, RelPos::AtLeft],
            Pipe::BottomRight => vec![RelPos::Above, RelPos::AtLeft],
            Pipe::BottomLeft => vec![RelPos::Above, RelPos::AtRight],
            Pipe::Animal => vec![RelPos::Above, RelPos::AtRight, RelPos::Below, RelPos::AtLeft],
        }
    }
}

struct AroundIterator<'a>{
    center: (i16, i16),
    center_obj: Pipe,
    area: &'a Vec<Vec<Option<Pipe>>>,
    current: RelPos,
}

impl <'a> AroundIterator<'a> {
    fn new(center: (i16, i16), area: &'a Vec<Vec<Option<Pipe>>>) -> Self {
        Self {
            center,
            center_obj: area[center.1 as usize][center.0 as usize].clone().unwrap(),
            area,
            current: RelPos::Same,
        }
    }
}

impl Iterator for AroundIterator<'_> {
    type Item = (i16, i16);

    fn next(&mut self) -> Option<Self::Item> {

        loop {

            self.current = match self.current {
                RelPos::Same => RelPos::Above,
                RelPos::Above => RelPos::AtRight,
                RelPos::AtRight => RelPos::Below,
                RelPos::Below => RelPos::AtLeft,
                RelPos::AtLeft => return None,
                _ => panic!("Unknown state: {:?}", self.current)
            };

            let (x, y) = self.current.clone().into();
            let (x, y) = (self.center.0 + x, self.center.1 + y);

            if x < 0 || y < 0 || x >= self.area[0].len() as i16 || y >= self.area.len() as i16 {
                continue;
            }

            if let Some(new_obj) = &self.area[y as usize][x as usize]{

                if self.center_obj.can_connect().contains(&self.current)
                && new_obj.can_connect().contains(&self.current.invert()) {
                    return Some((x, y));
                }
            }


        }


    }
}


impl TryFrom<char> for Pipe {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c == '.' {
            return Err("Cannot convert '.' to Pipe");
        } else {

            Ok(match c {
                        '-' => Pipe::Horizontal,
                        '|' => Pipe::Vertical,
                        'F' => Pipe::TopLeft,
                        '7' => Pipe::TopRight,
                        'J' => Pipe::BottomRight,
                        'L' => Pipe::BottomLeft,
                        'S' => Pipe::Animal,
                        _ => unreachable!()
                    })
        }
       
    }
}

fn rec_find_path(area: &Vec<Vec<Option<Pipe>>>, seen: &mut Vec<(i16,i16)>, current: (i16, i16), max: usize) {

    let around = AroundIterator::new(current, area).filter(|(x, y)|
        !seen.last().unwrap().eq(&(*x, *y))
    ).collect::<Vec<_>>();

    if around.contains(&seen[0]) {
        return;
    }

    seen.push(current);

    for next in around {
        rec_find_path(area, seen, next, max+1);
    }

}

fn process(input: String) -> usize {

    let area : Vec<Vec<Option<Pipe>>> = input.lines().map(|line| line.chars().map(|c| c.try_into().ok()).collect()).collect();

    let animal = area.iter().enumerate().find_map(|(y, line)| {
        line.iter().enumerate().find_map(|(x, obj)| {
            if let Some(Pipe::Animal) = obj {
                Some((x as i16, y as i16))
            } else {
                None
            }
        })
    }).unwrap();
    
    let around = AroundIterator::new(animal, &area).collect::<Vec<_>>();

    println!("around: {:?}", around);

    let mut seen = vec![animal];
    rec_find_path(&area, &mut seen, *around.first().unwrap(), 1);


    (seen.len()+1)/2
}

fn main() {
    let input = include_str!("../../input/d10.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d10exa.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 8);
    }
}