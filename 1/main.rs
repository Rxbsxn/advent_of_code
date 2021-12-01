use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut last_depth: Option<u16> = None;
    let mut increases_count: i32 = 0;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let depth = line.unwrap().parse().unwrap();
        if last_depth.is_none() {
            last_depth = Some(depth);
            continue;
        } else {
            if depth > last_depth.unwrap() {
                increases_count += 1;
            }
            last_depth = Some(depth);
        }
    }

    println!("{}", increases_count);

    Ok(())
}
