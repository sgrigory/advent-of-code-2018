use std::fs;
use std::str;

const buf_size: usize = 250;

fn run_part1(input: &str, n_iter: u32) -> i32 {

	let lines: Vec<&str> = input.split("\n").filter(|s| s.len() > 0).collect();
	let init_state = lines[0].split(": ").collect::<Vec<&str>>()[1];
	let rules: Vec<(&[u8], u8)> = parse_rules(lines);
	let mut state = vec![b'.'; buf_size];
	let basis = 10; //(state.len() / 2) as usize;
	for i in 0..init_state.len() {
		state[i + basis] = init_state.as_bytes()[i];
	};
	println!("{}", str::from_utf8(&state).unwrap());
	println!("--------");
	for i in 0..n_iter {
		state = transform_state(&state, &rules);
		println!("{} {} {}", str::from_utf8(&state).unwrap(), get_pos_sum(&state, basis), i + 1);
	};

	get_pos_sum(&state, basis)
}

fn get_pos_sum(state: &[u8], basis: usize) -> i32{

	(0..state.len()).filter(|&i| state[i] == b'#').map(|i| i as i32 - basis as i32).sum()
}

fn parse_rules(lines: Vec<&str>) -> Vec<(&[u8], u8)> {
	lines[1..lines.len()].iter().map(|x| x.split(" => ").collect()).map(|x: Vec<&str>| (x[0].as_bytes(), x[1].as_bytes()[0])).collect()
}


fn transform_state<'a>(state: &'a [u8], rules: &Vec<(&[u8], u8)>) -> Vec<u8> {
		let mut new_state = Vec::<u8>::new();
		new_state = state.clone().to_vec();
		//println!("starting with a state");
		//println!("{}", str::from_utf8(&new_state).unwrap());
			//println!("{} {}", str::from_utf8(rule.0).unwrap(), rule.1); 
			for i in 0..state.len() {
				new_state[i] = b'.';
			  for rule in rules {
				if (i > 1) && (i < state.len() - 2) && rule_matches(&state[i - 2..i + 3], rule.0)
					{	
						//println!("replacing {} with {} at {}", new_state[i], rule.1, i);
						//println!("{}", str::from_utf8(&new_state).unwrap());
						new_state[i] = rule.1;
						//println!("{}", str::from_utf8(&new_state).unwrap());
					}
			}
		}
		//println!("returning new state");
		//println!("{}", str::from_utf8(&new_state).unwrap());
		new_state
	}


fn rule_matches(state_slice: &[u8], rule_lhs: &[u8]) -> bool {
	let res = state_slice.iter().zip(rule_lhs.iter()).all(|x| x.0 == x.1);
	// if res {
	// 	println!("{} {} {} {} {}", state_slice.len(), rule_lhs.len(), str::from_utf8(&state_slice).unwrap(), str::from_utf8(&rule_lhs).unwrap(), res);
	// }
	res
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

// #[test]
// fn test_transform_state() {
	
// 	assert_eq!(transform_state(b"...##", &vec![(b"...##", b'#')]),
// 							   b"..###");

// 	assert_eq!(transform_state(b".#....##....", &vec![(b"...##", b'#')]),
// 							   b".#...###....");

// 	assert_eq!(transform_state(b".#...###....", &vec![(b"..###", b'.')]),
// 						       b".#....##....");

// }


#[test]
fn test_rule_matches() {
	assert!(rule_matches(b"...#...", b"...#..."));
}


fn main() {

	let file_content = fs::read_to_string("inputs/day12_input.txt").expect("error");

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
	

	//let res_part_1 = run_part1(test_input, 100);	

	let res_part_1 = run_part1(file_content.trim(), 20);

	println!("part 1 answer: {}", res_part_1);

	let v1 = run_part1(file_content.trim(), 100);
	let v2 = run_part1(file_content.trim(), 101);
	let res_part_2 = (v2 - v1) as i64 * (50000000000i64 - 100) + v1 as i64;

	println!("part 2 answer: {}", res_part_2);

}