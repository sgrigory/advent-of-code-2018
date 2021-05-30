use std::fs;

fn count(s: String) -> [u32; 256] {

	let mut res: [u32; 256] = [0; 256];
	for b in s.bytes() {
		res[b as usize] += 1
	};
	res
}

fn is_n(arr: [u32; 256], n: u32) -> bool {
	arr.iter().any(|&x| x == n)
}

fn is_one_diff(s1: String, s2: String) -> String {

	let same: String = s1.chars().zip(s2.chars()).filter(|e| e.0 == e.1).map(|e| e.0).collect();
	if same.len() == s1.len() - 1 {
		same
	} 
	else {
		"".to_string()
	}
}

fn main() {

	let file_content = fs::read_to_string("day2_input.txt").expect("error");
	let content: Vec<String> = file_content.trim().split("\n").map(|s| s.to_string()).collect();
	
	let counts: Vec<[u32; 256]> = content.clone().into_iter().map(|s| count(s)).collect();

	let num2: u32 = counts.iter().map(|&x| is_n(x, 2) as u32).sum();
	let num3: u32 = counts.iter().map(|&x| is_n(x, 3) as u32).sum();
	println!("{}", num2 * num3);

	let res_part2: String = content.clone().iter().flat_map(
		|s1| content.clone().iter().map(move |s2| is_one_diff(s1.to_string(), s2.to_string())).collect::<Vec<_>>()
		).filter(|s| s.len() > 0).collect::<Vec<String>>().join("|");
	println!("{}", res_part2);
}