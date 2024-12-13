use std::collections::HashSet;

const MOVES: &'static [(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn moves(point: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let lmax = map.len() as isize;
    let cmax = map[0].len() as isize;
    let point: (isize, isize) = (point.0 as isize, point.1 as isize);
    MOVES
        .iter()
        .map(|m| (point.0 + m.0, point.1 + m.1))
        .filter(|n| n.0 >= 0 && n.0 < lmax && n.1 >= 0 && n.1 < cmax)
        .map(|(i, j)| (i as usize, j as usize))
        .collect()
}

fn day12a(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut ret = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let plant = map[i][j];
            let mut plot: HashSet<(usize, usize)> = HashSet::new();
            let mut queue: Vec<(usize, usize)> = Vec::new();
            let mut perimeter = 0;
            queue.push((i, j));
            while let Some(m) = queue.pop() {
                if visited.contains(&m) {
                    continue;
                }
                visited.insert(m);
                plot.insert(m);

                let check = moves(m, &map);
                perimeter += 4-check.len();
                for mm in check {
                    if map[mm.0][mm.1] == plant {
                        queue.push(mm);
                    } else {
                        perimeter += 1;
                    }
                }
            }
            ret += plot.len() * perimeter;
        }
    }

    ret
}

fn day12b(_infile: &str) -> usize {
    println!("(╯°□°)╯︵ ┻━┻");
    0
}

fn main() {
    println!("a sample: {}", day12a(include_str!("sample-day12.txt")));
    println!("a input:  {}", day12a(include_str!("input-day12.txt")));
    println!("b sample: {}", day12b(include_str!("sample-day12.txt")));
    println!("b input:  {}", day12b(include_str!("input-day12.txt")));
}
