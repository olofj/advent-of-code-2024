/*
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
 */
fn day13a(infile: &str) -> usize {
    let ret: isize = infile
        .split("\n\n")
        .filter_map(|b| {
            let nums: Vec<isize> = b
                .lines()
                .filter_map(|l| l.split_once(": ").unwrap().1.split_once(", "))
                .flat_map(|(a, b)| vec![a, b])
                .filter_map(|v| v.split(['+', '=']).nth(1))
                .filter_map(|v| v.parse::<isize>().ok())
                .collect();
            let a_0 = nums[0];
            let a_1 = nums[1];
            let b_0 = nums[2];
            let b_1 = nums[3];
            let tot_0 = nums[4];
            let tot_1 = nums[5];

            let pb = (tot_0 * a_1 - tot_1 * a_0) / (b_0 * a_1 - a_0 * b_1);
            let pa = (tot_0 - pb * b_0) / a_0;
            if pa * a_0 + pb * b_0 == tot_0 && pa * a_1 + pb * b_1 == tot_1 {
                Some(pa * 3 + pb)
            } else {
                None
            }
        })
        .sum();

    ret as usize
}

fn day13b(infile: &str) -> usize {
    let ret: isize = infile
        .split("\n\n")
        .filter_map(|b| {
            let nums: Vec<isize> = b
                .lines()
                .filter_map(|l| l.split_once(": ").unwrap().1.split_once(", "))
                .flat_map(|(a, b)| vec![a, b])
                .filter_map(|v| v.split(['+', '=']).nth(1))
                .filter_map(|v| v.parse::<isize>().ok())
                .collect();
            let a_0 = nums[0];
            let a_1 = nums[1];
            let b_0 = nums[2];
            let b_1 = nums[3];
            let tot_0 = 10000000000000 + nums[4];
            let tot_1 = 10000000000000 + nums[5];

            let pb = (tot_0 * a_1 - tot_1 * a_0) / (b_0 * a_1 - a_0 * b_1);
            let pa = (tot_0 - pb * b_0) / a_0;
            if pa * a_0 + pb * b_0 == tot_0 && pa * a_1 + pb * b_1 == tot_1 {
                Some(pa * 3 + pb)
            } else {
                None
            }
        })
        .sum();

    ret as usize
}

fn main() {
    println!("a sample: {}", day13a(include_str!("sample-day13.txt")));
    println!("a input:  {}", day13a(include_str!("input-day13.txt")));
    println!("b sample: {}", day13b(include_str!("sample-day13.txt")));
    println!("b input:  {}", day13b(include_str!("input-day13.txt")));
}
