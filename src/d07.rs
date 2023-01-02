use std::collections::HashMap;
use std::fs;

const INPUT: &str = "./inputs/7_1.txt";

#[derive(Debug)]
    struct Database {
    dir: HashMap<String, Database>,
    size: i32,
}

impl Database {
    fn add_contents(&mut self, cwd: &[&str], contents: HashMap<String, Database>, cwd_file_sizes: i32) {
        if cwd.len() > 1 {
            self.dir.get_mut(cwd[0]).unwrap().add_contents(&cwd[1..], contents, cwd_file_sizes);
        } else {
            self.dir.get_mut(cwd[0]).unwrap().dir = contents;
            self.dir.get_mut(cwd[0]).unwrap().size = cwd_file_sizes;
        }
    }
}

pub fn create_file_structure() {
    let inputs = fs::read_to_string(INPUT).expect("File access error");
    let commands: Vec<_> = inputs.split("\n").collect();
    let mut db = Database {
        dir: HashMap::from([(
            "/".to_string(),
            Database {
                dir: HashMap::new(),
                size: 0,
            },
        )]),
        size: 0,
    };
    let mut current_working_directory = Vec::<&str>::new();
    let commands_len = commands.len();
    let mut i = 0;
    while i < commands_len {
        // todo: implement command parsing
        let command_raw = commands[i].trim();
        let command: Vec<_> = command_raw.split(" ").collect();
        if command[1] == "cd" {
            if command[2] == ".." {
                current_working_directory.remove(current_working_directory.len() - 1);
                i += 1;
            } else {
                current_working_directory.push(command[2]);
                i += 1;
            }
        } else if command[1] == "ls" {
            let (contents, file_sizes, skip_index) = ls(&commands, i);
            db.add_contents(&current_working_directory, contents, file_sizes);
            i += skip_index;
        }
    }
    dbg!(db);
}

fn ls(cmds: &Vec<&str>, i: usize) -> (HashMap<String, Database>, i32, usize) {
    let mut skip_index = 1;
    let mut contents = HashMap::<String, Database>::new();
    let mut cwd_file_sizes = 0;
    while i + skip_index < cmds.len() {
        let cmd_raw = cmds[i + skip_index].trim();
        let cmd: Vec<_> = cmd_raw.split(" ").collect();
        if cmd[0] == "dir" {
            contents.insert(
                cmd[1].to_string(),
                Database {
                    dir: HashMap::new(),
                    size: 0,
                },
            );
            skip_index += 1;
        } else if cmd[0] == "$" {
            return (contents, cwd_file_sizes, skip_index);
        } else {
            let size: i32 = match cmd[0].parse() {
                Ok(int) => int,
                _ => panic!("Invalid value for size provided"),
            };
            cwd_file_sizes += size;
            skip_index += 1;
        }
    }
    return (contents, cwd_file_sizes, skip_index);
}

// todo: implement filesize parsing
// todo: implement recursive function for calculating directory size
