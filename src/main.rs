use std::fs;

const INPUT_1_1: &str = "./inputs/1_1.txt";
const INPUT_2_1: &str = "./inputs/2_1.txt";

fn main() {
    aoc_1();
    aoc_2();
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
