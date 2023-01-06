use aoc_2022::get_input;

const INPUT: &str = "./inputs/10_1.txt";

#[derive(Debug)]
struct Clock {
    signal: i32,
    cycles: Vec<i32>,
}

impl Clock {
    fn noop(&mut self) {
        self.update();
    }

    fn addx(&mut self, v: i32) {
        self.update();
        self.signal += v;
        self.update();
    }

    fn update(&mut self) {
        self.cycles.push(self.signal);
    }

    fn strength(&self, n: i32) -> i32 {
        return n * self.cycles[(n as usize) - 1];
    }
}

pub fn solve() {
    let mut clock = Clock {
        cycles: vec![0],
        signal: 0,
    };
    let instructions = get_cycles();
    for instruction in &instructions {
        if instruction.contains("noop") {
            clock.noop();
        } else {
            let instruction_v: Vec<_> = instruction.split(" ").collect();
            let v: i32 = instruction_v[1].trim().parse().unwrap();
            clock.addx(v);
        }
    }
    let ans = clock.strength(20)
        + clock.strength(60)
        + clock.strength(100)
        + clock.strength(140)
        + clock.strength(180)
        + clock.strength(220);

    println!("10_1: {ans}"); // first attempt - 14140 - too low
}

fn get_cycles() -> Vec<String> {
    let mut cycles = Vec::<String>::new();
    let inputs = get_input(INPUT);
    for line in inputs.split("\n") {
        if line != "" {
            cycles.push(line.to_string());
        }
    }
    return cycles;
}
