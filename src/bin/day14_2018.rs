const STEPS_AFTER: usize = 10;

fn run_part1(a: usize) -> String {
	let mut buf = vec![0u8; 2 * (a + STEPS_AFTER)];
	buf[0] = 3;
	buf[1] = 7;
	let mut pos1 = 0;
	let mut pos2 = 1;
	let mut length = 2;
	for _ in 0..a + STEPS_AFTER {
		let sm = buf[pos1] + buf[pos2];
		let dig_1 = sm / 10;
		let dig_2 = sm % 10;
		if dig_1 > 0 {
			buf[length] = dig_1;
			buf[length + 1] = dig_2;
			length += 2;
		} else {
			buf[length] = dig_2;
			length += 1;
		};
		 pos1 = (pos1 + 1 + buf[pos1] as usize) % length;
		 pos2 = (pos2 + 1 + buf[pos2] as usize) % length;

	}

	buf[a..a + STEPS_AFTER].iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
}

#[test]
fn test_run_part1() {
 	/* If the Elves think their skill will improve after making 9 recipes, 
 	the scores of the ten recipes after the first nine on the scoreboard would be 5158916779 (highlighted in the last line of the diagram).
After 5 recipes, the scores of the next ten would be 0124515891.
After 18 recipes, the scores of the next ten would be 9251071085.
After 2018 recipes, the scores of the next ten would be 5941429882.
*/
	assert_eq!(run_part1(9), "5158916779");
	assert_eq!(run_part1(5), "0124515891");
	assert_eq!(run_part1(18), "9251071085");
	assert_eq!(run_part1(2018), "5941429882");
}


fn main() {

	
	let res_part_1 = run_part1(864801);

	println!("part 1 result: {}", res_part_1);

}