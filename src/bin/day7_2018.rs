// Day 7 for Advent of Code 2018: https://adventofcode.com/2018/day/7
extern crate regex;

use std::fs;
use regex::Regex;



fn main() {

	let line_reg = Regex::new(r"Step (.) must be finished before step (.) can begin\.$").unwrap();

	let file_content = fs::read_to_string("inputs/day7_input.txt").expect("error");
	let edges: Vec<(&str, &str)> = file_content.trim().split("\n").filter(|s| s.len() > 0).map(|s| (line_reg.captures(s).unwrap().get(1).map_or("", |m| m.as_str()), 
																									line_reg.captures(s).unwrap().get(1).map_or("", |m| m.as_str())
																									)
																								).collect();

	for (a, b) in edges {
		println!("{} {}", a, b)
	}
}