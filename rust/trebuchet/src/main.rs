use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines("./calibration.txt") {
        for line in lines.flatten() {
            ans += trebuchet(line);
        }
    }

    println!("{}", ans);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn trebuchet(line: String) -> i32 {
    let mut nums: Vec<i32> = Vec::new();

    for c in line.chars() {
        if c.is_numeric() {
            let n: i32 = c as i32 - 0x30;
            nums.push(n);
        }
    }

    let min: i32 = nums[0];
    let vec_length: usize = nums.len();
    let max: i32 = nums[vec_length-1];

    let value: i32 = min*10 + max;

    return value;
}
