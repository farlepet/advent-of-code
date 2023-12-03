use std::env;
use std::fs;
use regex::Regex;

struct Cubes {
    red:   u32,
    green: u32,
    blue:  u32
}

struct Game {
    num:   u32,
    pulls: Vec<Cubes>
}

fn game_possible(game: &Game, max: &Cubes) -> bool {
    for pull in &game.pulls {
        if (pull.red   > max.red)   ||
           (pull.green > max.green) ||
           (pull.blue  > max.blue) {
            return false;
        }
    }

    return true;
}

fn parse_line(line: &str) -> Option<Game> {
    let mut game = Game{
        num:   0,
        pulls: vec![]
    };

    let game_re  = Regex::new(r"Game ([0-9]*)").unwrap();
    let game_cap = game_re.captures(line).unwrap();
    if game_cap.len() < 2 {
        return None;
    }
    game.num = game_cap[1].parse::<u32>().unwrap();

    let red_re   = Regex::new(r".*?([0-9]*) red").unwrap();
    let green_re = Regex::new(r".*?([0-9]*) green").unwrap();
    let blue_re  = Regex::new(r".*?([0-9]*) blue").unwrap();

    let mut begin  = line.find(':').unwrap() + 1;

    let mut finish = false;

    while !finish {
        let substr = &line[begin..];

        let mut pull = Cubes {
            red:   0,
            green: 0,
            blue:  0
        };

        let _end = substr.find(';');
        let mut end = 0;
        match _end {
            Some(n) => {
                begin += n + 1;
                end    = n;
            },
            None    => {
                end    = substr.len();
                finish = true;
            }
        }
        let ssubstr = &substr[..end];

        if let Some(cap) = red_re.captures(ssubstr) {
            if cap.len() > 1 {
                pull.red = cap[1].parse::<u32>().unwrap();
            }
        }
        if let Some(cap) = green_re.captures(ssubstr) {
            if cap.len() > 1 {
                pull.green = cap[1].parse::<u32>().unwrap();
            }
        }
        if let Some(cap) = blue_re.captures(ssubstr) {
            if cap.len() > 1 {
                pull.blue = cap[1].parse::<u32>().unwrap();
            }
        }

        game.pulls.push(pull);
    }

    return Some(game);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut sum = 0;

    let criteria = Cubes {
        red:   12,
        green: 13,
        blue:  14
    };

    // Horribly inefficient
    for line in fs::read_to_string(filename).unwrap().lines() {
        let game = parse_line(line).unwrap();
        if game_possible(&game, &criteria) {
            sum += game.num;
        }
    }

    println!("Result: {sum}");

    std::process::exit(0);
}
