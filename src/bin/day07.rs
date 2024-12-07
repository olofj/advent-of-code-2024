fn combinations(values: &[usize], part_b: bool) -> Vec<usize> {
    let first = values[0];
    if values.len() == 1 {
        vec![first]
    } else {
        combinations(&values[1..], part_b)
            .into_iter()
            .flat_map(|v| {
                if part_b {
                    vec![
                        first + v,
                        first * v,
                        format!("{}{}", v, first).parse::<usize>().unwrap(),
                    ]
                } else {
                    vec![first + v, first * v]
                }
            })
            .collect::<Vec<_>>()
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
