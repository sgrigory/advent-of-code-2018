use std::fs;
use std::collections::HashSet;

fn main() {

	let content = fs::read_to_string("day1_input.txt").expect("error");
	
	let res: Vec<i32> = content.trim().split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
	let res_sum: i32 = res.iter().sum();
	println!("{}", res_sum);

	let mut curr = 0;
	let mut i = 0;
	let mut set = HashSet::new();
	let mut found = false;
	while !found {

		curr += res[i];
		i = (i + 1) % res.len();
		if set.contains(&curr) {
			println!("{}", curr);
			found = true;
		}
		set.insert(curr);
	}


}