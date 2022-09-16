
use std::fs;
use std::collections::HashSet;

extern crate itertools;
use itertools::Itertools;



fn parse_line(x: &str) -> (i32, i32, i32, i32) {

 let parsed = x.split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
 (parsed[0], parsed[1], parsed[2], parsed[3])
}


fn find_neighbours_point(point: (i32, i32, i32, i32))-> HashSet<(i32, i32, i32, i32)> {
	let mut res: HashSet<(i32, i32, i32, i32)> = HashSet::new();
	let range = -3..4_i32;
	let shifts = (0..4).map(|_| range.clone()).multi_cartesian_product().map(|a| (a[0], a[1], a[2], a[3])).filter(|a| a.0.abs() + a.1.abs() + a.2.abs() + a.3.abs() <= 3).collect::<Vec<(i32, i32, i32, i32)>>();

	for shift in shifts{
		res.insert((point.0 + shift.0, point.1 + shift.1, point.2 + shift.2, point.3 + shift.3));
		//println!("{} {} {} {} {} {} {} {}", point.0, point.1, point.2, point.3, shift.0, shift.1, shift.2, shift.3);
	}
	
	res
}

fn find_neighbours(points: HashSet<(i32, i32, i32, i32)>)-> HashSet<(i32, i32, i32, i32)> {
	let start_set = HashSet::<(i32, i32, i32, i32)>::new();
	points.into_iter().map(find_neighbours_point).fold(start_set, |x, y| x.union(&y).map(|x| *x).collect())
}


fn run_part1(file_content: &str) -> u32 {

	let points = file_content.trim().split("\n").map(|x| parse_line(x)).collect::<Vec<(i32, i32, i32, i32)>>();

	let mut n_components: u32 = 0;
	let mut remaining_points: HashSet<(i32, i32, i32, i32)> = points.into_iter().collect();
	while remaining_points.len() > 0 {
		//println!("remaining_points: {}", remaining_points.len());
		let mut new_nodes: HashSet<(i32, i32, i32, i32)> = HashSet::new();
		let first_node = remaining_points.iter().next().unwrap();
		new_nodes.insert(*first_node);

		while new_nodes.len() > 0 {
			//println!("new_nodes: {}", new_nodes.len());
			remaining_points = remaining_points.difference(&new_nodes).map(|x| *x).collect();
			let new_nodes_candidates = find_neighbours(new_nodes);
			//println!("new_nodes_candidates: {} remaining_points: {}", new_nodes_candidates.len(), remaining_points.len());
			new_nodes = new_nodes_candidates.intersection(&remaining_points).map(|x| *x).collect();
			//println!("new_nodes intersection: {}", new_nodes.len());
			// for point in &remaining_points {
			// 	println!("{} {} {} {}", point.0, point.1, point.2, point.3);
			// };
		};

		n_components += 1;
	};
	n_components
}


#[test]
fn test_part1_1() {
 let input = "
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
";

assert_eq!(run_part1(input), 2);

}


#[test]
fn test_part1_2() {
 let input = "
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
";

assert_eq!(run_part1(input), 4);

}


#[test]
fn test_part1_3() {
 let input = "
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
";

assert_eq!(run_part1(input), 3);

}


#[test]
fn test_part1_4	() {
 let input = "
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
";

assert_eq!(run_part1(input), 8);

}


fn main() {

	let file_content = fs::read_to_string("inputs/day25_input.txt").expect("error");
	
	let res_part1 = run_part1(&file_content);
	
	println!("{}", res_part1);
}