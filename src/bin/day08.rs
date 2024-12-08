use std::collections::HashSet;

fn day08a(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nlines = map.len() as isize;
    let nchars = map[0].len() as isize;

    let mut antennas: Vec<(char, (isize, isize))> = Vec::new();
    for l in 0..map.len() {
        for c in 0..map[0].len() {
            if map[l][c] != '.' {
                antennas.push((map[l][c], (l as isize, c as isize)));
            }
        }
    }
    let frequencies: HashSet<char> = antennas.iter().map(|(c, _)| *c).collect::<HashSet<char>>();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for f in frequencies {
        let ant = antennas
            .iter()
            .filter(|(c, _)| c == &f)
            .map(|(_, p)| *p)
            .collect::<Vec<(isize, isize)>>();
        for a in ant.clone() {
            for b in ant.clone() {
                if a != b {
                    let diff = (b.0 - a.0, b.1 - a.1);
                    let e = (a.0 - diff.0, a.1 - diff.1);
                    let f = (b.0 + diff.0, b.1 + diff.1);
                    if e.0 >= 0 && e.0 < nlines && e.1 >= 0 && e.1 < nchars {
                        antinodes.insert(e);
                    }
                    if f.0 >= 0 && f.0 < nlines && f.1 >= 0 && f.1 < nchars {
                        antinodes.insert(f);
                    }
                }
            }
        }
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

    let antennas = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(j, c)| (c != &'.').then_some((*c, (i as isize, j as isize))))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let frequencies: HashSet<char> = antennas.iter().map(|(c, _)| *c).collect::<HashSet<char>>();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for f in frequencies {
        let ant = antennas
            .iter()
            .filter(|(c, _)| c == &f)
            .map(|(_, p)| *p)
            .collect::<Vec<(isize, isize)>>();
        for a in ant.clone() {
            for b in ant.clone() {
                if a != b {
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
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    println!("a sample: {}", day08a(include_str!("sample-day08.txt")));
    println!("a input:  {}", day08a(include_str!("input-day08.txt")));
    println!("b sample: {}", day08b(include_str!("sample-day08.txt")));
    println!("b input:  {}", day08b(include_str!("input-day08.txt")));
}
