use std::collections::HashMap;

fn day11a(infile: &str) -> usize {
    let mut stones = infile
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..25 {
        stones = stones
            .iter()
            .flat_map(|v| {
                if *v == 0 {
                    vec![1]
                } else if format!("{}", v).len() % 2 == 0 {
                    let str = format!("{}", v);
                    let (s1, s2) = str.split_at(str.len() / 2);
                    vec![s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap()]
                } else {
                    vec![v * 2024]
                }
            })
            .collect();
    }
    stones.len()
}

fn expand(stones: Vec<usize>) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();

    for i in 0..stones.len() {
        if stones[i] == 0 {
            ret.push(1);
        } else if format!("{}", stones[i]).len() % 2 == 0 {
            let str = format!("{}", stones[i]);
            let (s1, s2) = str.split_at(str.len() / 2);
            ret.push(s1.parse::<usize>().unwrap());
            ret.push(s2.parse::<usize>().unwrap());
        } else {
            ret.push(stones[i] * 2024);
        }
    }
    ret
}

fn day11b(infile: &str) -> usize {
    let stones = infile
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut h: HashMap<usize, usize> = HashMap::new();
    for s in stones.into_iter() {
        h.entry(s).and_modify(|s| *s += 1).or_insert(1);
    }
    for _blink in 0..75 {
        for (k, v) in h.clone().into_iter() {
            // remove count for this entry
            h.entry(k).and_modify(|s| *s -= v);
            for s in expand(vec![k]) {
                h.entry(s).and_modify(|s| *s += v).or_insert(v);
            }
        }
    }
    let ret = h.iter().map(|(_, v)| v).sum::<usize>();
    ret
}

fn main() {
    println!("a sample: {}", day11a(include_str!("sample-day11.txt")));
    println!("a input:  {}", day11a(include_str!("input-day11.txt")));
    println!("b sample: {}", day11b(include_str!("sample-day11.txt")));
    println!("b input:  {}", day11b(include_str!("input-day11.txt")));
}
