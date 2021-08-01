// Day 9 for Advent of Code 2018: https://adventofcode.com/2018/day/9

use std::fs;
use regex::Regex;

struct Node<'a> {
	prev: &'a mut Node<'a>,
	next: &'a mut Node<'a>,
	value: u32,
}

impl<'a> Node<'a> {
	fn walk(&'a self, steps: u32) -> &'a Node<'a> {
		
		let mut res: &'a Node = self;
		
		for _ in 0..steps {
			res = res.next;
		};
		
		&res
	} 

	fn insert(&'a mut self, value: u32) -> &'a Node<'a> {
		
		let self_next = self.walk(1);

		let mut new_node: Node<'a> = Node {
			prev: self,
			next: self_next,
			value: value,
		};

		self.next = &mut new_node;
		self.next.prev = &mut new_node;
		
		&new_node
	}

}


fn run_part1(input: &str) -> u32 {

	let reg = Regex::new(r"(\d+) players; last marble is worth (\d+) points$").unwrap();
	let parsed = reg.captures(input).unwrap();
	let n_players: u32 = parsed.get(1).map_or("", |m| m.as_str()).parse().unwrap();
	let last_marble: u32 = parsed.get(2).map_or("", |m| m.as_str()).parse().unwrap();
	println!("n_players: {},  last_marble: {}", n_players,last_marble );
	
	0

}


#[test]
fn test_part_1() {

	let test_examples = (("10 players; last marble is worth 1618 points", 8317),
					("13 players; last marble is worth 7999 points", 146373),
					("21 players; last marble is worth 6111 points", 54718),
					("30 players; last marble is worth 5807 points", 37305),
					);

	for (test_input, test_ouput) in test_examples {
		assert_eq!(run_part1(test_input), test_ouput);
	}
}



fn main() {

	let file_content = fs::read_to_string("inputs/day9_input.txt").expect("error");

	let part1_res = run_part1(file_content.trim());
	
	println!("part 1 answer: {}", part1_res);


	

}