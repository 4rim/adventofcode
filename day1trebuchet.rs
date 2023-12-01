use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main () {
    let mut stack: Vec<i32> = Vec::new();
    let mut ret_vec: Vec<i32> = Vec::new();
    let mut combined: i32;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes iterator, prints a string (optional)
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                for ch in ip.chars() {
                    if ch.is_numeric() {
                        let mut num = ch as i32;
                        num -= 48;
                        stack.push(num);
                        // println!("stack: {:?}\n", stack);
                    }
                }
                if stack.len() == 1 {
                    let mut ret: i32 = *stack.first().unwrap();
                    let tens = ret * 10;
                    ret += tens;
                    ret_vec.push(ret);
                } else {
                    let mut head: i32 = *stack.first().unwrap();
                    let tail: i32 = *stack.last().unwrap();

                    head *= 10;
                    combined = head + tail;
                    ret_vec.push(combined);
                }
                stack.clear();
            }
        }
    }

    // println!("{:?}\n", ret_vec);
    let sum: i32 = ret_vec.iter().sum();

    println!("The sum of all calibration values is: {}", sum);

}

// Returns an iterator to the reader of the lines of file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

