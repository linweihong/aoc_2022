use aoc_2022::get_input;

const INPUT: &str = "./inputs/9_1.txt";

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn step(&mut self, s: &str) {
        if s == "U" {
            self.y += 1;
        } else if s == "D" {
            self.y -= 1;
        } else if s == "L" {
            self.x -= 1;
        } else if s == "R" {
            self.x += 1;
        }
    }

    fn diff(&self, p: &Point) -> (i32, i32) {
        return (p.x - self.x, p.y - self.y);
    }

    fn update(&mut self, p: &Point) {
        let change = self.diff(p);
        // # # # # #
        // # * * * #
        // # * x * #
        // # * * * #
        // # # # # #
        match change {
            (0, 2) => self.y += 1,
            (1, 2) => {
                self.x += 1;
                self.y += 1;
            }
            (2, 2) => {
                self.x += 1;
                self.y += 1;
            }
            (2, 1) => {
                self.x += 1;
                self.y += 1;
            }
            (2, 0) => self.x += 1,
            (2, -1) => {
                self.x += 1;
                self.y -= 1;
            }
            (2, -2) => {
                self.x += 1;
                self.y -= 1;
            }
            (1, -2) => {
                self.x += 1;
                self.y -= 1;
            }
            (0, -2) => self.y -= 1,
            (-1, -2) => {
                self.x -= 1;
                self.y -= 1;
            }
            (-2, -2) => {
                self.x -= 1;
                self.y -= 1;
            }
            (-2, -1) => {
                self.x -= 1;
                self.y -= 1;
            }
            (-2, 0) => self.x -= 1,
            (-2, 1) => {
                self.x -= 1;
                self.y += 1;
            }
            (-2, 2) => {
                self.x -= 1;
                self.y += 1;
            }
            (-1, 2) => {
                self.x -= 1;
                self.y += 1;
            }
            _ => (),
        }
    }

    fn report(&self) -> (i32, i32) {
        return (self.x, self.y);
    }
}

#[derive(Debug)]
struct Step {
    direction: String,
    steps: i32,
}

pub fn solve() {
    let steps = get_moves();
    let mut pointlist = vec![(0, 0)];
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    for s in &steps {
        for _ in 0..s.steps {
            head.step(&s.direction);
            tail.update(&head);
            pointlist.push(tail.report());
        }
    }
    pointlist.sort();
    pointlist.dedup();
    println!("9_1: {}", pointlist.len());
    let mut pointlist = vec![(0, 0)];
    let mut rope = Vec::<Point>::new();
    for _ in 0..10 {
        rope.push(Point { x: 0, y: 0 });
    }
    for s in &steps {
        for _ in 0..s.steps {
            rope[0].step(&s.direction);
            for i in 1..rope.len() {
                let leading_segment = rope[i - 1].to_owned();
                rope[i].update(&leading_segment);
                pointlist.push(rope[rope.len() - 1].report());
            }
        }
    }
    pointlist.sort();
    pointlist.dedup();
    println!("9_2: {}", pointlist.len());
}

fn get_moves() -> Vec<Step> {
    let inputs = get_input(INPUT);
    let mut steps = Vec::<Step>::new();
    for line in inputs.split("\n") {
        if line != "" {
            let mut step = Vec::<String>::new();
            for s in line.split(" ") {
                step.push(s.trim().to_string());
            }
            let step_struct = Step {
                direction: step[0].to_string(),
                steps: step[1].parse().unwrap(),
            };
            steps.push(step_struct);
        }
    }
    return steps;
}
