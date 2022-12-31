use std::collections::VecDeque;
use std::{env, fs};

const INPUT_1_1: &str = "./inputs/1_1.txt";
const INPUT_2_1: &str = "./inputs/2_1.txt";
const INPUT_3_1: &str = "./inputs/3_1.txt";
const INPUT_4_1: &str = "./inputs/4_1.txt";
const INPUT_5_1: &str = "./inputs/5_1.txt";
const INPUT_6_1: &str = "./inputs/6_1.txt";

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    // aoc_1();
    // aoc_2();
    // aoc_3();
    // aoc_4();
    // aoc_5();
    aoc_6();
}

fn aoc_1() {
    let contents = fs::read_to_string(INPUT_1_1).expect("File access error");

    let mut elves = Vec::<i32>::new();
    let mut cal: i32 = 0;

    for element in contents.split("\n") {
        // println!("{element}");
        if element.trim() != "" {
            let num: i32 = match element.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            cal += num;
        } else {
            // println!("cal: {cal}");
            // println!("max_cal: {max_cal}");
            elves.push(cal);
            cal = 0;
        }
    }
    elves.sort();
    println!("1_1: {}", elves[elves.len() - 1]);
    let mut top_three_elves: i32 = 0;
    for i in elves.len() - 3..elves.len() {
        top_three_elves += elves[i];
    }
    println!("1_2: {}", top_three_elves);
}

fn aoc_2() {
    let contents: String = fs::read_to_string(INPUT_2_1).expect("File access error");

    let mut scores_1 = 0;
    let mut scores_2 = 0;
    for element in contents.split("\n") {
        // println!("{element}");
        let (score_1, score_2) = scoring(&element[..]);
        scores_1 += score_1;
        scores_2 += score_2;
    }
    println!("2_1: {scores_1}");
    println!("2_2: {scores_2}");

    fn scoring(round: &str) -> (i32, i32) {
        let mut score_1 = 0;
        let mut score_2 = 0;
        let mut plays = Vec::<char>::new();
        for c in round.chars() {
            if c != ' ' {
                plays.push(c)
            }
            // println!("{plays:#?}")
        }

        if plays[1] == 'X' {
            score_1 += 1;
        } else if plays[1] == 'Y' {
            score_1 += 2;
        } else if plays[1] == 'Z' {
            score_1 += 3;
        }

        if plays[0] == 'A' {
            if plays[1] == 'X' {
                score_1 += 3;
            } else if plays[1] == 'Y' {
                score_1 += 6;
            }
        } else if plays[0] == 'B' {
            if plays[1] == 'Y' {
                score_1 += 3;
            } else if plays[1] == 'Z' {
                score_1 += 6;
            }
        } else if plays[0] == 'C' {
            if plays[1] == 'X' {
                score_1 += 6;
            } else if plays[1] == 'Z' {
                score_1 += 3;
            }
        }

        if plays[0] == 'A' {
            if plays[1] == 'X' {
                score_2 += 3;
            } else if plays[1] == 'Y' {
                score_2 += 4;
            } else if plays[1] == 'Z' {
                score_2 += 8;
            }
        } else if plays[0] == 'B' {
            if plays[1] == 'X' {
                score_2 += 1;
            } else if plays[1] == 'Y' {
                score_2 += 5;
            } else if plays[1] == 'Z' {
                score_2 += 9;
            }
        } else if plays[0] == 'C' {
            if plays[1] == 'X' {
                score_2 += 2;
            } else if plays[1] == 'Y' {
                score_2 += 6;
            } else if plays[1] == 'Z' {
                score_2 += 7;
            }
        }
        return (score_1, score_2);
    }
}

