// Day 5 for Advent of Code 2018: recursive solution
use std::fs;


fn annihilate(a: u8, b: u8) -> bool {
	// Check if two characters can annihilate each other, i.e. if one is upper of lower case of another
	let a_A = (a >= b'a') && (a <= b'z') && (a as i8 - b as i8 == b'a' as i8 - b'A' as i8);
	let A_a = (a >= b'A') && (a <= b'Z') && (a as i8 - b as i8== b'A' as i8 - b'a' as i8);
	a_A || A_a
}

fn react_polymer(s: &[u8]) -> Vec<u8> {
	// Reduce a polymer string recursively
	if s.len() < 2 {
		// If string is 1 or 0 characters, don't do anything
		s.to_vec()
	}
	else {
		// Reduce left and right parts
		let left_reacted = react_polymer(&s[0..(s.len() / 2)]);
		let right_reacted = react_polymer(&s[(s.len() / 2)..s.len()]);
		// Reduce characters on the border of reduced left and right parts
		let mut i = 0;
		let l_len = left_reacted.len();
		let r_len = right_reacted.len();
		while (i < l_len) && (i < r_len) && annihilate(left_reacted[l_len - i - 1], right_reacted[i])
			   {
			i += 1;
		}
		let mut result: Vec<u8> = left_reacted[0..l_len - i].to_vec();
		// Put together reduced left and right parts minus reduced characters on the border
		result.extend(&right_reacted[i..r_len]);
		result
	}

}

fn main() {

	let file_content = fs::read_to_string("inputs/day5_input.txt").expect("error");
	let final_polymer = react_polymer(file_content.as_bytes());
	println!("part 1 answer: {}", final_polymer.len() - 1);
	
	let mut min_len = final_polymer.len() - 1;
	// For part 2, go through all characters of the alphabet, and try reducing the string with the corresponding character eliminated
	for c_lower in b'a'..(b'z' + 1) {
		let c_upper = c_lower + b'A' - b'a';
		let filtered_polymer = final_polymer.iter().filter(|&&x| (x != c_lower) && (x != c_upper)).collect::<Vec<&u8>>();
		let filtered_polymer_unref: Vec<u8> = filtered_polymer.into_iter().map(|x| *x).collect();
		let further_reduced = react_polymer(&filtered_polymer_unref[..]);
		if further_reduced.len() - 1 < min_len {
			min_len = further_reduced.len() - 1;
		}
	}
	println!("part 2 answer: {}", min_len);
}