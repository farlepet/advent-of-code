use std::env;
use std::fs;

struct Readings {
    derivatives: Vec<Vec<i32>>
}


fn parse_line(line: &str) -> Readings {
    return Readings {
        derivatives: vec![
            line.split_whitespace().map(|val| val.parse::<i32>().unwrap()).collect()
        ]
    };
}

fn calc_derivatives(readings: &mut Readings) {
    while !readings.derivatives.last().unwrap().iter().all(|&val| val == 0) {
        readings.derivatives.push(
            readings.derivatives.last().unwrap().windows(2).map(|vals| vals[1] - vals[0]).collect()
        );
    }
}

fn calc_prev(readings: &mut Readings) {
    let mut newvec: Vec<Vec<i32>> = vec![];

    newvec.push(vec![0]);

    for i in (0..(readings.derivatives.len()-1)).rev() {
        let mut invec = readings.derivatives[i].clone();

        invec.insert(0,
            invec.first().unwrap() - newvec.first().unwrap().first().unwrap()
        );

        newvec.insert(0, invec);
    }

    readings.derivatives = newvec;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut all_readings: Vec<Readings> = vec![];

    for line in fs::read_to_string(filename).unwrap().lines() {
        let mut readings = parse_line(line);
        calc_derivatives(&mut readings);
        calc_prev(&mut readings);
        all_readings.push(readings);
    }

    let mut sum = 0;

    for readings in all_readings {
        println!("{:?}", readings.derivatives[0]);

        sum += readings.derivatives[0].first().unwrap();
    }

    println!("Result: {}", sum);

    std::process::exit(0);
}
