use std::env;
use std::fs;
use std::collections::HashMap;

struct Map {
    instructions: Vec<char>,
    mapping:      HashMap<String, [String; 2]>
}


fn parse_line(line: &str, map: &mut Map) {
    let key   = line[0..3].to_string();
    let left  = line[7..10].to_string();
    let right = line[12..15].to_string();

    map.mapping.insert(key, [left, right]);
}

fn solve(map: &Map) -> u32 {
    let mut steps = 0;

    let mut pos: String = String::from("AAA");

    while pos != "ZZZ" {
        println!("{}", pos);
        let choices: &[String; 2] = map.mapping.get(&pos).unwrap();
        let idx = if map.instructions[steps % map.instructions.len()] == 'L' {0} else {1};
        pos = String::from(&choices[idx]);
        steps += 1;
    }

    return steps as u32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut map = Map {
        instructions: vec![],
        mapping:      HashMap::new()
    };

    for (idx, line) in fs::read_to_string(filename).unwrap().lines().enumerate() {
        if idx == 0 {
            map.instructions = line.chars().collect();
        } else if idx > 1 {
            parse_line(line, &mut map);
        }
    }

    let steps = solve(&map);

    println!("Steps: {}", steps);

    std::process::exit(0);
}
