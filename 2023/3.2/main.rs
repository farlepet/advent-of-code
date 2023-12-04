use std::env;
use std::fs;

struct TriRow {
    line_sz: usize,
    lines:   [Vec<char>; 3]
}

fn get_num(line: &Vec<char>, pos: usize) -> u32 {
    let mut start = pos;
    let mut end   = pos;

    let mut i = pos;
    while line[i].is_digit(10) {
        start = i;
        if i == 0 {
            break;
        }
        i -= 1;
    }
    i = pos;
    while line[i].is_digit(10) {
        end = i;
        if i == (line.len() - 1) {
            break;
        }
        i += 1;
    }

    let num = line[start..(end+1)].iter().collect::<String>().parse::<u32>().unwrap();
    return num;
}

fn find_gr(trow: &TriRow) -> Vec<u32> {
    let mut gears = vec![];

    let mut i: usize = 0;
    while i < trow.line_sz {
        if trow.lines[1][i] == '*' {
            /* This is nasty, but it works */
            let pos = i;

            let start = if pos > 0                  { pos - 1 } else { pos };
            let end   = if pos < (trow.line_sz - 1) { pos + 1 } else { pos };

            let mut nums = vec![];

            if trow.lines[1][start].is_digit(10) {
                nums.push(get_num(&trow.lines[1], start));
            }
            if trow.lines[1][end].is_digit(10) {
                nums.push(get_num(&trow.lines[1], end));
            }

            let mut j = start;
            while j <= end {
                if trow.lines[0][j].is_digit(10) {
                    nums.push(get_num(&trow.lines[0], j));
                    /* Skip rest of number */
                    while (trow.lines[0][j].is_digit(10)) && (j <= end) {
                        j += 1;
                    }
                }
                j += 1;
            }
            j = start;
            while j <= end {
                if trow.lines[2][j].is_digit(10) {
                    nums.push(get_num(&trow.lines[2], j));
                    /* Skip rest of number */
                    while (trow.lines[2][j].is_digit(10)) && (j <= end) {
                        j += 1;
                    }
                }
                j += 1;
            }

            if nums.len() > 1 {
                if (nums.len() > 2) {
                    print!("Too many: ");
                    for i in &nums { print!("{i} "); }
                    println!();
                } else {
                    println!("Adding gear: {}, {} [{}]", nums[0], nums[1], nums.len());
                    let gear = nums[0] * nums[1];
                    gears.push(gear);
                }
            }

            i = end;
        }

        i += 1;
    }

    return gears;
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
        all_nums.extend(find_gr(&trow));

        i += 1;
    }
    trow = next_line(&trow, &vec!['.'; lines[0].len()]);
    all_nums.extend(find_gr(&trow));



    all_nums.iter().for_each(|x| sum += x);

    println!("Result: {sum}");

    std::process::exit(0);
}
