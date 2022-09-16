const STEPS_AFTER: usize = 10;
const MAX_SIZE: usize = 100000000;

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


fn run_part2(pattern: Vec<u8>) -> usize {
	let mut buf = Vec::with_capacity(MAX_SIZE + 2); //vec![0u8; MAX_SIZE + 2];
	buf[0] = 3;
	buf[1] = 7;
	let mut pos1: usize = 0;
	let mut pos2: usize = 1;
	let mut length: usize = 2;
	let mut flag = 0;
	while (length < MAX_SIZE) && (flag == 0) {
		let sm = buf[pos1] + buf[pos2];
		let dig_1 = sm / 10;
		let dig_2 = sm % 10;
		if dig_1 > 0 {
			buf.push(dig_1);
			buf.push(dig_2);
			length += 2;
		} else {
			buf.push(dig_2);
			length += 1;
		};
		 
		 pos1 = (pos1 + 1 + buf[pos1] as usize) % length;
		 pos2 = (pos2 + 1 + buf[pos2] as usize) % length;

		 if (length >= pattern.len()) && buf[length - pattern.len()..length] == pattern {
		 	flag = 1;
		 }
		 else if (length > pattern.len()) && (buf[length - pattern.len() - 1..length - 1] == pattern) {
		 	flag = 2;
		 };
	}
	println!("{} {} {} {}", pos1, pos2, buf[pos1], buf[pos2]);
	println!("{} {} {} {}", buf[length - 4], buf[length - 3], buf[length - 2], buf[length - 1]);
	length - pattern.len() + 1 - flag
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


#[test]
fn test_run_part2() {
 	/* 51589 first appears after 9 recipes.
01245 first appears after 5 recipes.
92510 first appears after 18 recipes.
59414 first appears after 2018 recipes.

*/
	assert_eq!(run_part2([5, 1, 5, 8, 9].to_vec()), 9);
	assert_eq!(run_part2([0, 1, 2, 4, 5].to_vec()), 5);
	assert_eq!(run_part2([9, 2, 5, 1, 0].to_vec()), 18);
	assert_eq!(run_part2([5, 9, 4, 1, 4].to_vec()), 2018);
}



fn main() {

	
	// let res_part_1 = run_part1(864801);

	// println!("part 1 result: {}", res_part_1);
	// // 864801
	// let res_part_2 = run_part2([8, 6, 4, 8, 0, 1].to_vec());
	
	// println!("part 2 result: {}", res_part_2);


	let res_part_1 = run_part1(380621);

	println!("part 1 result: {}", res_part_1);
	// 864801
	let res_part_2 = run_part2([3, 8, 0, 6, 2, 1].to_vec());
	
	println!("part 2 result: {}", res_part_2);



}