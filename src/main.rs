use std::{io, collections::HashMap};

fn get_input_vec() -> Vec<i32> {
    let mut num_vec: Vec<i32> = Vec::new();
    'input: loop {
        let mut input = String::new();

        io::stdin().
            read_line(&mut input).
            expect("line should be read from stdin");
    
        for num_str in input.trim().replace(" ", "").split(',') {
            match num_str.parse() {
                Ok(num) => {
                    num_vec.push(num);
                }
                Err(_) => {
                    println!("Please input a correct list of numbers");
                    continue 'input;
                }
            };
        }
        break 'input;
    }
    num_vec
}

#[derive(Debug)]
enum Action {
    Mean,
    Median,
    Mode,
}

fn get_input_action() -> Action {
    loop {
        let mut input = String::new();

        io::stdin().
            read_line(&mut input).
            expect("line should be read from stdin");
    
        match input.trim().parse() {
            Ok(action) => {
                return match action {
                    1 => Action::Mean,
                    2 => Action::Median,
                    3 => Action::Mode,
                    _ => {
                        println!("Please input a correct number");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("Please input a correct number");
                continue;
            }
        }
    }
}

fn main() {
    let vec = dbg!(get_input_vec());

    println!("Please, input action: \n1. mean\n2. median\n3. mode");

    match get_input_action() {
        Action::Mean => {
            let mut sum = 0;
            for num in &vec {
                sum += num;
            }
            println!("Mean is: {}", sum as f64/vec.len() as f64);
        }
        Action::Median => {
            let mut vec = vec;
            vec.sort();
            println!("Median is: {}", vec[vec.len()/2]);
        }
        Action::Mode => {
            let mut map = HashMap::new();
            for num in &vec {
                let count = map.entry(num).or_insert(0);
                *count += 1;
            }
            let mut num_count_vec: Vec<(i32, i32)> = Vec::new();
            for (k, v) in map {
                num_count_vec.push((*k,v));
            }
            num_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
            println!("Mode is: {}", num_count_vec[0].0);
        }
    }
}
