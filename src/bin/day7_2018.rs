// Day 7 for Advent of Code 2018: https://adventofcode.com/2018/day/7
extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;


struct Vertex<'a> {
	ins: HashSet<&'a str>,
	outs: HashSet<&'a str>,
	seen: bool,
}


impl Vertex<'_> {
	pub fn new() -> Self {
		Vertex {
				ins: HashSet::new(),
				outs: HashSet::new(),
				seen: false,
			}
	}
}

type VertMap<'a> = HashMap<&'a str, Vertex<'a>>;


fn find_next_vertices<'a>(a: &str, vertices: &VertMap<'a>) -> Vec<&'a str> {
	
	// Identify vertices which haven't been seen
	let unseen_next_vertices = (&vertices[a]).outs.clone().into_iter().filter(|b| !vertices[b].seen).collect::<Vec<&str>>();

	// Filter out vertices for which some of the inputs still haven't been completed
	let final_next_vertices = unseen_next_vertices.into_iter().filter(|b| (vertices[b].ins.len() == 0) || (vertices[b].ins.clone().into_iter().all(|c| vertices[c].seen))).collect::<Vec<&str>>();

	final_next_vertices
}


fn run_part1(input: &str) -> String {

	let line_reg = Regex::new(r"Step (.) must be finished before step (.) can begin\.$").unwrap();

	let edges: Vec<(&str, &str)> = input.split("\n").filter(|s| s.len() > 0).map(|s| (line_reg.captures(s).unwrap().get(1).map_or("", |m| m.as_str()), 
																									line_reg.captures(s).unwrap().get(2).map_or("", |m| m.as_str())
																									)
																								).collect();

	// Create a vector of vertices, specifying incoming and outgoing neighbours of each
	let mut vertices: VertMap = HashMap::new();
	for (a, b) in edges {
		vertices.entry(a).or_insert(Vertex::new());
		vertices.entry(b).or_insert(Vertex::new());
		
		(*vertices.get_mut(a).unwrap()).outs.insert(b);
		(*vertices.get_mut(b).unwrap()).ins.insert(a);
	};

	// Do width-first search
	let mut path: Vec<&str> = Vec::new();
	let mut reachable_vertices: HashSet<&str> = vertices.iter().filter(|v| v.1.ins.len() == 0).map(|v| *v.0).collect();
	let mut change_flag = true;
	while change_flag{
		
		let mut sorted_reachable: Vec<_> = reachable_vertices.clone().into_iter().collect();
		sorted_reachable.sort();
		
		// Go through reachable vertices alphabetically till we find one which can be executed and haven't been seen before
		change_flag = false;
		for vertex_to_add in sorted_reachable {
			
			// If the vertex hasn been seen before, skip it
			if (*vertices.get_mut(vertex_to_add).unwrap()).seen {
				continue;
			};

			// If the vertex hasn't been seen before, add execute it - i.e. add to path
			(*vertices.get_mut(vertex_to_add).unwrap()).seen = true;
			change_flag = true;
			path.push(vertex_to_add);

			// Get all vertices reachable from the one we just added
			let new_vertices: Vec<&str> = path.iter().flat_map(|a| find_next_vertices(a, &vertices)).collect();
			// If a new reachable vertex was found, add it to the list of reachable vertices and continue into the outer loop
			if new_vertices.len() > 0 {
				reachable_vertices.extend(&new_vertices);
				break
			}
			
		};

		
	};

	path.join("")
}


#[test]
fn test_part_1() {

	let test_str = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";

let expected_out = "CABDFE";

assert_eq!(&run_part1(test_str), expected_out);

}


fn main() {

	let file_content = fs::read_to_string("inputs/day7_input.txt").expect("error");

	let part1_res = run_part1(file_content.trim());
	
	println!("part 1 answer: {}", part1_res);
}