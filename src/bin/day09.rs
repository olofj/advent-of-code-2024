fn day09a(infile: &str) -> usize {
    let input = infile
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut disk: Vec<Option<usize>> = Vec::with_capacity(input.iter().sum());
    let mut wp = 0;
    let mut fnum = 0;
    let mut it = input.iter();
    while let Some(flen) = it.next() {
        for _ in wp..(wp + flen) {
            disk.push(Some(fnum));
        }
        wp += flen;
        fnum += 1;
        if let Some(space) = it.next() {
            for _ in wp..(wp + space) {
                disk.push(None);
            }
            wp += space;
        }
    }

    let mut rp = disk.len() - 1;
    let mut wp = 0;
    while wp < rp {
        if disk[rp] == None {
            rp -= 1;
            continue;
        }
        if disk[wp] == None {
            if disk[rp] != None {
                disk[wp] = disk[rp];
                disk[rp] = None;
            }
            rp -= 1;
        }
        wp += 1;
    }
    disk.iter()
        .enumerate()
        .map(|(i, f)| if f == &None { 0 } else { i * f.unwrap() })
        .sum()
}

struct File {
    number: usize,
    start: usize,
    len: usize,
}

struct Space {
    start: usize,
    len: usize,
}

fn day09b(infile: &str) -> usize {
    let input = infile
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    // Files: fileno, start, size
    let mut files: Vec<File> = Vec::new();
    // Space: start, size
    let mut space: Vec<Space> = Vec::new();

    let mut wp = 0;
    let mut fnum = 0;
    let mut iter = input.iter();
    while let Some(flen) = iter.next() {
        files.push(File {
            number: fnum,
            start: wp,
            len: *flen,
        });
        fnum += 1;
        wp += flen;
        if let Some(slen) = iter.next() {
            space.push(Space {
                start: wp,
                len: *slen,
            });
            wp += slen;
        }
    }

    // Defrag
    let mut rf = files.len() - 1;
    while rf > 0 {
        for rs in 0..space.len() {
            if space[rs].len >= files[rf].len && space[rs].start < files[rf].start {
                files[rf].start = space[rs].start;
                space[rs].start += files[rf].len;
                space[rs].len -= files[rf].len;
                break;
            }
        }
        rf -= 1;
    }

    files
        .iter()
        .map(|f| {
            (f.start..(f.start + f.len))
                .map(|i| i * f.number)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    println!("a sample: {}", day09a(include_str!("sample-day09.txt")));
    println!("a input:  {}", day09a(include_str!("input-day09.txt")));
    println!("b sample: {}", day09b(include_str!("sample-day09.txt")));
    println!("b input:  {}", day09b(include_str!("input-day09.txt")));
}
