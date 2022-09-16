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


fn any_worker_busy(workers: &Vec<u8>) -> bool {
	workers.iter().any(|x| x > &0)
}


fn any_worker_free(workers: &Vec<u8>) -> bool {
	workers.iter().any(|x| x == &0)
}


fn get_free_worker(workers: &Vec<u8>) -> usize {
	workers.iter().position(|x| x == &0).unwrap()
}


fn get_load(letter: &str, base: u8) -> u8 {
	println!("{} {}", letter, letter.as_bytes()[0] - 65 + base);
	letter.as_bytes()[0] - 65 + base
}


// fn do_tick<'a>(workers: &'a mut Vec<u8>, queue: &'a mut HashMap<&str, u8>) -> bool {
	
// 	let mut change_flag = false;

// 	for i in 0..workers.len() {
// 			if workers[i] > 0 {
// 				workers[i] -= 1;
// 				};
// 			};

// 	let keys: Vec<&str> = queue.keys().map(|x| *x).collect();

// 	for key in keys {
// 		*queue.get_mut(key).unwrap() -= 1;
// 		if queue[key] == 0 {
// 			change_flag = true;
// 			queue.remove(key);

// 		};
// 	};

// 	change_flag
// }

fn do_tick_workers(workers: &mut Vec<u8>)  {

	for i in 0..workers.len() {
			if workers[i] > 0 {
				workers[i] -= 1;
				};
			};

}


