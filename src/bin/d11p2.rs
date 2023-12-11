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
        if !line.contains(&'#') {
            new_area.push(line.iter().map(|_| 'o').collect_vec());
        } else {
            new_area.push(line);
        }
    }

    let new_area = transpose(&new_area);

    let mut area = Vec::new();

    for line in new_area.iter().cloned() {
        if !line.contains(&'#') {
            area.push(line.iter().map(|_| 'o').collect_vec());
        } else {
            area.push(line);
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

    display_area(&area);

    galaxies_pos.iter().combinations(2).map(|pair| {
        let (x1, y1) = pair[0];
        let (x2, y2) = pair[1];

        let dx = (x1 - x2).abs();
        let dy = (y1 - y2).abs();

        let mut count_x = 0;
        if x1 <= x2 {
            for x in *x1..*x2 {
                if area[0][x as usize] == 'o'{
                    count_x+=1;
                }
            }
        } else {
            for x in *x2..*x1 {
                if area[0][x as usize] == 'o'{
                    count_x+=1;
                }
            }
        }

        let mut count_y = 0;
        if y1 <= y2 {
            for y in *y1..*y2 {
                if area[y as usize][0] == 'o'{
                    count_y+=1;
                }
            }
        } else {
            for y in *y2..*y1 {
                if area[y as usize][0] == 'o'{
                    count_y+=1;
                }
            }
        }

        let res = dx + dy + ((count_x + count_y)*(1000000-1));

        if *y1 == 0 && *y2 == 4{
            println!("res: {}", res);
            println!("dx: {}", dx);
            println!("dy: {}", dy);
            println!("count_x: {}", count_x);
            println!("count_y: {}", count_y);
        }
        res
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
        assert_eq!(res, 8410);
    }
}