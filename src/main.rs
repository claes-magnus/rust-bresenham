
extern crate clap;
#[derive(Debug)]


struct Point {
    x: i32,
    y: i32
}

use clap::{ App, Arg };

fn main() {
    let args = App::new("draw line")
        .version("0.0.1")
        .about("Drawes a line")
        .arg(Arg::with_name("x1") 
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("y1")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("x2")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("y2")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("width")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("height")
             .takes_value(true)
             .required(true))
        .get_matches();
    
    let x1:i32 = match args.value_of("x1").unwrap().parse() {
        Ok(n) => n,
        Err(_) => 1
    };
    let y1:i32 = match args.value_of("y1").unwrap().parse() {
        Ok(n) => n,
        Err(_) => 1
    };
    let x2:i32 = match args.value_of("x2").unwrap().parse() {
        Ok(n) => n,
        Err(_) => 1
    };
    let y2:i32 = match args.value_of("y2").unwrap().parse() {
        Ok(n) => n,
        Err(_) => 1
    };
    let w:i32 = match args.value_of("width").unwrap().parse() {
        Ok(n) =>    {
                        if n >= 10 && n <= 100 {
                            n
                        } else {
                            70
                        }
                    },
        Err(_) => 70
    };
    let h:i32 = match args.value_of("height").unwrap().parse() {
        Ok(n) =>    {
                        if n > 10 && n < 100 {
                            n
                        } else {
                            30
                        }
                    },
        Err(_) => 30
    };

  //  let result = get_coordinates(x1, y1, x2, y2);
  //  draw_line(result, w, h);
    get_lines();
}

fn get_coordinates(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Point> {
    let mut coordinates = vec![];
    let dx:i32 = i32::abs(x2 - x1);
    let dy:i32 = i32::abs(y2 - y1);
    let sx:i32 = {
        if x1 < x2 {
            1
        } else {
            -1
        }
    };
    let sy:i32 ={
        if y1 < y2 {
            1
        } else {
            -1
        }
    };
    let mut error:i32 = dx - dy;
    let mut current_x:i32 = x1;
    let mut current_y:i32 = y1;
    coordinates.push(Point { x: current_x, y: current_y });
    while current_x != x2 && current_y != y2 {
        let error2:i32 = 2 * error;
        if error2 >= i32::abs(dy) {
            error -= dy;
            current_x += sx;
            coordinates.push(Point { x: current_x, y: current_y });
        } else if error2 <= i32::abs(dx) {
            error += dx;
            current_y += sy;
            coordinates.push(Point { x: current_x, y: current_y });
        }
    }
    coordinates
}

fn draw_line(line: std::vec::Vec<Point>, width: i32, height: i32) {
    for col in 0..height {
        for row in 0..width {
            let is_point_in_line = line.iter().any(| point| point.x == row && point.y == col);
            match is_point_in_line {
                true => print!("❖"),
                _ => {
                    if col == 0 || col == (height - 1) || row == 0 || row == (width - 1) {
                        print!("☗");
                    } else {
                        print!(".");
                    }
                }
            };
        }
        print!("\n");
    }
}

use std::io;
fn get_lines() {

    println!("Please input width: ");
    let mut w_input = String::new();
    io::stdin().read_line(&mut w_input)
        .expect("Fail!");
    let w:i32 = match w_input.trim().parse() {
        Ok(n) => n,
        Err(_) => 70
    };
    println!("Please input height: ");
    let mut h_input = String::new();
    io::stdin().read_line(&mut h_input)
        .expect("Fail!");
    let h:i32 = match h_input.trim().parse() {
        Ok(n) => n,
        Err(_) => 30
    };

    println!("----------------------");
    let mut array_of_lines = vec![];

    let mut is_loop = true;
    while is_loop {
        let mut temp_arr = vec![];
        

        for question in 0..4 {
            let mut temp_input = String::new();
            println!("Enter value {}", question);
            io::stdin().read_line(&mut temp_input)
                .expect("Fail!");

            let value:i32 = match temp_input.trim().parse() {
                Ok(n) => n,
                Err(_) => -1
            };
            temp_arr.push(value);
        }
        let mut value = get_coordinates(temp_arr[0], temp_arr[1], temp_arr[2], temp_arr[3]);
        
        array_of_lines.push(value);
    //    println!("{:?}", array_of_lines);
        let mut ask_for_another_line = String::new();
        println!("Enter yes if you want to define another line: ");
        io::stdin().read_line(&mut ask_for_another_line)
            .expect("FAIL");
        match ask_for_another_line.trim() == "yes" {
            true => continue,
            _ => is_loop = false
        };
    }
    draw_lines(w, h, array_of_lines);
}

fn draw_lines(width: i32, height: i32, lines: std::vec::Vec<std::vec::Vec<Point>>) {
   

    //println!("{:?}", lines);
    
    
    for col in 0..height {
        for row in 0..width {
            let mut is_point_in_line = false;

            for line in &lines {

                let temp_check = line.iter().any(| point| point.x == row && point.y == col);
                if temp_check == true {
                    is_point_in_line = true;
                    break;
                } 
            }
            match is_point_in_line {
                true => print!("❖"),
                _ => {
                    if col == 0 || col == (height - 1) || row == 0 || row == (width - 1) {
                        print!("☗");
                    } else {
                        print!(".");
                    }
                }
            };
        }
        print!("\n");
    
    }
    
}


