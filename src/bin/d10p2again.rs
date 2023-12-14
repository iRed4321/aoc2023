use std::{collections::HashMap, io::Write};


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

#[derive(Debug, Clone, PartialEq, Eq)]
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

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
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

fn transpose_res(res: &HashMap<(i16, i16), bool>) -> HashMap<(i16, i16), bool> {
    let mut new_res = HashMap::new();

    for ((x, y), v) in res {
        new_res.insert((*y, *x), *v);
    }

    new_res
}

fn display_area(area: &HashMap<(i16, i16), bool>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in area.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(v) = area.get(&(x, y)) {
                if *v {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}


fn display_seen(seen: &Vec<(i16, i16)>, area: &Vec<Vec<Option<Pipe>>>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in seen {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if seen.contains(&(x, y)) {
                if let Some(v) = area[y as usize][x as usize].clone() {
                    match v {
                        Pipe::Horizontal => print!("-"),
                        Pipe::Vertical => print!("|"),
                        Pipe::TopLeft => print!("F"),
                        Pipe::TopRight => print!("7"),
                        Pipe::BottomRight => print!("J"),
                        Pipe::BottomLeft => print!("L"),
                        Pipe::Animal => print!("S"),
                    }
                } else {
                    print!(".");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

enum State {
    Outside,
    Wall,
    Inside
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

    display_seen(&seen, &area);

    let mut left_right: HashMap<(i16, i16), bool> = HashMap::new();

    for (y, line) in (0i16..).zip(area.iter()) {

        let mut prev = State::Outside;
        let mut last_in = false;

        for (x, _) in (0i16..).zip(line.iter()) {
            match (prev, seen.contains(&(x, y)), area[y as usize][x as usize].clone()) {
                (_,true,None) => panic!("Seen but no pipe"),
                (State::Outside | State::Wall, true, _) => {
                    prev = State::Wall;
                    left_right.insert((x, y), false);
                },
                (State::Wall, false, _) => {
                    if last_in {
                        left_right.insert((x, y), false);
                        last_in = false;
                        prev = State::Outside;
                    } else {
                        left_right.insert((x, y), true);
                        last_in = true;
                        prev = State::Inside;
                    }
                },
                (State::Inside, false, _) => {
                    left_right.insert((x, y), true);
                    last_in = true;
                },
                (State::Inside, true, _) => {
                    left_right.insert((x, y), false);
                    last_in = false;
                },
                _ => ()
            }
        }
    }

    println!();
    display_area(&left_right);

    // let mut up_down: HashMap<(i16, i16), bool> = HashMap::new();

    // for (y, line) in (0i16..).zip(transpose(&area).iter()) {

    //     let mut prev = false;

    //     for (x, _) in (0i16..).zip(line.iter()) {
    //         if seen.contains(&(y, x)) && area[x as usize][y as usize].clone().unwrap() == Pipe::Horizontal {
    //             prev = !prev;
    //         } else {
    //             if prev {
    //                 up_down.insert((y, x), true);
    //             } else {
    //                 up_down.insert((y, x), false);
    //             }
    //         }
    //     }
    // }

    // display_area(&up_down);



    0
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
        assert_eq!(res, 10);
    }
}