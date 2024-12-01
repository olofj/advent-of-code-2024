use std::collections::HashMap;

fn day01a(infile: &str) -> usize {
    // Process into two lists of integers
    let (mut first, mut second): (Vec<usize>, Vec<usize>) = infile
        .lines()
        .map(|b| {
            b.split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|b| (b[0], b[1]))
        .unzip();

    // Sort them per instructions
    first.sort();
    second.sort();

    // Calculate delta for the zipped pairs and sum per instructions
    let sum: usize = first
        .into_iter()
        .zip(second.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("{}", sum);
    sum
}

fn day01b(infile: &str) -> usize {
    // Again, process into two lists of integers
    let (first, second): (Vec<usize>, Vec<usize>) = infile
        .lines()
        .map(|b| {
            b.split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|b| (b[0], b[1]))
        .unzip();

    // Build a hashmap of counts for each element in the second list
    let mut counts: HashMap<usize, usize> = HashMap::new();

    // Increment by the element value to avoid needing to multiply later
    for s in second {
        *counts.entry(s).or_insert(0) += s;
    }

    // Per instructions, sum up the counts for each element in the first list
    let sum: usize = first.iter().filter_map(|f| counts.get(f)).sum();

    println!("{}", sum);
    sum
}

fn main() {
    println!("day01a sample (should be 11)");
    day01a(include_str!("sample-day01.txt"));

    println!("day01a input");
    day01a(include_str!("input-day01.txt"));

    println!("day01b sample (should be 31)");
    day01b(include_str!("sample-day01.txt"));

    println!("day01b input");
    day01b(include_str!("input-day01.txt"));
}
