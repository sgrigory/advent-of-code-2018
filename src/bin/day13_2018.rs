use std::fs;

struct Cart {
    x: usize,
    y: usize,
    dir: u8,
    turns: u32,
}

fn get_shifts(dir: u8) -> (i32, i32) {
    match dir {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => panic!("wrong dir"),
    }
}


fn main() {

    let file_content = fs::read_to_string("inputs/day13_input.txt").expect("error");
    
    let mut field = file_content.split("\n").filter(|x| x.len() > 0).map(|s| s.split("").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    for s in &field {
        print!("H");
        for p in s {
            print!("{}#", p);
        }
        println!("\n");
    }

    let mut carts = Vec::<Cart>::new();
    let size_x = field[0].len();
    let size_y = field.len();
    println!("{} {}", size_x, size_y);

    for i in 0..size_y {
        //println!("{}", field[i].len());
        for j in 0..size_x {
           

         match field[i][j].clone() {
             "^" => { carts.push(Cart{x: j, y: i, dir: 0, turns: 0}); field[i][j] = "|"; },
             ">" => { carts.push(Cart{x: j, y: i, dir: 1, turns: 0}); field[i][j] = "-"; },
             "v" => { carts.push(Cart{x: j, y: i, dir: 2, turns: 0}); field[i][j] = "|"; },
             "<" => { carts.push(Cart{x: j, y: i, dir: 3, turns: 0}); field[i][j] = "-"; },
             _ => ()
         };
         
        }
    }

    let mut time = 0;

    let mut carts_old: Vec<Cart> = carts.iter().map(|c| Cart{x: c.x, y: c.y, dir: c.dir, turns: c.turns}).collect();

    loop {
        
        carts.sort_by_key(|c| (c.y, c.x));
        // Make copy of cart positions before moving carts
        carts_old = carts.iter().map(|c| Cart{x: c.x, y: c.y, dir: c.dir, turns: c.turns}).collect();

        // Move carts
        for cart in carts.iter_mut() {

            if field[cart.y][cart.x] == "+" {
                
                let crossroad_dir = match cart.turns % 3 {
                    0 => (cart.dir + 4 - 1) % 4,
                    2 => (cart.dir + 1) % 4,
                    _ => cart.dir,
    
                };

                let shift = get_shifts(crossroad_dir); 
                cart.x = (cart.x as i32 + shift.0) as usize; 
                cart.y = (cart.y as i32 + shift.1) as usize; 
                cart.turns += 1;
                cart.dir = crossroad_dir;
            } else {

                match cart.dir {
                    0 => match field[cart.y][cart.x] {
                            "|" => cart.y -= 1,
                            "/" => { cart.x += 1; cart.dir = 1; },
                            "\\" => { cart.x -= 1; cart.dir = 3; },
                            _ => panic!(format!("can't go: {} {} {} {}", cart.x, cart.y, cart.dir, field[cart.y][cart.x]))
                        },
                    1 => match field[cart.y][cart.x] {
                        "-" => cart.x += 1,
                        "/" => { cart.y -= 1; cart.dir = 0; },
                        "\\" => { cart.y += 1; cart.dir = 2; },
                        _ => panic!(format!("can't go: {} {} {} {}", cart.x, cart.y, cart.dir, field[cart.y][cart.x]))
                        },

                    2 => match field[cart.y][cart.x] {
                        "|" => cart.y += 1,
                        "/" => { cart.x -= 1; cart.dir = 3; },
                        "\\" => { cart.x += 1; cart.dir = 1; },
                        _ => panic!(format!("can't go: {} {} {} {}", cart.x, cart.y, cart.dir, field[cart.y][cart.x]))
                    },

                    3 => match field[cart.y][cart.x] {
                        "-" => cart.x -= 1,
                        "/" => { cart.y += 1; cart.dir = 2; },
                        "\\" => { cart.y -= 1; cart.dir = 0; },
                        _ => panic!(format!("can't go: {} {} {} {}", cart.x, cart.y, cart.dir, field[cart.y][cart.x]))
                        },

                    _ => panic!(format!("can't go: {} {} {} {}", cart.x, cart.y, cart.dir, field[cart.y][cart.x]))
                }
        }
        }
        
        // Identify crashes
        for i in 0..carts.len() {
            for j in 0..i {
                let same_x = carts[i].x == carts[j].x;
                let same_y = carts[i].y == carts[j].y;
                
                if same_x && same_y {
                    println!("crash at time {}: {} {}", time, carts[i].x, carts[i].y);
                    return
                }

                if ((carts[i].x == carts_old[j].x) && (carts[i].y == carts_old[j].y) 
                && (carts_old[i].x == carts[j].x) && (carts_old[i].y == carts[j].y))
                {
                    println!("mirror crash at time {}: {} {}, {} {}; old: {} {}, {} {}", time, carts[i].x, carts[i].y, carts[j].x, carts[j].y,
                    carts_old[i].x, carts_old[i].y, carts_old[j].x, carts_old[j].y);
                    return

                }

            }
                
        };

        
        time += 1;
        println!("time = {}", time);

        
    }

}