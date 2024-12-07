use std::collections::HashSet;

const MOVES: &'static [(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn visited(map: &Vec<Vec<char>>, pos: (usize, usize), moveidx: usize) -> HashSet<(usize, usize)> {
    let mut pos = pos;
    let mut moveidx = moveidx;

    let nlines = map.len();
    let nchars = map[0].len();

    let mut ret: HashSet<(usize, usize)> = HashSet::new();

    loop {
        ret.insert(pos);
        let newpos = (
            pos.0.checked_add_signed(MOVES[moveidx].0).unwrap_or(nlines),
            pos.1.checked_add_signed(MOVES[moveidx].1).unwrap_or(nchars),
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
    ret
}

fn day06a(infile: &str) -> usize {
    let map = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let moveidx = 0;

    let pos = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| l.iter().enumerate().map(move |(j, c)| (i, j, c)))
        .find(|(_, _, &c)| c != '#' && c != '.')
        .map(|(i,j,_)| (i,j))
        .unwrap();

    let path = visited(&map, pos, moveidx);

    let ret = path.len();
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
    let mut turns: HashSet<(usize, usize, usize)> = HashSet::new();

    let nlines = map.len();
    let nchars = map[0].len();

    loop {
        let newpos = (
            pos.0.checked_add_signed(MOVES[moveidx].0).unwrap_or(nlines),
            pos.1.checked_add_signed(MOVES[moveidx].1).unwrap_or(nchars),
        );
        if newpos.0 == nlines || newpos.1 == nchars {
            // Walking off the map, we're done (no loop)
            break;
        }
        if map[newpos.0][newpos.1] == '#' {
            // Hit a wall, update direction and restart
            moveidx = (moveidx + 1) % MOVES.len();
            if turns.contains(&(pos.0, pos.1, moveidx)) {
                // We have looped if we're about to turn on a tile and direction seen before
                return true;
            }
            turns.insert((pos.0, pos.1, moveidx));
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

    let pos = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| l.iter().enumerate().map(move |(j, c)| (i, j, c)))
        .find(|(_, _, &c)| c != '#' && c != '.')
        .map(|(i,j,_)| (i,j))
        .unwrap();

    let moveidx = 0;

    // We need to know the default visited tiles, since it's the
    // candidates for new obstructions.
    let path = visited(&map, pos, moveidx);

    // Then iterate through each visited location and consider if
    // there's a loop created if an obstruction is added there.
    let mut ret = 0;
    for v in path {
        let mut map = map.clone();
        map[v.0][v.1] = '#';
        if loops(map, pos, moveidx) {
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
