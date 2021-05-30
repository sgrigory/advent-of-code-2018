extern crate regex;

use std::fs;
use regex::Regex;

struct Rect {
	id: u32,
	x0: usize,
	y0: usize,
	dx: usize,
	dy: usize,
}

fn get_claim(s: &str) -> Rect {
	// input looks like: #1 @ 596,731: 11x27
	let claim_re = Regex::new(r"#(\d+) @ (\d+).(\d+): (\d+)x(\d+)$").unwrap();

	let parsed = claim_re.captures(s).unwrap();
	let id: u32 = parsed.get(1).map_or("", |m| m.as_str()).parse().unwrap();
	let x0: usize = parsed.get(2).map_or("", |m| m.as_str()).parse().unwrap();
	let y0: usize = parsed.get(3).map_or("", |m| m.as_str()).parse().unwrap();
	let dx: usize = parsed.get(4).map_or("", |m| m.as_str()).parse().unwrap();
	let dy: usize = parsed.get(5).map_or("", |m| m.as_str()).parse().unwrap();
	
	Rect {id, x0, y0, dx, dy}
}

fn check_intrect(r1: &Rect, r2: &Rect) -> bool {
	let x_intersect = (r1.x0 < r2.x0 + r2.dx) && (r2.x0 < r1.x0 + r1.dx);
	let y_intersect = (r1.y0 < r2.y0 + r2.dy) && (r2.y0 < r1.y0 + r1.dy);
	x_intersect && y_intersect
}

fn main() {

	let file_content = fs::read_to_string("inputs/day3_input.txt").expect("error");
	let rects = file_content.trim().split("\n").map(get_claim).collect::<Vec<Rect>>();
	
	let mut field = [[0u8; 1000]; 1000];
	for rect in &rects {
		for x in rect.x0..rect.x0 + rect.dx {
			for y in rect.y0..rect.y0 + rect.dy {
				field[x][y] = field[x][y].saturating_add(1);
			}
		}
	}

	let res_part1 = field.iter().flat_map(|x| x.iter().collect::<Vec<&u8>>()).filter(|&x| x > &1).collect::<Vec<&u8>>().len();
	println!("{}", res_part1);

	'outer: for r1 in rects.iter() {
			
		for r2 in rects.iter() {

			if r1.id != r2.id {
				if check_intrect(&r1, &r2) {
					continue 'outer;
				}
			}

		}
		println!("found one with no intersections: {}", r1.id)
	}

}