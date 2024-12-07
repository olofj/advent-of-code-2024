fn combinations(values: &[usize]) -> Vec<usize> {
    if values.len() == 1 {
        vec![values[0]]
    } else {
        let first = values[0];
        let vals = combinations(&values[1..]);
        vals.into_iter()
            .flat_map(|v| vec![first * v, first + v])
            .collect::<Vec<_>>()
    }
}

fn day07a(infile: &str) -> usize {
    let input = infile
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(t, vals)| {
            (
                t.parse::<usize>().unwrap(),
                vals
                    .split_whitespace()
                    .rev()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let ret = input
        .into_iter()
        .map(|(t, v)| (t, combinations(&v)))
        .filter(|(t, v)| v.contains(t))
        .map(|(t, _)| t)
        .sum();

    ret
}

fn combinations2(values: &[usize]) -> Vec<usize> {
    if values.len() == 1 {
        vec![values[0]]
    } else {
        let first = values[0];
        let vals = combinations2(&values[1..]);
        vals.into_iter()
            .flat_map(|v| {
                vec![
                    first * v,
                    first + v,
                    format!("{}{}", v, first).parse::<usize>().unwrap(),
                ]
            })
            .collect::<Vec<_>>()
    }
}

fn day07b(infile: &str) -> usize {
    let input = infile
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(t, vals)| {
            (
                t.parse::<usize>().unwrap(),
                vals
                    .split_whitespace()
                    .rev()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let ret = input
        .into_iter()
        .map(|(t, v)| (t, combinations2(&v)))
        .filter(|(t, v)| v.contains(t))
        .map(|(t, _)| t)
        .sum();

    ret
}

fn main() {
    println!("a sample: {}", day07a(include_str!("sample-day07.txt")));
    println!("a input:  {}", day07a(include_str!("input-day07.txt")));
    println!("b sample: {}", day07b(include_str!("sample-day07.txt")));
    println!("b input:  {}", day07b(include_str!("input-day07.txt")));
}
