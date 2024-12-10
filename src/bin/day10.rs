use std::collections::HashSet;

const MOVES: &'static [(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn moves(point: (usize, usize), map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
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

fn day10a(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map.iter()
        .enumerate()
        // Find starting points
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(j, c)| (c == &0).then_some((i, j)))
        })
        // For each starting point, navigate the map
        .map(|s| {
            let mut nines: HashSet<(usize, usize)> = HashSet::new();
            let mut queue: Vec<(usize, usize)> = vec![s];

            while let Some(point) = queue.pop() {
                let h = map[point.0][point.1];
                if h == 9 {
                    nines.insert(point);
                }
                for m in moves(point, &map) {
                    if map[m.0][m.1] == h + 1 {
                        queue.push(m);
                    }
                }
            }
            nines.len()
        })
        .sum()
}

fn day10b(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map.iter()
        .enumerate()
        // Find starting points
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(j, c)| (c == &0).then_some((i, j)))
        })
        // For each starting point, navigate the map
        .map(|s| {
            let mut nroutes = 0;
            let mut queue = vec![s];
            while let Some(point) = queue.pop() {
                let h = map[point.0][point.1];
                if h == 9 {
                    nroutes += 1;
                }
                for m in moves(point, &map) {
                    if map[m.0][m.1] == h + 1 {
                        queue.push(m);
                    }
                }
            }
            nroutes
        })
        .sum()
}

fn main() {
    println!("a sample: {}", day10a(include_str!("sample-day10.txt")));
    println!("a input:  {}", day10a(include_str!("input-day10.txt")));
    println!("b sample: {}", day10b(include_str!("sample-day10.txt")));
    println!("b input:  {}", day10b(include_str!("input-day10.txt")));
}
