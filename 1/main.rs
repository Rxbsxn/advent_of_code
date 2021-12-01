use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut calculated_window: [u16; 3] = [0; 3];
    let mut last_depth: Option<u16> = None;
    let mut increases_count: i32 = 0;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);


    for (idx, line) in reader.lines().enumerate() {
        let depth = line.unwrap().parse().unwrap();

        calculated_window[idx % 3] = depth;

        if idx < 2 {
            continue;
        }

        let window_sum = calculated_window.iter().sum();
        if last_depth.is_none() {
            last_depth = Some(depth);
            continue;
        } else {
            if window_sum > last_depth.unwrap() {
                increases_count += 1;
            }
            last_depth = Some(window_sum);
        }
    }

    println!("{}", increases_count);

    Ok(())
}
