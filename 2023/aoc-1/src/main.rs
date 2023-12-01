/**
 * 
 */

use std::fs;
use substring::Substring;


fn check_line(x:String, charver:bool) -> u32 {
    let mut first_found = false;
    let mut first : u32 = 0;
    let mut last : u32 = 0;
    let spelled_nums = vec!["zero", "one", "two", "three",
			    "four", "five", "six", "seven",
			    "eight", "nine"];
    for  (i, c) in x.chars().enumerate() {
	let substr : String = x.substring(i,x.len()).to_string();
	if char::is_numeric(c) {
	    last = String::from(c).parse::<u32>().unwrap();
	    if !first_found {
		first_found = true;
		first = String::from(c).parse::<u32>().unwrap();
	    }
	} else if charver {
	    for (j, name) in spelled_nums.iter().enumerate() {
		
		if substr.starts_with(name) {
		    last = j as u32;
		    if !first_found {
			first = j as u32;
			first_found = true;
		    }
		}
	    }
	}
    }
    let first_v = first * 10;
    let last_v = last;
    return first_v + last_v;
}



fn main() {

    let contents = fs::read_to_string("aoc-input-2023-12-01.txt").unwrap();
    let mut sum : u32 = 0;
    let mut sum2 : u32 = 0;
    let split_contents = contents.split('\n');
    for line in split_contents {
	if line.len() > 0 {
	    sum = sum + check_line(line.to_string(), false);
	    sum2 = sum2 + check_line(line.to_string(), true);
	}
    }
    println!("Sum:          {}", sum);
    println!("Sum (part 2): {}", sum2);
}
