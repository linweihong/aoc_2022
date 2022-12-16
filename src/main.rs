use std::fs;

const INPUT_1_1: &str = "./inputs/1_1.txt";

fn main() {
    aoc_1();
}

fn aoc_1() {
    let contents = fs::read_to_string(INPUT_1_1).expect("Should have been able to read the file");

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
    println!("aoc_1_1 - max_cal: {}", elves[elves.len() - 1]);
    let mut top_three_elves: i32 = 0;
    for i in elves.len() - 3..elves.len() {
        top_three_elves += elves[i];
    }
    println!("aoc_1_2 - top three elves: {}", top_three_elves);
}