fn do_tick_queue<'a>(queue: &'a mut HashMap<&str, u8>) -> Vec<&'a str> {
	
	//let mut change_flag = false;
	let mut done_nodes: Vec<&str> = Vec::new();

	let keys: Vec<&str> = queue.keys().map(|x| *x).collect();

	for key in keys {
		*queue.get_mut(key).unwrap() -= 1;
		if queue[key] == 0 {
			//change_flag = true;
			queue.remove(key);
			done_nodes.push(key);

		};
	};

	done_nodes
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
	while change_flag {
		
		let mut sorted_reachable: Vec<_> = reachable_vertices.clone().into_iter().collect();
		sorted_reachable.sort();
		
		// Go through reachable vertices alphabetically till we find one which can be executed and haven't been seen before
		change_flag = false;
		for vertex_to_add in sorted_reachable {
			
			// If the vertex has been seen before, skip it
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



fn run_part2(input: &str, n_workers: usize, base: u8) -> u32 {
	println!("-----------------------------------------");
	let mut workers = vec![0u8; n_workers];
	let mut queue: HashMap<&str, u8> = HashMap::new();
	let mut tot_time = 0;

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
	while change_flag | any_worker_busy(&workers) | (queue.len() > 0) {
		println!("--- outer loop --- ");
		
		if any_worker_free(&workers) {
			println!("found free workers");
		
			let mut sorted_reachable: Vec<_> = reachable_vertices.clone().into_iter().collect();
			sorted_reachable.sort();
			println!("sorted_reachable: {}", sorted_reachable.join(""));

			
			// Go through reachable vertices alphabetically till we find one which can be executed and haven't been seen before
			change_flag = false;
			for vertex_to_add in sorted_reachable {
				println!("checking vertex {}", vertex_to_add);
				
				// If the vertex has been seen before, skip it
				if (*vertices.get_mut(vertex_to_add).unwrap()).seen {
					continue;
				};

				// If the vertex hasn't been seen before, add execute it - i.e. add to path
				(*vertices.get_mut(vertex_to_add).unwrap()).seen = true;
				change_flag = true;
				println!("pushing vertex {}", vertex_to_add);
				queue.insert(vertex_to_add, get_load(vertex_to_add, base));

				while ! any_worker_free(&workers){
					println!("waiting...");
					do_tick_workers(&mut workers);
					//let done_nodes = do_tick_queue(&mut queue);
					change_flag = change_flag | (done_nodes.len() > 0);
					// for node in done_nodes {
					// 	path.push(node.clone());
					// };
					tot_time += 1;
					//println!("tot_time: {}, path: {}, queue: {}, change_flag: {}", tot_time, path.join(""), queue.keys().map(|x| *x).collect::<Vec<&str>>().join(""), change_flag);
					for worker in &workers {
						println!("worker {}", worker);
				};
				}

				
				let free_worker = get_free_worker(&workers);
				workers[free_worker] += get_load(vertex_to_add, base);
				println!("allocated load {} to {}", vertex_to_add, free_worker);

				// Get all vertices reachable from the one we just added
				let new_vertices: Vec<&str> = path.iter().flat_map(|a| find_next_vertices(a, &vertices)).collect();
				// If a new reachable vertex was found, add it to the list of reachable vertices and continue into the outer loop
				if new_vertices.len() > 0 {
					println!("{} new reachable vertics were found, breaking", new_vertices.len());
					reachable_vertices.extend(&new_vertices);
					//break
				}
				else {
					println!("no new vertices");
				}
				
			};

		}

		do_tick_workers(&mut workers);
		let done_nodes1 = do_tick_queue(&mut queue);
		//path.append(&mut done_nodes1);
		change_flag = change_flag | (done_nodes1.len() > 0);
		tot_time += 1;
		

		//println!("tot_time: {}, path: {}, queue: {}, change_flag: {}", tot_time, path.join(""), queue.keys().map(|x| *x).collect::<Vec<&str>>().join(""), change_flag);
		for worker in &workers {
			println!("worker {}", worker);
		};

		// Get all vertices reachable from the one we just added
		let new_vertices1: Vec<&str> = path.iter().flat_map(|a| find_next_vertices(a, &vertices)).collect();
		// If a new reachable vertex was found, add it to the list of reachable vertices and continue into the outer loop
		if new_vertices1.len() > 0 {
			println!("{} new reachable vertics were found, breaking", new_vertices1.len());
			reachable_vertices.extend(&new_vertices1);
		}

	};

	tot_time
}



// fn run_part2(input: &str, n_workers: usize, base: u8) -> u32 {
// 	println!("-----------------------------------------");
// 	let mut workers = vec![0u8; n_workers];
// 	let mut queue: HashMap<&str, u8> = HashMap::new();
// 	let mut tot_time = 0;

// 	let line_reg = Regex::new(r"Step (.) must be finished before step (.) can begin\.$").unwrap();

// 	let edges: Vec<(&str, &str)> = input.split("\n").filter(|s| s.len() > 0).map(|s| (line_reg.captures(s).unwrap().get(1).map_or("", |m| m.as_str()), 
// 																									line_reg.captures(s).unwrap().get(2).map_or("", |m| m.as_str())
// 																									)
// 																								).collect();

// 	// Create a vector of vertices, specifying incoming and outgoing neighbours of each
// 	let mut vertices: VertMap = HashMap::new();
// 	for (a, b) in edges {
// 		vertices.entry(a).or_insert(Vertex::new());
// 		vertices.entry(b).or_insert(Vertex::new());
		
// 		(*vertices.get_mut(a).unwrap()).outs.insert(b);
// 		(*vertices.get_mut(b).unwrap()).ins.insert(a);
// 	};

// 	// Do width-first search
// 	let mut path: Vec<&str> = Vec::new();
// 	let mut reachable_vertices: HashSet<&str> = vertices.iter().filter(|v| v.1.ins.len() == 0).map(|v| *v.0).collect();
// 	let mut change_flag = true;
// 	while change_flag | any_worker_busy(&workers) | (queue.len() > 0) {
// 		println!("--- outer loop --- ");
		
// 		if any_worker_free(&workers) {
// 			println!("found free workers");
		
// 			let mut sorted_reachable: Vec<_> = reachable_vertices.clone().into_iter().filter(|a| !queue.contains_key(*a)).collect();
// 			sorted_reachable.sort();
// 			println!("sorted_reachable: {}", sorted_reachable.join(""));

			
// 			// Go through reachable vertices alphabetically till we find one which can be executed and haven't been seen before
// 			change_flag = false;
// 			for vertex_to_add in sorted_reachable {
// 				println!("checking vertex {}", vertex_to_add);
				
// 				// If the vertex has been seen before, skip it
// 				if (*vertices.get_mut(vertex_to_add).unwrap()).seen {
// 					continue;
// 				};

// 				// If the vertex hasn't been seen before, add execute it - i.e. add to path
// 				(*vertices.get_mut(vertex_to_add).unwrap()).seen = true;
// 				change_flag = true;
// 				println!("pushing vertex {}", vertex_to_add);
// 				path.push(vertex_to_add);
// 				queue.insert(vertex_to_add, get_load(vertex_to_add, base));

// 				while ! any_worker_free(&workers){
// 					println!("waiting...");
// 					change_flag = change_flag | do_tick(&mut workers, &mut queue);
// 					tot_time += 1;
// 					println!("tot_time: {}, path: {}, queue: {}, change_flag: {}", tot_time, path.join(""), queue.keys().map(|x| *x).collect::<Vec<&str>>().join(""), change_flag);
// 					for worker in &workers {
// 						println!("worker {}", worker);
// 				};
// 				}

				
// 				let free_worker = get_free_worker(&workers);
// 				workers[free_worker] += get_load(vertex_to_add, base);
// 				println!("allocated load {} to {}", vertex_to_add, free_worker);

// 				// Get all vertices reachable from the one we just added
// 				let new_vertices: Vec<&str> = path.iter().filter(|a| !queue.contains_key(*a)).flat_map(|a| find_next_vertices(a, &vertices)).collect();
// 				// If a new reachable vertex was found, add it to the list of reachable vertices and continue into the outer loop
// 				if new_vertices.len() > 0 {
// 					println!("{} new reachable vertics were found, breaking", new_vertices.len());
// 					reachable_vertices.extend(&new_vertices);
// 					//break
// 				}
// 				else {
// 					println!("no new vertices");
// 				}
				
// 			};

// 		}

// 		change_flag = change_flag | do_tick(&mut workers, &mut queue);
// 		tot_time += 1;
		

// 		println!("tot_time: {}, path: {}, queue: {}, change_flag: {}", tot_time, path.join(""), queue.keys().map(|x| *x).collect::<Vec<&str>>().join(""), change_flag);
// 		for worker in &workers {
// 			println!("worker {}", worker);
// 		};

// 		// Get all vertices reachable from the one we just added
// 		let new_vertices1: Vec<&str> = path.iter().filter(|a| !queue.contains_key(*a)).flat_map(|a| find_next_vertices(a, &vertices)).collect();
// 		// If a new reachable vertex was found, add it to the list of reachable vertices and continue into the outer loop
// 		if new_vertices1.len() > 0 {
// 			println!("{} new reachable vertics were found, breaking", new_vertices1.len());
// 			reachable_vertices.extend(&new_vertices1);
// 		}

// 	};

// 	tot_time
// }


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



#[test]
fn test_part_2() {

	let test_str0 = "
Step C must be finished before step A can begin.
";

assert_eq!(run_part2(test_str0, 2, 1), 5);


	let test_str1 = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";
assert_eq!(run_part2(test_str1, 2, 1), 15);

// let test_str3 = "
// Step C must be finished before step A can begin.
// Step C must be finished before step F can begin.
// Step A must be finished before step B can begin.
// ";
// assert_eq!(run_part2(test_str3, 2, 1), 9);

let test_str2 = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
";
assert_eq!(run_part2(test_str2, 2, 1), 10);




}


fn main() {

	let file_content = fs::read_to_string("inputs/day7_input.txt").expect("error");

	let part1_res = run_part1(file_content.trim());
	
	println!("part 1 answer: {}", part1_res);

	let part2_res = run_part2(file_content.trim(), 5, 61);
	
	println!("part 2 answer: {}", part2_res);

}