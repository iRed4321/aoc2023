use itertools::Itertools;


fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn display_area(area: &Vec<Vec<char>>) {
    for line in area.iter() {
        println!("{}", line.iter().join(""));
    }
}

fn process(input: String) -> usize {

    let area : Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut new_area = Vec::new();

    for line in area.iter().cloned() {
        new_area.push(line.clone());

        if !line.contains(&'#') {
            new_area.push(line.clone())
        }
    }

    let new_area = transpose(&new_area);

    let mut area = Vec::new();

    for line in new_area.iter().cloned() {
        area.push(line.clone());

        if !line.contains(&'#') {
            area.push(line.clone())
        }
    }

    let area = transpose(&area);

    let galaxies_pos = area.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().filter_map(move |(x, obj)| {
            if *obj == '#' {
                Some((x as i64, y as i64))
            } else {
                None
            }
        })
    }).flatten().collect::<Vec<_>>();

    galaxies_pos.iter().combinations(2).map(|pair| {
        let (x1, y1) = pair[0];
        let (x2, y2) = pair[1];

        let dx = (x1 - x2).abs();
        let dy = (y1 - y2).abs();

        dx + dy
    }).sum::<i64>() as usize
}

fn main() {
    let input = include_str!("../../input/d11.txt");
    println!("{}", process(input.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let input = include_str!("../../input/d11ex.txt");
    
        let res = process(input.to_owned());
        assert_eq!(res, 374);
    }
}