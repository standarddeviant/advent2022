
pub fn run(fname: &str) {
    part1_simple(fname);
    part2_simple(fname);
}


fn part1_simple(fname: &str) {
    let sums = elf_sums(fname);
    println!("sums.len() = {}", sums.len());
    println!("sums.max() = {}", sums.iter().max().unwrap());
}

fn part2_simple(fname: &str) {
    let mut sums = elf_sums(fname);
    sums.sort();
    let max3: i32 = sums.iter().rev().take(3).sum();
    println!("max3 = {}", max3);
}

fn elf_sums(fname: &str) -> Vec<i32> {
    let contents = std::fs::read_to_string(fname)
        .expect("Should have been able to read the file");
    println!("file contents.len() = {}", contents.len());

    let mut out: Vec<i32> = vec![];
    for s1 in contents.split("\n\n") {
        let mut sum: i32 = 0;
        for s2 in s1.split("\n") {
            let i = s2.parse::<i32>();
            sum += {
                match i {
                    Ok(x) => x,
                    Err(_e) => 0
                }
            }
        }
        out.push(sum);
    }
    out
}
