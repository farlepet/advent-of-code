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

fn solve(map: &Map) {
    let begin: Vec<&String> = map.mapping.keys().filter(|node| node.chars().collect::<Vec<char>>()[2] == 'A').collect();

    println!("Calculate the Least Common Multiple (LCM) of the following values:");

    for node in &begin {
        let mut stp: usize = 0;
        let mut cnode    = node.clone();

        while !(cnode.chars().collect::<Vec<char>>()[2] == 'Z') {
            let path = if map.instructions[stp % map.instructions.len()] == 'L' {0} else {1};

            cnode = &map.mapping.get(&cnode.clone()).unwrap()[path];

            stp += 1;

        }

        let oldstp = stp;

        while (oldstp == stp) || (cnode.chars().collect::<Vec<char>>()[2] != 'Z') {
            let path = if map.instructions[stp % map.instructions.len()] == 'L' {0} else {1};

            cnode = &map.mapping.get(&cnode.clone()).unwrap()[path];

            stp += 1;
        }

        /* NOTE: This only works because the recurrance is actually equal to the
         * number of steps from the initial xxA to the first xxZ. */
        println!("Recurrance: {}", stp - oldstp);
    }

    /* Brute force. With my puzzle input, this would have taken nearly 14 trillion
     * iterations. */
    /*
    while !pos.iter().all(|node| node.chars().collect::<Vec<char>>()[2] == 'Z') {
        let path = if map.instructions[steps % map.instructions.len()] == 'L' {0} else {1};

        pos = pos.iter().map(|&node| &map.mapping.get(node).unwrap()[path]).collect();

        steps += 1;
    }
    */
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

    solve(&map);

    std::process::exit(0);
}
