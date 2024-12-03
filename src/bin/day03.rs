fn day03a(infile: &str) -> usize {
    let ret = infile
        .split_terminator("mul(")
        .filter_map(|s| s.split_once(","))
        .filter(|(_, s)| s.split_once(")") != None)
        .map(|(s1, s2)| (s1, s2.split(")").next().unwrap()))
        .map(|(s1, s2)| (s1.parse::<usize>(), s2.parse::<usize>()))
        .filter_map(|(s1, s2)| match (s1, s2) {
            (Ok(s1), Ok(s2)) => Some((s1, s2)),
            _ => None,
        })
        .filter(|&(s1, s2)| s1 > 0 && s1 <= 999 && s2 > 0 && s2 <= 999)
        .map(|(a, b)| a * b)
        .sum::<usize>();
    println!("{}", ret);
    ret
}

fn day03b(infile: &str) -> usize {
    let infile = "do()".to_owned() + infile;
    let pruned = infile
        .split("don't()")
        .flat_map(|s| s.split("do()").skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    day03a(pruned.join("").as_str())
}

fn main() {
    println!("day03a sample");
    day03a(include_str!("sample-day03.txt"));

    println!("day03a input");
    day03a(include_str!("input-day03.txt"));

    println!("day03b sample");
    day03b(include_str!("sample-day03b.txt"));

    println!("day03b input");
    day03b(include_str!("input-day03.txt"));
}
