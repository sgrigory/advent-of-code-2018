extern crate regex;

use std::fs;
use regex::Regex;

struct Rect {
	x0: usize,
	y0: usize,
	dx: usize,
	dy: usize,
}

fn get_claim(s: &str) -> Rect {
	// #1 @ 596,731: 11x27
	let claim_re = Regex::new(r".+ @ (\d+).(\d+): (\d+)x(\d+)$").unwrap();//Regex::new(r".+ @ (x0),(y0): (dx)x(dy)").unwrap();
	// let parsed = claim_re.captures(s).map(|cap| cap.iter().flat_map(|c| c).map(|c| c.as_str()).collect::<Vec<_>>()
	// 	);
	// for c in parsed {
	// 	for a in c {
	// 	println!("{}", a)
	// };
	// };
	let parsed = claim_re.captures(s).unwrap();
	let x0: usize = parsed.get(1).map_or("", |m| m.as_str()).parse().unwrap();
	let y0: usize = parsed.get(2).map_or("", |m| m.as_str()).parse().unwrap();
	let dx: usize = parsed.get(3).map_or("", |m| m.as_str()).parse().unwrap();
	let dy: usize = parsed.get(4).map_or("", |m| m.as_str()).parse().unwrap();
	//println!("{} {} {} {}", x0, y0, dx, dy);
	Rect {x0, y0, dx, dy}
}

fn main() {

	let file_content = fs::read_to_string("inputs/day3_input.txt").expect("error");
	let rects = file_content.trim().split("\n").map(get_claim).collect::<Vec<Rect>>();
	
	let mut field = [[0u8; 1000]; 1000];
	for rect in rects {
		for x in rect.x0..rect.x0 + rect.dx {
			for y in rect.y0..rect.y0 + rect.dy {
				field[x][y] = field[x][y].saturating_add(1);
			}
		}
	}

	let res_part1 = field.iter().flat_map(|x| x.iter().collect::<Vec<&u8>>()).filter(|&x| x > &1).collect::<Vec<&u8>>().len();
	println!("{}", res_part1);

	// for row in field[0..20].iter() {
	// 	for v in row[0..20].iter() {
	// 		print!("{}", v);
	// 	};
	// 	println!("")
	// }

}