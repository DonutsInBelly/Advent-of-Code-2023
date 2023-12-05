use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let mut result = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let char_array: Vec<char> = line.expect("no characters!").chars().collect();
            let temp_char_array = char_array.clone();
            let char_array_rev = temp_char_array.iter().rev();

            let mut digit1 = char::from_u32(0);
            let mut digit2 = char::from_u32(0);
            for c in char_array {
                if (c.is_digit(10)) {
                    digit1 = Some(c);
                    break;
                }
            }

            for x in char_array_rev {
                if (x.is_digit(10)) {
                    digit2 = Some(*x);
                    break;
                }
            }

            let mut total_str:String = digit1.unwrap().to_string().to_owned();
            total_str.push_str(&digit2.unwrap().to_string());
            let total: i32 = total_str.parse().unwrap();
            result += total;
        }
    }
    println!("{}", result);
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}