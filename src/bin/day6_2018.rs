// Day 6 for Advent of Code 2018: https://adventofcode.com/2018/day/6
use std::fs;


fn get_counts(closest_locs: Vec<Vec<usize>>, tie_placeholder: usize, size_x: usize, size_y: usize) -> Vec<usize> {
	// Given array of closest locations for each cell, find size of the region corresponding to each location

	let mut counts: Vec<usize> = vec![0; tie_placeholder + 1];
	// Count size of the region corresponding to each marked location
	for cell_x in 0..size_x {
		for cell_y in 0..size_y {
			counts[closest_locs[cell_x][cell_y]] += 1
		}
	}
	
	// Region which reach the boundary rectangle are infinite - exclude such locations by setting their count to zero
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

	let mut min_x: i32 = 1000;
	let mut max_x: i32 = 0;
	let mut min_y: i32 = 1000;
	let mut max_y: i32 = 0;

	// Find left, right, top, and bottom sides of the rectangle covering all marked locations
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

	// Find of the rectangle covering all marked locations
	let size_x = (max_x - min_x + 1) as usize;
	let size_y = (max_y - min_y + 1) as usize;
	// This will denote tie cells (those which are equidistant to at least two locations)
	let tie_placeholder = coords.len() + 1;

	// Array to store distances to the closest location
	let mut distances = vec![vec![1000; size_y]; size_x];
	// Array to store indices of the closest location
	let mut closest_locs: Vec<Vec<usize>> = vec![vec![tie_placeholder; (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];
	// Array to store total distances to all locations - for part 2
	let mut total_dists = vec![vec![0; size_y]; size_x];

	// Iterate over marked locations
	for (loc_idx, (loc_x, loc_y)) in coords.iter().enumerate() {
		// Iterate over cells  and find the closest location for each cell
		for cell_x in 0..size_x {
			for cell_y in 0..size_y {
				let x = loc_x - min_x;
				let y = loc_y - min_y;

				// Manhattan distance from the current location to the current cell
				let curr_dist = (cell_x as i32 - x).abs() + (cell_y as i32 - y).abs();
				
				// Update total distances to all locations from the current cell - for part 2
				total_dists[cell_x][cell_y] += curr_dist;

				// If found a location which is closer to the given cell then the current closest one, update
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

// Given array of closest locations for each cell, find size of the region corresponding to each location
let counts = get_counts(closest_locs, tie_placeholder, size_x, size_y);
// Find location with the region of maximum size
let part1_answer = counts.iter().max().unwrap();
println!("part 1 answer: {}", part1_answer);

// Count cells where total distance to all locations < 10000
let part2_answer = total_dists.iter().flat_map(|x| x.iter().filter(|&&y| y < 10000)).collect::<Vec<&i32>>().len();
println!("part 2 answer: {}", part2_answer);
}