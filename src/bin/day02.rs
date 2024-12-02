fn is_safe(l: &Vec<isize>) -> bool {
    // windows(2) makes it easy to pairwise compare all entries
    let delta = l
        .as_slice()
        .windows(2)
        .map(|a| a[1] - a[0])
        .collect::<Vec<isize>>();

    // Then just check if they're all either negative or positive
    let monotonic = delta.iter().all(|&x| x <= 0) || delta.iter().all(|&x| x >= 0);

    // And make sure the delta is 1, 2 or 3
    let limits = delta.iter().all(|&x| x.abs() > 0 && x.abs() < 4);

    // If both of those are true, it's a safe report
    monotonic && limits
}

fn is_safe_dampened(l: &Vec<isize>) -> bool {
    // This isn't pretty, but brute-force is fast enough -- if the
    // string isn't safe by itself, iterate over each entry and see
    // if the result is safe if it's removed.

    is_safe(l)
        || (0..l.len()).into_iter().any(|i| {
            let mut ll = l.clone();
            ll.remove(i);
            is_safe(&ll)
        })
}

fn day02a(infile: &str) -> usize {
    let ret = infile
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|l| is_safe(l))
        .count();

    println!("{}", ret);
    ret
}

fn day02b(infile: &str) -> usize {
    let ret = infile
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|l| is_safe_dampened(l))
        .count();

    println!("{}", ret);
    ret
}

fn main() {
    println!("day02a sample");
    day02a(include_str!("sample-day02.txt"));

    println!("day02a input");
    day02a(include_str!("input-day02.txt"));

    println!("day02b sample");
    day02b(include_str!("sample-day02.txt"));

    println!("day02b input");
    day02b(include_str!("input-day02.txt"));
}