fn aoc_3() {
    let contents: String = fs::read_to_string(INPUT_3_1).expect("File access error.");
    // println!("contents: {contents}");

    let mut backpacks = Vec::<&str>::new();

    let mut scores = 0;

    // Consolidate backpacks into a vector
    for backpack in contents.split("\n") {
        // println!("backpack: {backpack}");
        backpacks.push(backpack);
    }

    for backpack in &backpacks[..] {
        scores += score(&backpack[..]);
    }
    println!("3_1: {scores}");

    // Calculate score for AOC2022 3_2
    scores = 0;
    let mut i = 0;
    while i < (&backpacks[..].len() - 1) {
        scores += score_2(backpacks[i], backpacks[i + 1], backpacks[i + 2]);
        i += 3;
    }
    println!("3_2: {scores}");
    // First attempt: 382 - too low

    fn score(backpack: &str) -> i32 {
        let compartment_size = backpack.len() / 2;
        let compartment_1 = &backpack[..compartment_size];
        let compartment_2 = &backpack[compartment_size..];
        // println!("backpack: {backpack}");
        // println!("compartment 1: {compartment_1}");
        // println!("compartment 2: {compartment_2}");
        let mut duplicate_item = ' ';
        'comparison_loop: for item in compartment_1.chars() {
            for item_2 in compartment_2.chars() {
                if item == item_2 {
                    duplicate_item = item;
                    break 'comparison_loop;
                }
            }
        }
        let duplicate_item_points = points(duplicate_item);
        if duplicate_item_points == 0 {
            println!("Error in backpack duplicate item.");
            println!("duplicate item: {duplicate_item}");
        };
        // println!("duplicate item: {duplicate_item}");
        // println!("points: {points}");
        return duplicate_item_points;
    }

    fn score_2(backpack_1: &str, backpack_2: &str, backpack_3: &str) -> i32 {
        for c in backpack_1.chars() {
            for d in backpack_2.chars() {
                if c == d {
                    for e in backpack_3.chars() {
                        if c == e {
                            return points(c);
                        }
                    }
                }
            }
        }
        println!("2_1 scoring error: no duplicate item found");
        return -1; // Error value
    }

    fn points(item: char) -> i32 {
        match item {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _ => 0,
        }
    }
}

fn aoc_4() {
    let contents: String = fs::read_to_string(INPUT_4_1).expect("File access error");

    // Create vector of assignments
    let mut assignments = Vec::<&str>::new();
    for assignment in contents.split("\n") {
        if assignment != "" {
            assignments.push(assignment);
        }
    }
    // println!("{assignments:#?}");

    let mut score_1 = 0;
    let mut score_2 = 0;
    let mut assignment_pair = Vec::<&str>::new();
    let mut a_1 = Vec::<i32>::new();
    let mut a_2 = Vec::<i32>::new();

    for assignment in &assignments[..] {
        for a in assignment.split(",") {
            assignment_pair.push(a);
        }
        // println!("assignment pair: {assignment_pair:#?}");
        (a_1, a_2) = parse_assignments(assignment_pair[0], assignment_pair[1]);
        score_1 += scoring_1(&a_1[..], &a_2[..]);
        score_2 += scoring_2(&a_1[..], &a_2[..]);
        assignment_pair.clear();
    }

    println!("4_1: {score_1}");
    println!("4_2: {score_2}");

    fn parse_assignments(assignment_1: &str, assignment_2: &str) -> (Vec<i32>, Vec<i32>) {
        let mut assign_1_vec = Vec::<i32>::new();
        let mut assign_2_vec = Vec::<i32>::new();

        for i in assignment_1.trim().split("-") {
            let id = match i.parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            if id == 0 {
                println!("ID error");
            } else {
                assign_1_vec.push(id);
            }
        }

        for i in assignment_2.trim().split("-") {
            let id = match i.parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            if id == 0 {
                println!("ID error");
            } else {
                assign_2_vec.push(id);
            }
        }

        return (assign_1_vec, assign_2_vec);
    }
    // println!("assignment_1: {assign_1_vec:#?}");
    // println!("assignment_2: {assign_2_vec:#?}");

    fn scoring_1(assign_1_vec: &[i32], assign_2_vec: &[i32]) -> i32 {
        if assign_1_vec[0] >= assign_2_vec[0] && assign_1_vec[1] <= assign_2_vec[1] {
            return 1;
        } else if assign_2_vec[0] >= assign_1_vec[0] && assign_2_vec[1] <= assign_1_vec[1] {
            return 1;
        } else {
            return 0;
        }
    }

    fn scoring_2(a_1: &[i32], a_2: &[i32]) -> i32 {
        // println!("scoring 2: {a_1:?}, {a_2:?}");
        if a_1[0] >= a_2[0] && a_1[0] <= a_2[1] {
            return 1;
        } else if a_1[1] >= a_2[0] && a_1[1] <= a_2[1] {
            return 1;
        } else if a_2[0] >= a_1[0] && a_2[0] <= a_1[1] {
            return 1;
        } else if a_2[1] >= a_1[0] && a_2[1] <= a_1[1] {
            return 1;
        } else {
            return 0;
        }
    }
}

