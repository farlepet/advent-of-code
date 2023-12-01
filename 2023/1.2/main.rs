use std::env;
use std::fs;

fn find_num(strn: &str) -> Option<u32> {
    let nums = [
        "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine"
    ];

    // Inefficient
    for (idx, num) in nums.iter().enumerate() {
        if strn.starts_with(num) {
            return Some(idx as u32);
        }
    }

    let ch = strn.chars().nth(0).unwrap();
    if ch.is_numeric() {
        let val = ch as u32 - '0' as u32;
        return Some(val);
    }

    return None;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut sum = 0;

    // Horribly inefficient
    for line in fs::read_to_string(filename).unwrap().lines() {
        let mut lineval = 0;
        for start in 0..line.len() {
            match find_num(&line[start..]) {
                Some(x) => {
                    lineval += x * 10;
                    break;
                },
                None => {}
            }
        }

        for start in (0..line.len()).rev() {
            match find_num(&line[start..]) {
                Some(x) => {
                    lineval += x;
                    break;
                },
                None => {}
            }
        }
        sum += lineval;
    }

    println!("Result: {sum}");

    std::process::exit(0);
}
