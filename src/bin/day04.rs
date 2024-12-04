const MOVES: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn day04a(infile: &str) -> usize {
    let grid = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let nlines = grid.len();
    let nchars = grid[0].len();

    let start: Vec<(usize, usize)> = (0..nlines)
        .flat_map(|l| {
            (0..nchars)
                .map(|c| (l, c))
                .filter(|(l, c)| grid[*l][*c] == 'X')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let word = vec!['X', 'M', 'A', 'S'];

    let mut ret = 0;
    for (l, c) in start {
        for d in MOVES.iter() {
            let mut ll = l;
            let mut cc = c;
            for ch in word.iter() {
                if ll < nlines && cc < nchars && grid[ll][cc] == *ch {
                    ll = ll.checked_add_signed(d.0).unwrap_or(nlines);
                    cc = cc.checked_add_signed(d.1).unwrap_or(nchars);
                    if *ch == 'S' {
                        ret += 1;
                    }
                } else {
                    break;
                }
            }
        }
    }

    println!("found {}", ret);
    ret
}

const STENCILS: [[[char; 3]; 3]; 4] = [
    [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']],
    [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']],
    [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']],
    [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']],
];

fn day04b(infile: &str) -> usize {
    let grid = infile
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let nlines = grid.len();
    let nchars = grid[0].len();

    let mut ret = 0;
    for l in 0..nlines - 2 {
        for c in 0..nchars - 2 {
            for s in STENCILS.iter() {
                let mut hit = 0;
                for dl in 0..3 {
                    for dc in 0..3 {
                        if s[dl][dc] != '.' && grid[l + dl][c + dc] == s[dl][dc] {
                            hit += 1;
                        }
                    }
                }
                if hit == 5 {
                    ret += 1;
                }
            }
        }
    }

    println!("found {}", ret);
    ret
}

fn main() {
    println!("day04a sample");
    day04a(include_str!("sample-day04.txt"));

    println!("day04a input");
    day04a(include_str!("input-day04.txt"));

    println!("day04b sample");
    day04b(include_str!("sample-day04.txt"));

    println!("day04b input");
    day04b(include_str!("input-day04.txt"));
}
