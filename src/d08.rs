use aoc_2022::get_input;

const INPUT: &str = "./inputs/8_1.txt";

pub fn solve() {
    let grid = &create_grid();
    // dbg!(&grid);
    let visible_trees = get_visible_trees(&grid);
    println!("8_1: {visible_trees}"); // first attempt - 392 too low; second attempt - 1556 too low
                                      // FIXME: troubleshoot
}

fn create_grid() -> Vec<Vec<u32>> {
    let inputs = get_input(INPUT);
    let mut grid = Vec::<Vec<u32>>::new();
    for line in inputs.split("\n") {
        if line != "" {
            let mut row = Vec::<u32>::new();
            for c in line.trim().chars() {
                let i = c.to_digit(10).unwrap();
                row.push(i);
            }
            grid.push(row);
        }
    }
    return grid;
}

fn get_visible_trees(grid: &[Vec<u32>]) -> u32 {
    let mut visible_trees: u32 = 0;
    for (row, trees) in grid.iter().enumerate() {
        if row == 0 {
            visible_trees += trees.len() as u32;
        } else if row == grid.len() - 1 {
            visible_trees += trees.len() as u32;
        } else {
            visible_trees += 2;
            visible_trees += count_visible_trees(grid, row);
        }
    }
    return visible_trees;
}

fn count_visible_trees(grid: &[Vec<u32>], row: usize) -> u32 {
    let mut row_visible_trees: u32 = 0;
    for i in 1..grid[row].len() - 1 {
        let h = grid[row][i];
        let mut flag_north = true;
        let mut flag_south = true;
        let mut flag_east = true;
        let mut flag_west = true;
        for j in 0..i {
            if grid[row][j] >= h {
                flag_west = false;
                break;
            }
        }
        for j in (i + 1)..grid[row].len() {
            if grid[row][j] >= h {
                flag_east = false;
                break;
            }
        }
        for j in 0..row {
            if grid[j][i] >= h {
                flag_north = false;
                break;
            }
        }
        for j in (row + 1)..grid.len() {
            if grid[j][i] >= h {
                flag_south = false;
                break;
            }
        }
        if flag_north == true || flag_south == true || flag_east == true || flag_west == true {
            row_visible_trees += 1;
        }
    }
    return row_visible_trees;
}
