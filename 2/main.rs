use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
		let mut horizontal_position: i32 = 0;
		let mut depth: i32 = 0;


    for line in reader.lines() {
			let line_result = line.unwrap();
			let split: Vec<&str> = line_result.split(" ").collect();

			if split.first().unwrap() == &"forward" {
				let parsed_pos: i16 = split.last().unwrap().parse().unwrap();
				horizontal_position += i32::from(parsed_pos)
			}

			if split.first().unwrap() == &"up" {
				let parsed_pos: i16 = split.last().unwrap().parse().unwrap();
				depth -= i32::from(parsed_pos)
			}

			if split.first().unwrap() == &"down" {
				let parsed_pos: i16 = split.last().unwrap().parse().unwrap();
				depth += i32::from(parsed_pos)
			}
    }

		let result: i32 = depth * horizontal_position;

		println!("{}", result);

    Ok(())
}
