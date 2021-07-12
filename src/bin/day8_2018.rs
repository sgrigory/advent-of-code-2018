// Day 8 for Advent of Code 2018: https://adventofcode.com/2018/day/8

use std::fs;


fn parse_line(input: &[u32]) -> (usize, u32, u32) {


	let num_children: u32 = input[0];
	let num_meta: u32 = input[1];

	let mut child_values = vec![0u32; num_children as usize];
	let mut total_meta = 0;
	let mut pos = 2;
	for i in 0 .. num_children {
		let (child_len, child_meta, child_value) = parse_line(&input[pos..input.len()]);

		total_meta += child_meta;
		pos += child_len;
		child_values[i as usize] = child_value;
	};

	// Sum up all metainfo of the current node
	let final_pos = pos + num_meta as usize;
	let total_own_meta: u32 = input[pos .. final_pos].iter().sum();

	let node_value: u32 = if num_children == 0 {
		 total_own_meta
	} else {
		input[pos .. final_pos].iter().map(|x| if x - 1 < num_children {child_values[*x as usize - 1]} else {0}).sum()
	};


	(final_pos, total_meta + total_own_meta, node_value)

}


fn run_part1(input: &str) -> u32 {

	let numbers: Vec<u32> = input.split(" ").filter(|s| s.len() > 0).map(|s| s.parse::<u32>().unwrap()).collect();
	
	let (_, total_meta, _) = parse_line(&numbers);

	total_meta
}


fn run_part2(input: &str) -> u32 {

	let numbers: Vec<u32> = input.split(" ").filter(|s| s.len() > 0).map(|s| s.parse::<u32>().unwrap()).collect();
	
	let (_, _, node_value) = parse_line(&numbers);

	node_value
}


#[test]
fn test_part_1() {

	let test_str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

	let expected_out: u32 = 138;

	assert_eq!(run_part1(test_str), expected_out);

}


#[test]
fn test_part_2() {

	let test_str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

	let expected_out: u32 = 66;

	assert_eq!(run_part2(test_str), expected_out);

}


fn main() {

	let file_content = fs::read_to_string("inputs/day8_input.txt").expect("error");

	let part1_res = run_part1(file_content.trim());
	
	println!("part 1 answer: {}", part1_res);


	let part2_res = run_part2(file_content.trim());
	
	println!("part 2 answer: {}", part2_res);

}