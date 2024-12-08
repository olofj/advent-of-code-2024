fn concat(a: usize, b: usize) -> usize {
    let mut aa = a * 10;
    let mut bb = b;
    while bb > 10 {
        bb /= 10;
        aa *= 10;
    }
    aa + b
}

// Permutate head <op> tail. Since we reverse the values when we
// initially created the vector of them, we can do it this way.
// Brute-force, this is certainly slower than trying to be smart, but
// with the input data it still runs in a few seconds.
fn combinations(values: &[usize], part_b: bool) -> Vec<usize> {
    let first = values[0];
    if values.len() == 1 {
        vec![first]
    } else {
        combinations(&values[1..], part_b)
            .into_iter()
            .fold(Vec::new(), |mut ret, v| {
                ret.push(first + v);
                ret.push(first * v);
                if part_b {
                    ret.push(concat(v, first));
                };
                ret
            })
    }
}

fn day07a(infile: &str) -> usize {
    infile
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(t, vals)| {
            (
                t.parse::<usize>().unwrap(),
                vals.split_whitespace()
                    .rev()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter_map(|(t, v)| combinations(&v, false).contains(&t).then_some(t))
        .sum()
}

fn day07b(infile: &str) -> usize {
    infile
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(t, vals)| {
            (
                t.parse::<usize>().unwrap(),
                vals.split_whitespace()
                    .rev()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter_map(|(t, v)| combinations(&v, true).contains(&t).then_some(t))
        .sum()
}

fn main() {
    println!("a sample: {}", day07a(include_str!("sample-day07.txt")));
    println!("a input:  {}", day07a(include_str!("input-day07.txt")));
    println!("b sample: {}", day07b(include_str!("sample-day07.txt")));
    println!("b input:  {}", day07b(include_str!("input-day07.txt")));
}
