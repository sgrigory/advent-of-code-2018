// Day 5 for Advent of Code 2018: recursive solution
use std::fs;


fn get_counts(closest_locs: Vec<Vec<usize>>, tie_placeholder: usize, size_x: usize, size_y: usize) -> Vec<usize> {

	let mut counts: Vec<usize> = vec![0; tie_placeholder + 1];

	for cell_x in 0..size_x {
		for cell_y in 0..size_y {
			counts[closest_locs[cell_x][cell_y]] += 1
		}
	}
	
	for x in 0..size_x {
		counts[closest_locs[x][0]] = 0;
		counts[closest_locs[x][size_y - 1]] = 0
	}

	for y in 0..size_y {
		counts[closest_locs[0][y]] = 0;
		counts[closest_locs[size_x - 1][y]] = 0
	}

	counts
}


fn main() {

	let file_content = fs::read_to_string("inputs/day6_input.txt").expect("error");
	let coords: Vec<(i32, i32)> = file_content.trim().split("\n").filter(|s| s.len() > 0).map(|s| s.split(", ").collect()).map(|x: Vec<&str>| (x[0].parse::<i32>().unwrap(),
																			    x[1].parse::<i32>().unwrap())).collect();
	// for c in coords {
	// 	println!("{} {}", c.0, c.1)
	// };

	let mut min_x: i32 = 1000;
	let mut max_x: i32 = 0;
	let mut min_y: i32 = 1000;
	let mut max_y: i32 = 0;

	for (c_x, c_y) in &coords {
		if c_x < &min_x {
			min_x = *c_x
		}

		if c_x > &max_x {
			max_x = *c_x
		}

		if c_y < &min_y {
			min_y = *c_y
		}

		if c_y > &max_y {
			max_y = *c_y
		}
	}

	let size_x = (max_x - min_x + 1) as usize;
	let size_y = (max_y - min_y + 1) as usize;
	let tie_placeholder = coords.len() + 1;

	let mut distances = vec![vec![1000; size_y]; size_x];
	let mut closest_locs: Vec<Vec<usize>> = vec![vec![tie_placeholder; (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];

	for (loc_idx, (loc_x, loc_y)) in coords.iter().enumerate() {
		for cell_x in 0..size_x {
			for cell_y in 0..size_y {
				let x = loc_x - min_x;
				let y = loc_y - min_y;

				let curr_dist = (cell_x as i32 - x).abs() + (cell_y as i32 - y).abs();

				if distances[cell_x][cell_y] > curr_dist {
					distances[cell_x][cell_y] = curr_dist;
					closest_locs[cell_x][cell_y] = loc_idx;
				} else {
					if distances[cell_x][cell_y] == curr_dist {
						closest_locs[cell_x][cell_y] = tie_placeholder
					}
				}
	}
	}
}


let counts = get_counts(closest_locs, tie_placeholder, size_x, size_y);

let part1_answer = counts.iter().max().unwrap();

println!("part 1 answer: {}", part1_answer);


}