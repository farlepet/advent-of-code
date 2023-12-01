use std::env;
use std::fs;

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
        for ch in line.chars() {
            if ch.is_numeric() {
                lineval = (ch as u32 - '0' as u32) * 10;
                break;
            }
        }
        for ch in line.chars().rev() {
            if ch.is_numeric() {
                lineval += ch as u32 - '0' as u32;
                break;
            }
        }
        sum += lineval;
    }

    println!("Result: {sum}");

    std::process::exit(0);
}
