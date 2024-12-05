use std::cmp::Ordering;
use std::collections::HashMap;

fn day05a(infile: &str) -> usize {
    let (ordering, runs) = infile.split_once("\n\n").unwrap();

    let map = ordering
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .fold(HashMap::new(), |mut map, (a, b)| {
            map.entry(a).or_insert(Vec::new()).push(b);
            map
        });

    let valid = runs
        .lines()
        .map(|l| {
            l.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|run| {
            let mut seen: Vec<usize> = Vec::new();
            run.into_iter().all(|r| {
                seen.push(*r);
                map.get(r)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .all(|cmp| !seen.contains(cmp))
            })
        })
        .collect::<Vec<_>>();

    let ret = valid.iter().map(|l| l[l.len() / 2]).sum();
    println!("{}", ret);
    ret
}

fn day05b(infile: &str) -> usize {
    let (ordering, runs) = infile.split_once("\n\n").unwrap();

    let map = ordering
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .fold(HashMap::new(), |mut map, (a, b)| {
            map.entry(a).or_insert(Vec::new()).push(b);
            map
        });

    let fixed = runs
        .lines()
        .map(|l| {
            l.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|run| {
            let mut seen: Vec<usize> = Vec::new();
            run.into_iter().any(|r| {
                seen.push(*r);
                map.get(r)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|cmp| seen.contains(cmp))
            })
        })
        .map(|run| {
            let mut run = run.clone();
            run.sort_by(|a, b| {
                if map.get(a).unwrap_or(&Vec::new()).contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            run
        })
        .collect::<Vec<_>>();

    let ret = fixed.iter().map(|l| l[l.len() / 2]).sum();
    println!("{}", ret);
    ret
}

fn main() {
    println!("day05a sample");
    day05a(include_str!("sample-day05.txt"));

    println!("day05a input");
    day05a(include_str!("input-day05.txt"));

    println!("day05b sample");
    day05b(include_str!("sample-day05.txt"));

    println!("day05b input");
    day05b(include_str!("input-day05.txt"));
}
