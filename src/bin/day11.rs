use std::collections::HashMap;

fn expand(stones: Vec<usize>) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();

    for s in stones.into_iter() {
        if s == 0 {
            ret.push(1);
            continue;
        }

        let digits = s.ilog10() + 1;
        let div = 10usize.pow(digits / 2);
        if digits % 2 == 0 {
            ret.push(s / div);
            ret.push(s % div);
        } else {
            ret.push(s * 2024);
        }
    }
    ret
}

fn day11a(infile: &str) -> usize {
    let mut stones = infile
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones = stones.iter().flat_map(|v| expand(vec![*v])).collect();
    }
    stones.len()
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
