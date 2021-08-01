// Day 10 for Advent of Code 2018: https://adventofcode.com/2018/day/10

use std::fs;
use regex::Regex;


fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
	// position=<-30580, -10081> velocity=< 3,  1>
	let reg = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$").unwrap();
	
	let parsed = reg.captures(line).unwrap();
	let x: i32 = parsed.get(1).map_or("", |m| m.as_str()).parse().unwrap();
	let y: i32 = parsed.get(2).map_or("", |m| m.as_str()).parse().unwrap();
	let vx: i32 = parsed.get(3).map_or("", |m| m.as_str()).parse().unwrap();
	let vy: i32 = parsed.get(4).map_or("", |m| m.as_str()).parse().unwrap();
	((x, y), (vx, vy))
}

fn draw(xs: &Vec<i32>, ys: &Vec<i32>) -> i32 {
	let min_x = xs.iter().min().unwrap();
	let max_x = xs.iter().max().unwrap();

	let min_y = ys.iter().min().unwrap();
	let max_y = ys.iter().max().unwrap();
	
	let size_x = max_x - min_x + 1;
	let size_y = max_y - min_y + 1;
	
	if (size_x < 100) && (size_y < 100) {
		let mut table = vec![vec!['.'; size_x as usize]; size_y as usize];
		
		for i in 0..xs.len() {
			let idx_x = (xs[i] - min_x) as usize;
			let idx_y = (ys[i] - min_y) as usize;
			table[idx_y][idx_x] = '#'
		}

		for row in table {
			let row_str: String = row.into_iter().collect();
			println!("{}", row_str);
		};
	};

	size_y
}

fn increment(xs: &mut Vec<i32>, ys: &mut Vec<i32>, vxs: &Vec<i32>, vys: &Vec<i32>) {

	for i in 0..xs.len() {
		xs[i] += vxs[i];
		ys[i] += vys[i];
	}
}


fn run_part1(input: &str) {

	let lines  = input.split("\n").filter(|s| s.len() > 0);
	
	let (coord, velocity): (Vec<(i32, i32)>, Vec<(i32, i32)>) = lines.map(parse_line).unzip();

	let (mut xs, mut ys) = coord.into_iter().unzip();
	let (vxs, vys) = velocity.into_iter().unzip();
	let mut i = 0;
	let mut size_y = 100;
	while size_y > 10 {
		size_y = draw(&xs, &ys);
		increment(&mut xs, &mut ys, &vxs, &vys);
		i += 1;
	};
	println!("i={}", i - 1);
}

#[test]
fn test_parse_line() {

	parse_line("position=< 3, -2> velocity=<-1,  1>");
	parse_line("position=< 3, 10> velocity=<-1, 1>");

}


#[test]
fn test_part1() {
		let lines  = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
";

	run_part1(lines);

}



fn main() {

	let file_content = fs::read_to_string("inputs/day10_input.txt").expect("error");

	run_part1(file_content.trim());

	

}