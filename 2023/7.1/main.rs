use std::env;
use std::fs;

struct Hand {
    cards: Vec<char>,
    bid:   u32,
    /* <rank><abcde> */
    score: u32
}

fn get_hand_score(hand: &Hand) -> u32 {
    let card_pos: Vec<char> = vec![ '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A' ];

    let mut score = 0;
    for card in &hand.cards {
        score = score * 16;
        score += card_pos.iter().position(|crd| crd == card).unwrap() as u32;
    }

    let mut tmp_cards = hand.cards.to_vec();
    tmp_cards.sort();
    tmp_cards.dedup();
    let uniq = tmp_cards.len();

    let cnts = tmp_cards.iter().map(|card| hand.cards.iter().filter(|&crd| crd == card).count() as u32).collect::<Vec<u32>>();

    match uniq {
        1 => {
            /* Five of a kind */
            score += 0x700000;
        },
        2 => {
            if cnts.contains(&4) {
                /* Four of a kind */
                score += 0x600000;
            } else {
                /* Full house */
                score += 0x500000;
            }
        },
        3 => {
            if cnts.contains(&3) {
                /* Three of a kind */
                score += 0x400000;
            } else {
                /* Two pair */
                score += 0x300000;
            }
        },
        4 => {
            /* One pair */
            score += 0x200000;
        },
        5 => {
            /* High card */
            score += 0x100000;
        }

        _ => score += 0
    }

    return score;
}

fn parse_line(line: &str) -> Hand {
    let split = line.split_whitespace().collect::<Vec<_>>();

    let mut hand = Hand {
        cards: split[0].chars().collect::<Vec<char>>(),
        bid:   split[1].parse::<u32>().unwrap(),
        score: 0
    };
    hand.score = get_hand_score(&hand);

    return hand;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut hands = fs::read_to_string(filename).unwrap().lines().map(|line| parse_line(line)).collect::<Vec<Hand>>();

    for hand in &hands {
        println!("cards: {:?}, bid: {}, score: {:x}", hand.cards, hand.bid, hand.score);
    }

    println!();

    hands.sort_by(|a, b| a.score.cmp(&b.score));

    let mut sum = 0;

    for (idx, hand) in hands.iter().enumerate() {
        let score = hand.bid * (idx as u32 + 1);
        println!("cards: {:?}, bid: {}, score: {:x} -> {}", hand.cards, hand.bid, hand.score, score);
        sum += score;
    }

    println!("Result: {}", sum);

    std::process::exit(0);
}
