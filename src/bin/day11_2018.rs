use std::cmp;

const GRID_SIZE: usize = 300;


fn get_cell_power(x: usize, y: usize, serial_number: u32) -> i32 {
	// 	Find the fuel cell's rack ID, which is its X coordinate plus 10.
	// Begin with a power level of the rack ID times the Y coordinate.
	// Increase the power level by the value of the grid serial number (your puzzle input).
	// Set the power level to itself multiplied by the rack ID.
	// Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
	// Subtract 5 from the power level.

	// 	The rack ID is 3 + 10 = 13.
	// The power level starts at 13 * 5 = 65.
	// Adding the serial number produces 65 + 8 = 73.
	// Multiplying by the rack ID produces 73 * 13 = 949.
	// The hundreds digit of 949 is 9.
	// Subtracting 5 produces 9 - 5 = 4.
	// (x * ((x + 10) * y + s)) / 100 - 5
	 let rack_id = x as i32 + 10;
	 (((rack_id * (y as i32) + serial_number as i32) * rack_id) / 100) % 10 - 5
}


fn get_square_power(cell_powers: &[[i32; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, square_size: usize) -> i32 {
	(0..square_size).flat_map(|dx| (0..square_size).map(move |dy| cell_powers[(y - dy) as usize][(x - dx) as usize])).sum()
}


fn get_best_square_power(cell_powers: &[[i32; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, max_square_size: usize) -> (i32, usize) {
	let mut s: i32 = 0;
	let mut best_square_power = i32::MIN;
	let mut best_square_size = 0;
	//println!("max_square_size = {}", max_square_size);
	for square_size in 0..max_square_size {
		s += cell_powers[(y - square_size) as usize][(x - square_size) as usize .. x + 1].iter().sum::<i32>();
		s += cell_powers[(y - square_size) as usize .. y + 1].iter().map(|row| row[(x - square_size) as usize]).sum::<i32>();
		s -= cell_powers[(y - square_size) as usize][(x - square_size) as usize];
		if s > best_square_power {
			best_square_power = s;
			best_square_size = square_size;
		}
	}
	(best_square_power, best_square_size)
}



fn run_part1(serial_number: u32) -> (usize, usize) {

	let mut cell_powers = [[0i32; GRID_SIZE]; GRID_SIZE];
	let mut squart_power = i32::MIN;
	let mut best_square_x = 0;
	let mut best_square_y = 0;

	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			cell_powers[y as usize][x as usize] = get_cell_power(x + 1, y + 1, serial_number);

			if (x > 3) & (y > 3) {
				let new_squart_power = get_square_power(&cell_powers, x, y, 3);
				if new_squart_power > squart_power {
					squart_power = new_squart_power;
					best_square_x = x - 3;
					best_square_y = y - 3;
				}
			}
		}
	}
	(best_square_x + 2, best_square_y + 2)
}


fn run_part2(serial_number: u32) -> (usize, usize, usize) {

	let mut cell_powers = [[0i32; GRID_SIZE]; GRID_SIZE];
	let mut squart_power = i32::MIN;
	let mut best_square_x = 0;
	let mut best_square_y = 0;
	let mut best_square_size = 0;

	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			cell_powers[y as usize][x as usize] = get_cell_power(x + 1, y + 1, serial_number);
		}
	};
	println!("finished computing cell powers");

	for x in 0..GRID_SIZE {
		println!("x = {}", x);
		for y in 0..GRID_SIZE {
			//println!("y = {}", y);
			let max_square_size = cmp::min(x, y) + 1;
			let (new_squart_power,  squart_size) = get_best_square_power(&cell_powers, x, y, max_square_size);
			if new_squart_power > squart_power {
				//println!("{} {} {}", squart_power, new_squart_power,  squart_size);
				//println!("replacing!");
				squart_power = new_squart_power;
				best_square_x = x - squart_size;
				best_square_y = y - squart_size;
				best_square_size = squart_size;

			}
		}
	}
	(best_square_x + 1, best_square_y + 1, best_square_size + 1)
}


#[test]
fn test_get_power() {
	// Fuel cell at 122,79, grid serial number 57: power level -5.
	// Fuel cell at 217,196, grid serial number 39: power level  0.
	// Fuel cell at 101,153, grid serial number 71: power level  4.
	assert_eq!(get_cell_power(3, 5, 8), 4);
	assert_eq!(get_cell_power(122, 79, 57), -5);
	assert_eq!(get_cell_power(217, 196, 39), 0);
	assert_eq!(get_cell_power(101, 153, 71), 4);

	

}


#[test]
fn test_run_part1() {
	assert_eq!(run_part1(18), (33, 45));
	assert_eq!(run_part1(42), (21, 61));
}


#[test]
fn test_run_part2_1() {
	assert_eq!(run_part2(18), (90, 269, 16));
	//assert_eq!(run_part2(42), (232, 251, 12));
}

#[test]
fn test_run_part2_2() {
	//assert_eq!(run_part2(18), (90, 269, 16));
	assert_eq!(run_part2(42), (232, 251, 12));
}


fn main() {

	
	let res_part1 = run_part1(1788);
	println!("part 1 result: {}, {}", res_part1.0, res_part1.1);

	let res_part2 = run_part2(1788);
	println!("part 2 result: {}, {}, {}", res_part2.0, res_part2.1, res_part2.2);

}