fn aoc_5() {
    let contents: String = fs::read_to_string(INPUT_5_1).expect("File access error");

    let mut num_stacks = 0;

    // Extract the number of stacks of crates
    for line in contents.trim().split("\n") {
        if line.contains("move") {
            continue;
        }
        if line.contains('1') {
            for element in line.trim().split(" ") {
                if element != "" {
                    num_stacks += 1;
                }
            }
        } else {
            continue;
        }
    }

    // Create vector mapping of crate positions
    let mut stacks = Vec::<Vec<char>>::new();
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }
    // println!("{stacks:?}");

    // Extract crate labels
    for line in contents.trim().split("\n") {
        if line.contains("move") {
            continue;
        } else if line.contains('1') {
            continue;
        } else {
            for (i, c) in line.chars().enumerate() {
                if c.is_alphabetic() {
                    let stack_num = (i - 1) / 4;
                    stacks[stack_num].push(c);
                }
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    // println!("{stacks:?}");

    // Process moves
    let mut stack_height: usize = 0;
    for line in contents.trim().split("\n") {
        if line.contains("move") {
            let (moves, source, destination) = process(&line[..]);
            // println!("{}:{}:{}", moves, source, destination);
            // println!("{stacks:?}");
            // println!("source: {source}, destination: {destination}, moves: {moves}");
            // for _ in 0..moves {
            //     stack_height = stacks[source].len();
            //     // println!("stack height: {stack_height}");
            //     let tail = stacks[source].split_off(stack_height - 1);
            //     stacks[destination].push(tail[0])
            // }
            stack_height = dbg!(stacks[source].len());
            dbg!(moves);
            let tail = stacks[source].split_off(stack_height - moves);
            for element in &tail {
                stacks[destination].push(*element);
            }
        }
    }

    // Final crate position
    let mut top_crates = Vec::<char>::new();
    for stack in &stacks {
        stack_height = dbg!(stack.len());
        top_crates.push(stack[stack_height - 1]);
    }
    println!("5_2: {top_crates:?}");

    fn process(line: &str) -> (usize, usize, usize) {
        let mut moves: usize = 0;
        let mut source: usize = 0;
        let mut destination: usize = 0;
        for (i, word) in line.trim().split(" ").enumerate() {
            if word.parse::<i32>().is_ok() {
                if i == 1 {
                    moves = match word.parse() {
                        Ok(num) => num,
                        Err(_) => 0,
                    };
                } else if i == 3 {
                    source = match word.parse() {
                        Ok(num) => num,
                        Err(_) => 0,
                    };
                } else if i == 5 {
                    destination = match word.parse() {
                        Ok(num) => num,
                        Err(_) => 0,
                    };
                }
            }
        }
        return (moves, source - 1, destination - 1);
    }
}

fn aoc_6() {
    let signal: String = fs::read_to_string(INPUT_6_1).expect("File access error.");
    let marker_length = 14;
    for i in 0..(signal.len() - marker_length) {
        let mut marker = Vec::<char>::new();
        for c in signal[i..i + marker_length].chars() {
            marker.push(c);
        }
        if has_dup(&marker) == false {
            println!("{}", i + marker_length);
            break;
        }
    }

    fn has_dup(slice: &[char]) -> bool {
        for i in 1..slice.len() {
            if slice[i..].contains(&slice[i - 1]) {
                return true;
            }
        }
        false
    }
}
