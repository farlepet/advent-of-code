use std::env;
use std::fs;

struct Card {
    card_n:  u32,
    winning: Vec<u32>,
    numbers: Vec<u32>
}

fn get_num(line: &str, start: usize) -> u32{
    let chars: Vec<char> = line.chars().collect();

    let mut end = start;
    while chars[end].is_digit(10) && end < line.len() {
        end += 1;
    }

    return line[start..end].parse::<u32>().unwrap();
}

fn parse_card(line: &str) -> Card {
    /* Format:
     *   Card <card_n>: <w1> <w2> ... | <n1> <n2> ... */

    let mut card = Card {
        card_n:  0,
        winning: vec![],
        numbers: vec![]
    };

    let chars: Vec<char> = line.chars().collect();

    let mut n_pos = 0;
    while !chars[n_pos].is_digit(10) {
        n_pos += 1;
    }

    card.card_n = get_num(line, n_pos);

    let win_start  = line.find(':').unwrap() + 1;
    let num_start  = line.find('|').unwrap() + 1;

    let win_str = &line[win_start..(num_start-2)];
    let num_str = &line[num_start..];

    for num in win_str.split_whitespace() {
        card.winning.push(num.parse::<u32>().unwrap());
    }
    for num in num_str.split_whitespace() {
        card.numbers.push(num.parse::<u32>().unwrap());
    }

    return card;
}

fn get_winnings(card: Card) -> u32 {
    let mut val = 0;

    for i in card.winning {
        if card.numbers.contains(&i) {
            if val > 0 {
                val <<= 1;
            } else {
                val = 1;
            }
        }
    }

    return val;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut sum = 0;

    /* Could probably convert to Vec<Vec<char>> here */
    for line in fs::read_to_string(filename).unwrap().lines() {
        let card = parse_card(&line);
        sum += get_winnings(card);
    }

    println!("Result: {sum}");

    std::process::exit(0);
}
