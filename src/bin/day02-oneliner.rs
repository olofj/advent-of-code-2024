fn day02a(infile: &str) -> usize {
    infile
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|l| {
            l.as_slice()
                .windows(2)
                .map(|a| a[1] - a[0])
                .collect::<Vec<isize>>()
        })
        .filter(|l| l.iter().all(|&x| x <= 0) || l.iter().all(|&x| x >= 0))
        .filter(|l| l.iter().all(|&x| x.abs() > 0 && x.abs() < 4))
        .count()
}

fn day02b(infile: &str) -> usize {
    infile
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|l| {
            std::iter::once(l.clone())
                .chain((0..l.len()).map(|i| {
                    l.iter()
                        .enumerate()
                        .filter(|&(j, _)| j != i)
                        .map(|(_, v)| *v)
                        .collect::<Vec<isize>>()
                }))
                .collect::<Vec<_>>()
        })
        .map(|ll| {
            ll.iter()
                .map(|l| {
                    l.as_slice()
                        .windows(2)
                        .map(|a| a[1] - a[0])
                        .collect::<Vec<isize>>()
                })
                .filter(|l| l.iter().all(|&x| x <= 0) || l.iter().all(|&x| x >= 0))
                .filter(|l| l.iter().all(|&x| x.abs() > 0 && x.abs() < 4))
                .collect::<Vec<_>>()
        })
        .filter(|l| l.len() > 0)
        .count()
}

fn main() {
    println!("sample {}", day02a(include_str!("sample-day02.txt")));
    println!("input {}", day02a(include_str!("input-day02.txt")));
    println!("sample {}", day02b(include_str!("sample-day02.txt")));
    println!("input {}", day02b(include_str!("input-day02.txt")));
}
