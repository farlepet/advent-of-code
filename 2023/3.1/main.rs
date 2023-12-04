use std::env;
use std::fs;

struct TriRow {
    line_sz: usize,
    lines:   [Vec<char>; 3]
}

fn check_char(ch: &char) -> bool {
    return !(ch.is_alphanumeric() || (ch == &'.'));
}

fn find_pns(trow: &TriRow) -> Vec<u32> {
    let mut nums = vec![];

    let mut i: usize = 0;
    while i < trow.line_sz {
        if trow.lines[1][i].is_digit(10) {
            /* This is nasty, but it works */
            let start = i;
            let mut j = start;
            while (j < trow.line_sz) && trow.lines[1][j].is_digit(10) {
                j += 1;
            }
            let end = j - 1;
            let num = trow.lines[1][start..(end+1)].iter().collect::<String>().parse::<u32>().unwrap();
            println!("Number: {num} ({start}..{end})");

            let real_start = if start > 0                  { start - 1 } else { start };
            let real_end   = (if end   < (trow.line_sz - 1) { end   + 1 } else { end }) + 1;

            if ((start > 0)                && check_char(&trow.lines[1][start - 1])) ||
               ((end < (trow.line_sz - 1)) && check_char(&trow.lines[1][end + 1])) ||
               trow.lines[0][real_start..real_end].into_iter().any(check_char) ||
               trow.lines[2][real_start..real_end].into_iter().any(check_char) {
                nums.push(num);
                println!("Added");
            } else {
                println!("{}, {}", trow.lines[0][real_start..real_end].iter().collect::<String>(), trow.lines[2][real_start..real_end].iter().collect::<String>());
            }
            i = end;
        }

        i += 1;
    }

    return nums;
}

fn next_line(trow: &TriRow, line: &Vec<char>) -> TriRow {
    return TriRow {
        line_sz: trow.line_sz,
        lines: [
            trow.lines[1].to_vec(),
            trow.lines[2].to_vec(),
            line.to_vec()
        ]
    };
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("Must pass in input file as an argument!");
        std::process::exit(-1);
    }
    let filename: &String  = &args[1];

    let mut sum = 0;

    let mut all_nums: Vec<u32> = vec![];

    /* Could probably convert to Vec<Vec<char>> here */
    let lines: Vec<String> = fs::read_to_string(filename).unwrap().lines().map(String::from).collect();

    let mut trow = TriRow {
        line_sz: lines[0].len(),
        lines: [
            vec!['.'; lines[0].len()],
            vec!['.'; lines[0].len()],
            lines[0].chars().collect()
        ]
    };

    let mut i = 1;
    while i < lines.len() {
        trow = next_line(&trow, &lines[i].chars().collect());
        all_nums.extend(find_pns(&trow));

        i += 1;
    }
    trow = next_line(&trow, &vec!['.'; lines[0].len()]);
    all_nums.extend(find_pns(&trow));



    all_nums.iter().for_each(|x| sum += x);

    println!("Result: {sum}");

    std::process::exit(0);
}
