use std::env;
use std::fs;

struct Race {
    time:     u128,
    distance: u128
}

fn parse_line(line: &str, races: &mut Vec<Race>) {
    let split = line.split_whitespace().collect::<Vec<_>>();
    if split.len() < 1 {
        return;
    }
    let start  = line.find(':').unwrap() + 1;

    match split[0] {
        "Time:" => {
            //let time: String = String::from(&line[start..]).retain(|c| c.is_whitespace());
            let time = &line[start..].chars().filter(|c| !c.is_whitespace()).collect::<String>();
            println!("{time}");
            races.push(Race { time: time.parse::<u128>().unwrap(), distance: 0 });
        },
        "Distance:" => {
            let dist = &line[start..].chars().filter(|c| !c.is_whitespace()).collect::<String>();
            println!("{dist}");
            races[0].distance = dist.parse::<u128>().unwrap();
        }

        _ => return
    }
}

fn test_time(race: &Race, time: u128) -> bool{
    if (race.time - time) * time > race.distance {
        return true;
    }

    return false;
}

fn get_ways_to_win(race: &Race) -> u64 {
    /* Come up with guesses using quadratic formula: */
    let root = f64::sqrt(((race.time * race.time) - 4*race.distance) as f64) as u128;
    let mut guess_low  = (race.time - root) / 2;
    let mut guess_high = (race.time + root) / 2;

    println!("{guess_low}, {guess_high}");

    /* NOTE: This correction might not actually be necessary, and may be a
     * misunderstanding caused by a typo earlier. */

    /* Correct low guess: */
    if test_time(race, guess_low) {
        while test_time(race, guess_low - 1) {
            guess_low -= 1;
        }
    } else {
        while !test_time(race, guess_low) {
            guess_low += 1;
        }
    }

    /* Correct high guess: */
    if test_time(race, guess_high) {
        while test_time(race, guess_high) {
            guess_high += 1;
        }
    } else {
        while !test_time(race, guess_high - 1) {
            guess_high -= 1;
        }
    }

    return (guess_high - guess_low).try_into().unwrap();
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
