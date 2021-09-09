use std::fs;
use std::str;

fn run_part1(input: &str, n_iter: u32) -> i32 {

	let lines: Vec<&str> = input.split("\n").filter(|s| s.len() > 0).collect();
	let init_state = lines[0].split(": ").collect::<Vec<&str>>()[1];
	let rules: Vec<(&[u8], u8)> = lines[1..lines.len()].iter().map(|x| x.split(" => ").collect()).map(|x: Vec<&str>| (x[0].as_bytes(), x[1].as_bytes()[0])).collect();
	let mut state = [b'.'; 200];
	let basis = (state.len() / 2) as usize;
	for i in 0..init_state.len() {
		state[i + basis] = init_state.as_bytes()[i];
	};
	println!("{}", str::from_utf8(&state).unwrap());
	println!("--------");
	for _ in 0..n_iter {
		let mut new_state = state.clone();
		for rule in &rules {
			//println!("{} {}", str::from_utf8(rule.0).unwrap(), rule.1);
			for i in 3..state.len() - 3 {
				if (state[i - 2] == rule.0[0]) & (state[i - 1] == rule.0[1]) & (state[i] == rule.0[2]) & (state[i + 1] == rule.0[3]) & (state[i + 2] == rule.0[4])
					{
						new_state[i] = rule.1;
					}
			}
		}
		state = new_state.clone();
		println!("{}", str::from_utf8(&state).unwrap());
	};

	(0..state.len()).filter(|&i| state[i] == b'#').map(|i| i as i32 - basis as i32).sum()
}

#[test]
fn test_part_1(){
	let test_input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

	assert_eq!(run_part1(test_input, 20), 325);
}


fn main() {

	let file_content = fs::read_to_string("inputs/day12_input.txt").expect("error");

	let res_part_1 = run_part1(file_content.trim(), 20);

	println!("{}", res_part_1);

}