use std::env;
use std::fs;

struct Race {
    time:     u32,
    distance: u32
}

fn parse_line(line: &str, races: &mut Vec<Race>) {
    let split = line.split_whitespace().collect::<Vec<_>>();
    if split.len() < 1 {
        return;
    }
    let start  = line.find(':').unwrap() + 1;

    match split[0] {
        "Time:" => {
            races.append(&mut line[start..].split_whitespace().map(|time| Race { time: time.parse::<u32>().unwrap(), distance: 0 }).collect::<Vec<Race>>());
        },
        "Distance:" => {
            for (idx, dist) in line[start..].split_whitespace().enumerate() {
                races[idx].distance = dist.parse::<u32>().unwrap();
            }
        }

        _ => return
    }
}

fn get_ways_to_win(race: &Race) -> u32 {
    let mut count = 0;
    for i in 1..race.time {
        if (race.time - i) * i > race.distance {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut races: Vec<Race> = vec![];

    for line in fs::read_to_string(filename).unwrap().lines() {
        parse_line(line, &mut races);
    }

    let mut mul = 1;
    for race in &races {
        let wtw = get_ways_to_win(race);
        println!("Ways to win: {wtw}");
        mul *= wtw;
    }

    println!("Result: {mul}");

    std::process::exit(0);
}
