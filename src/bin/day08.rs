use std::collections::HashSet;

fn day08a(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nlines = map.len() as isize;
    let nchars = map[0].len() as isize;

    let antennas: Vec<(char, (isize, isize))> = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(j, c)| (c != &'.').then_some((*c, (i as isize, j as isize))))
        })
        .collect();

    let frequencies: HashSet<char> = antennas.iter().map(move |(c, _)| *c).collect();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for f in frequencies {
        let mut ant: HashSet<(isize, isize)> = antennas
            .iter()
            .filter_map(|(c, p)| (c == &f).then_some(*p))
            .collect();
        ant.clone().iter().for_each(|&a| {
            ant.remove(&a);
            ant.iter().for_each(|&b| {
                let diff = (b.0 - a.0, b.1 - a.1);
                let aa = (a.0 - diff.0, a.1 - diff.1);
                let bb = (b.0 + diff.0, b.1 + diff.1);
                if aa.0 >= 0 && aa.0 < nlines && aa.1 >= 0 && aa.1 < nchars {
                    antinodes.insert(aa);
                }
                if bb.0 >= 0 && bb.0 < nlines && bb.1 >= 0 && bb.1 < nchars {
                    antinodes.insert(bb);
                }
            })
        })
    }

    antinodes.len()
}

fn day08b(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nlines = map.len() as isize;
    let nchars = map[0].len() as isize;

    let antennas: Vec<(char, (isize, isize))> = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(j, c)| (c != &'.').then_some((*c, (i as isize, j as isize))))
        })
        .collect();

    let frequencies: HashSet<char> = antennas.iter().map(move |(c, _)| *c).collect();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for f in frequencies {
        let mut ant: HashSet<(isize, isize)> = antennas
            .iter()
            .filter(|(c, _)| c == &f)
            .map(|(_, p)| *p)
            .collect();
        ant.clone().iter().for_each(|&a| {
            ant.remove(&a);
            ant.iter().for_each(|&b| {
                let diff = (b.0 - a.0, b.1 - a.1);
                let mut aa = a;
                let mut bb = b;
                while aa.0 >= 0 && aa.0 < nlines && aa.1 >= 0 && aa.1 < nchars {
                    antinodes.insert(aa);
                    aa = (aa.0 - diff.0, aa.1 - diff.1);
                }
                while bb.0 >= 0 && bb.0 < nlines && bb.1 >= 0 && bb.1 < nchars {
                    antinodes.insert(bb);
                    bb = (bb.0 + diff.0, bb.1 + diff.1);
                }
            })
        })
    }

    antinodes.len()
}

fn main() {
    println!("a sample: {}", day08a(include_str!("sample-day08.txt")));
    println!("a input:  {}", day08a(include_str!("input-day08.txt")));
    println!("b sample: {}", day08b(include_str!("sample-day08.txt")));
    println!("b input:  {}", day08b(include_str!("input-day08.txt")));
}
