use std::env;
use std::fs;

struct Map {
    data: Vec<Vec<char>>,
    start: (usize, usize)
}

fn parse_line(line: &str, map: &mut Map) {
    map.data.push(line.chars().collect());
    match line.find('S') {
        Some(idx) => {
            map.start = (idx, map.data.len() - 1)
        },
        None => {}
    }
}

fn find_next(map: &Map, pos: (usize, usize), prev: (usize, usize)) -> Option<(usize, usize)> {
    let tries: &[(i32, i32)];

    const accept: [char; 7] = ['|', '-', 'L', 'J', '7', 'F', 'S'];

    match map.data[pos.1][pos.0] {
        '|' => { tries = &[( 0, -1), ( 0,  1)]; },
        '-' => { tries = &[(-1,  0), ( 1,  0)]; },
        'L' => { tries = &[( 0, -1), ( 1,  0)]; },
        'J' => { tries = &[( 0, -1), (-1,  0)]; },
        '7' => { tries = &[( 0,  1), (-1,  0)]; },
        'F' => { tries = &[( 1,  0), ( 0,  1)]; },
        'S' => { tries = &[(-1, 0), (1, 0), (0, -1), (0, 1)]; },
        _   => { return None; }
    }

    for off in tries {
        let test = (
            (pos.0 as i32 + off.0),
            (pos.1 as i32 + off.1)
        );
        if (test.0 < 0) || (test.0 >= map.data[0].len() as i32) ||
           (test.1 < 0) || (test.1 >= map.data.len() as i32) {
           continue;
        }

        let newpos = (test.0 as usize, test.1 as usize);

        if newpos == prev {
            continue;
        };

        if !accept.contains(&map.data[newpos.1][newpos.0]) {
            continue;
        }

        return Some(newpos);
    }

    return None;
}

fn follow_maze(map: &Map) -> u32 {
    let mut pos  = map.start;
    let mut prev = pos;
    let mut len  = 0;

    while (len == 0) || (pos != map.start) {
        let next = find_next(map, pos, prev);
        match next {
            Some(newpos) => {
                prev = pos;
                pos  = newpos;
            },
            None => {
                println!("Bruh");
                return len;
            }
        }

        len += 1;
    }


    return len;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut map = Map { data: vec![], start: (0,0) };

    for line in fs::read_to_string(filename).unwrap().lines() {
        parse_line(line, &mut map);
    }

    let len = follow_maze(&map);

    println!("Total length: {}", len);
    println!("Answer: {}", len / 2);

    std::process::exit(0);
}
