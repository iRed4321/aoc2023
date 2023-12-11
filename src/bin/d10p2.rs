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
        println!("Found path: {:?}", seen);
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

    // write to output.txt
    let mut file = std::fs::File::create("output.txt").unwrap();
    
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(v) = area.get(&(x, y)) {
                if *v {
                    file.write_all(b"#").unwrap();
                } else {
                    file.write_all(b".").unwrap();
                }
            } else {
                file.write_all(b".").unwrap();
            }
        }
        file.write_all(b"\n").unwrap();
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

    // write to output.txt
    let mut file = std::fs::File::create("output.txt").unwrap();
    
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if seen.contains(&(x, y)) {
                let pipe = area[y as usize][x as usize].clone().unwrap();
                match pipe {
                    Pipe::Horizontal => file.write_all(b" ").unwrap(),
                    Pipe::Vertical => file.write_all(b" ").unwrap(),
                    Pipe::TopLeft => file.write_all(b"\xE2\x97\xA4").unwrap(),
                    Pipe::TopRight => file.write_all(b"\xE2\x97\xA5").unwrap(),
                    Pipe::BottomRight => file.write_all(b"\xE2\x97\xA2").unwrap(),
                    Pipe::BottomLeft => file.write_all(b"\xE2\x97\xA3").unwrap(),
                    Pipe::Animal => file.write_all(b"S").unwrap(),
                }
            } else {
                file.write_all(b"\xE2\x96\xA0").unwrap();
            }
        }
        file.write_all(b"\n").unwrap();
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

    let mut left_right: HashMap<(i16, i16), bool> = HashMap::new();

    for (y, line) in (0i16..).zip(area.iter()) {

        let mut prev = false;

        for (x, _) in (0i16..).zip(line.iter()) {
            if seen.contains(&(x, y)) {
                left_right.insert((x, y), false);
                prev = true;
            } else {
                if prev {
                    left_right.insert((x, y), true);
                } else {
                    left_right.insert((x, y), false);
                }
            }
        }
    }


    let mut right_left: HashMap<(i16, i16), bool> = HashMap::new();

    for (y, line) in (0i16..).zip(area.iter()) {

        let mut prev = false;

        for (x, _) in (0i16..line.len() as i16).zip(line.iter()).rev() {
            if seen.contains(&(x, y)) {
                right_left.insert((x, y), false);
                prev = true;
            } else {
                if prev {
                    right_left.insert((x, y), true);
                } else {
                    right_left.insert((x, y), false);
                }
            }
        }
    }

    let mut top_bottom: HashMap<(i16, i16), bool> = HashMap::new();

    for (y, line) in (0i16..).zip(transpose(&area).iter()) {

        let mut prev = false;

        for (x, _) in (0i16..).zip(line.iter()) {
            if seen.contains(&(y, x)) {
                top_bottom.insert((x, y), false);
                prev = true;
            } else {
                if prev {
                    top_bottom.insert((x, y), true);
                } else {
                    top_bottom.insert((x, y), false);
                }
            }
        }
    }


    let mut bottom_top: HashMap<(i16, i16), bool> = HashMap::new();

    for (y, line) in (0i16..).zip(transpose(&area).iter()) {

        let mut prev = false;

        for (x, _) in (0i16..line.len() as i16).zip(line.iter()).rev() {
            if seen.contains(&(y, x)) {
                bottom_top.insert((x, y), false);
                prev = true;
            } else {
                if prev {
                    bottom_top.insert((x, y), true);
                } else {
                    bottom_top.insert((x, y), false);
                }
            }
        }
    }


    let intersecion: HashMap<(i16, i16), bool> = left_right.iter().map(|((x, y), v)| {
        if *right_left.get(&(*x, *y)).unwrap() 
        && *top_bottom.get(&(*y, *x)).unwrap()
        && *bottom_top.get(&(*y, *x)).unwrap() 
        && *v {
            ((*x, *y), true)
        } else {
            ((*x, *y), false)
        }
    }).collect::<HashMap<(i16, i16), bool>>();

    // display_area(&intersecion);

    display_seen(&seen, &area);

    intersecion.iter().filter(|(_, v)| **v).count()
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
        let input = include_str!("../../input/d10exb.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 10);
    }
}