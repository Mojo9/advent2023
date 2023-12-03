use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()>{
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut sum = 0;
    let input = File::open("files/day2.txt")?;
    println!("---  input: {:?}  ---", input);

    let reader = io::BufReader::new(input);
    println!("---  reader: {:?}  ---", &reader);

    for line in reader.lines() {
        println!("--  {:?}  --", line);
        let mut game_num = 0;
        let mut max_values = HashMap::new();
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split(|c| c == ':').collect();
                // get the number of the game in a general way:
                if let Some(game) = parts.first() {
                    let game_parts: Vec<&str> = game.split_whitespace().collect();

                    if let Some(number_str) = game_parts.get(1) {
                        if let Ok(game_number) = number_str.parse::<i32>() {
                            println!("-  game nr: {}  -", game_number);
                            game_num = game_number
                        } else {
                            println!("Failed to parse the number in string");
                        }
                    } else {
                        println!("Invalid format");
                    }
                } else {
                    println!("The vector for game is empty.");
                }
                // collect the 3 color maximum values in the hashmap "max_values"
                if let Some(colors) = parts.last() {
                    let sets:Vec<&str> = colors.split(';').collect();
                    for set in sets {
                        let color_values: Vec<&str> = set.split(',').map(|s| s.trim()).collect();
                        for cv in color_values {
                            let cv_parted: Vec<&str> = cv.split_whitespace().collect();
                            if cv_parted.len() == 2 {
                                if let Ok(value) = cv_parted[0].parse::<i32>() {
                                    let current_max = max_values.entry(cv_parted[1]).or_insert(0);
                                    if value > *current_max {
                                        println!("set new max form {} to {}",current_max,value);
                                        *current_max = value;
                                    }
                                } else {
                                    println!("Failed to parse value {}", cv_parted[0])
                                }
                            }
                        }
                    }
                } else {
                    println!("The vector for colors is empty.");
                }
                // check if a value in the hashmap is bigger than the limit or rgb
                for (color, value) in &max_values {
                    println!("--  col {} :: val {}  --",color,value);

                }
                let hred = max_values.get("red").unwrap();
                let hgreen = max_values.get("green").unwrap();
                let hblue = max_values.get("blue").unwrap();
                if hred <= &red && hgreen <= &green && hblue <= &blue {
                    sum = sum + game_num
                }


            }
            Err(err) => {
                eprintln!("Error in line 16: {err}", err)
            }
        }


    }
    println!("----  Result: {}  ----",sum);
    Ok(())
}
