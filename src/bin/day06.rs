use std::collections::HashSet;

const MOVES: &'static [(char, isize, isize)] =
    &[('^', -1, 0), ('>', 0, 1), ('v', 1, 0), ('<', 0, -1)];

fn day06a(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut pos = (0, 0);
    let mut dir = '.';
    let nlines = map.len();
    let nchars = map[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c != '#' && *c != '.' {
                pos = (i, j);
                dir = *c;
            }
        }
    }

    let mut moveidx = MOVES
        .iter()
        .enumerate()
        .filter_map(|(i, m)| if m.0 == dir { Some(i) } else { None })
        .next()
        .unwrap();

    loop {
        visited.insert(pos);
        let newpos = (
            pos.0.checked_add_signed(MOVES[moveidx].1).unwrap_or(nlines),
            pos.1.checked_add_signed(MOVES[moveidx].2).unwrap_or(nchars),
        );
        if newpos.0 == nlines || newpos.1 == nchars {
            break;
        }
        if map[newpos.0][newpos.1] == '#' {
            moveidx = (moveidx + 1) % MOVES.len();
            continue;
        }
        pos = newpos;
    }

    let ret = visited.len();
    println!("{}", ret);
    ret
}

// Determine if a specific map (with starting position and direction) results
// in a loop or not
// We can do that by keeping track of all turning locations (instead of all visited),
// since that's a lot less information to track. We might overshoot the first possible detection of
// a loop, but that's OK.
fn loops(map: Vec<Vec<char>>, pos: (usize, usize), moveidx: usize) -> bool {
    let mut pos = pos;
    let mut moveidx = moveidx;
    let mut turned: HashSet<(usize, usize, usize)> = HashSet::new();

    let nlines = map.len();
    let nchars = map[0].len();

    loop {
        let newpos = (
            pos.0.checked_add_signed(MOVES[moveidx].1).unwrap_or(nlines),
            pos.1.checked_add_signed(MOVES[moveidx].2).unwrap_or(nchars),
        );
        if newpos.0 == nlines || newpos.1 == nchars {
            // Walking off the map, we're done (no loop)
            break;
        }
        if map[newpos.0][newpos.1] == '#' {
            // Hit a wall, update direction and restart
            moveidx = (moveidx + 1) % MOVES.len();
            if turned.contains(&(pos.0, pos.1, moveidx)) {
                // We have looped if we're about to turn on a tile and direction seen before
                return true;
            }
            turned.insert((pos.0, pos.1, moveidx));
            continue;
        }
        pos = newpos;
    }

    false
}

fn day06b(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut pos = (0, 0);
    let mut dir = '.';
    let nlines = map.len();
    let nchars = map[0].len();

    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c != '#' && *c != '.' {
                pos = (i, j);
                dir = *c;
            }
        }
    }

    let mut moveidx = MOVES
        .iter()
        .enumerate()
        .filter_map(|(i, m)| if m.0 == dir { Some(i) } else { None })
        .next()
        .unwrap();

    let orig_pos = pos;
    let orig_moveidx = moveidx;

    // We need to know the default visited tiles, since it's the
    // candidates for new obstructions.
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        visited.insert(pos);
        let newpos = (
            pos.0.checked_add_signed(MOVES[moveidx].1).unwrap_or(nlines),
            pos.1.checked_add_signed(MOVES[moveidx].2).unwrap_or(nchars),
        );
        if newpos.0 == nlines || newpos.1 == nchars {
            break;
        }
        if map[newpos.0][newpos.1] == '#' {
            moveidx = (moveidx + 1) % MOVES.len();
            continue;
        }
        pos = newpos;
    }

    // Then iterate through each visited location and consider if
    // there's a loop created if an obstruction is added there.

    let mut ret = 0;
    for v in visited {
        let mut map = map.clone();
        map[v.0][v.1] = '#';
        if loops(map, orig_pos, orig_moveidx) {
            ret += 1;
        }
    }
    println!("{}", ret);
    ret
}

fn main() {
    println!("day06a sample");
    day06a(include_str!("sample-day06.txt"));

    println!("day06a input");
    day06a(include_str!("input-day06.txt"));

    println!("day06b sample");
    day06b(include_str!("sample-day06.txt"));

    println!("day06b input");
    day06b(include_str!("input-day06.txt"));
}
