fn day03a(infile: &str) -> usize {
    let ret = infile
        .split_terminator("mul(")
        .filter_map(|s| s.split_once(","))
        .filter(|(_, s)| s.split_once(")") != None)
        .map(|(s1, s2)| (s1, s2.split(")").next().unwrap()))
        .map(|(s1, s2)| (s1.parse::<usize>(), s2.parse::<usize>()))
        .filter_map(|(s1, s2)| {
            if s1.is_ok() && s2.is_ok() {
                Some((s1.unwrap(), s2.unwrap()))
            } else {
                None
            }
        })
        .filter(|&(s1, s2)| s1 > 0 && s1 <= 999 && s2 > 0 && s2 <= 999)
        .map(|(a, b)| a * b)
        .sum::<usize>();
    println!("{}", ret);
    ret
}

fn day03b(infile: &str) -> usize {
    let infile = "do()".to_owned() + infile;
    let input = infile
        .split("don't()")
        .map(|s| s.split("do()").skip(1).collect::<Vec<_>>().join(""))
        .collect::<String>();
    let ret = input
        .split_terminator("mul(")
        .filter_map(|s| s.split_once(","))
        .filter(|(_, s)| s.split_once(")") != None)
        .map(|(s1, s2)| (s1, s2.split(")").next().unwrap()))
        .map(|(s1, s2)| (s1.parse::<usize>(), s2.parse::<usize>()))
        .filter_map(|(s1, s2)| {
            if s1.is_ok() && s2.is_ok() {
                Some((s1.unwrap(), s2.unwrap()))
            } else {
                None
            }
        })
        .filter(|&(s1, s2)| s1 > 0 && s1 <= 999 && s2 > 0 && s2 <= 999)
        .map(|(a, b)| a * b)
        .sum::<usize>();
    println!("{}", ret);
    ret
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
