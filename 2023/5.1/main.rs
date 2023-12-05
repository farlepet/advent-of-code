use std::env;
use std::fs;
use std::ptr;

struct Mapping {
    src_start:  u32,
    dest_start: u32,
    range:      u32
}

struct Almanac {
    seeds:                Vec<u32>,
    seed2soil:            Vec<Mapping>,
    soil2fertilizer:      Vec<Mapping>,
    fertilizer2water:     Vec<Mapping>,
    water2light:          Vec<Mapping>,
    light2temperature:    Vec<Mapping>,
    temperature2humidity: Vec<Mapping>,
    humidity2location:    Vec<Mapping>,

    current: *mut Vec<Mapping>
}


fn parse_seeds(line: &str, alm: &mut Almanac) {
    /* Format:
     *   seeds: <s1> <s2> ... */

    let start  = line.find(':').unwrap() + 1;
    for seed in line[start..].split_whitespace() {
        alm.seeds.push(seed.parse::<u32>().unwrap());
    }
}

fn parse_map(line: &str, map: *mut Vec<Mapping>) {
    /* Format:
     *   <src_start> <src_end> <range> */
    let nums = line.split_whitespace().collect::<Vec<_>>();
    if nums.len() != 3 {
        println!("parse_map: nums.len() = {}", nums.len())
    }
    unsafe {
        (*map).push(Mapping {
            src_start:  nums[1].parse::<u32>().unwrap(),
            dest_start: nums[0].parse::<u32>().unwrap(),
            range:      nums[2].parse::<u32>().unwrap()
        });
    }
}


fn parse_line(line: &str, alm: &mut Almanac) {
    let split = line.split_whitespace().collect::<Vec<_>>();
    if split.len() < 1 {
        return;
    }

    match split[0] {
        "seeds:" => parse_seeds(line, alm),

        "seed-to-soil"            => alm.current = &mut alm.seed2soil,
        "soil-to-fertilizer"      => alm.current = &mut alm.soil2fertilizer,
        "fertilizer-to-water"     => alm.current = &mut alm.fertilizer2water,
        "water-to-light"          => alm.current = &mut alm.water2light,
        "light-to-temperature"    => alm.current = &mut alm.light2temperature,
        "temperature-to-humidity" => alm.current = &mut alm.temperature2humidity,
        "humidity-to-location"    => alm.current = &mut alm.humidity2location,

        _ => parse_map(line, alm.current)
    }
}

fn map_find(map: &Vec<Mapping>, value: u32) -> u32 {
    for i in map {
        if (value >= i.src_start) &&
           ((value - i.src_start) < i.range) {
            return i.dest_start + (value - i.src_start);
        }
    }

    return value;
}

fn find_loc(alm: &Almanac, seed: u32) -> u32 {
    let soil        = map_find(&alm.seed2soil,            seed);
    let fertilizer  = map_find(&alm.soil2fertilizer,      soil);
    let water       = map_find(&alm.fertilizer2water,     fertilizer);
    let light       = map_find(&alm.water2light,          water);
    let temperature = map_find(&alm.light2temperature,    light);
    let humidity    = map_find(&alm.temperature2humidity, temperature);
    let location    = map_find(&alm.humidity2location,    humidity);

    println!("{} {} {} {} {} {} {} {}", seed, soil, fertilizer, water, light, temperature, humidity, location);

    return location;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut sum = 0;

    let mut alm = Almanac {
        seeds:                vec![],
        seed2soil:            vec![],
        soil2fertilizer:      vec![],
        fertilizer2water:     vec![],
        water2light:          vec![],
        light2temperature:    vec![],
        temperature2humidity: vec![],
        humidity2location:    vec![],
        current:              ptr::null_mut()
    };

    /* Could probably convert to Vec<Vec<char>> here */
    for line in fs::read_to_string(filename).unwrap().lines() {
        parse_line(line, &mut alm);
    }

    for map in &alm.seed2soil {
        println!("{} {} {}", map.src_start, map.dest_start, map.range);
    }

    let locations = alm.seeds.iter().map(|seed| find_loc(&alm, *seed)).collect::<Vec<u32>>();
    println!("Locs: {:?}", locations);

    println!("Result: {}", locations.iter().min().unwrap());

    std::process::exit(0);
}
