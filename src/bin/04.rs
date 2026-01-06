advent_of_code::solution!(4);

const DIRS_8: [(isize, isize); 8] = [
    (-1, -1), (-1,  0), (-1,  1),
    ( 0, -1),           ( 0,  1),
    ( 1, -1), ( 1,  0), ( 1,  1),
];

pub fn make_grid(input: &str) -> Vec<Vec<char>> {
    let height = input.lines().count();
    let width  = input.lines().next().unwrap().len();

    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];

    let mut row = 0;
    for line in input.lines() {
        let mut col = 0;
        for c in line.chars() {
            grid[row][col] = c;
            col += 1;
        }
        row += 1;
    }

    grid
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let height = input.lines().count();
    let width  = input.lines().next()?.len();
    let grid   = make_grid(input);

    let mut roll_count: u64 = 0;

    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == '@' {
                let mut num_rolls = 0;

                for (dr, dc) in DIRS_8 {
                    let nr = row as isize + dr;
                    let nc = col as isize + dc;

                    if nr < 0 || nc < 0 { continue; }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if nr >= height || nc >= width { continue; }

                    if grid[nr][nc] == '@' {
                        num_rolls += 1;
                    }
                }

                if num_rolls < 4 {
                    roll_count += 1;
                }
            }
        }
    }

    println!(" ########### {} ############## ", roll_count);
    Some(roll_count)
}

pub fn recurse_to_get_rolls(grid: &mut Vec<Vec<char>>, roll_count: u64, height: usize, width: usize) -> Option<u64> {
    

    let mut new_roll_count: u64 = roll_count;

    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == '@' {
                let mut num_rolls = 0;

                for (dr, dc) in DIRS_8 {
                    let nr = row as isize + dr;
                    let nc = col as isize + dc;

                    if nr < 0 || nc < 0 { continue; }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if nr >= height || nc >= width { continue; }

                    if grid[nr][nc] == '@' {
                        num_rolls += 1;
                    }
                }

                if num_rolls < 4 {
                    new_roll_count += 1;
                    grid[row][col] = '.';
                }
            }
        }
    }

    println!("# {} #", new_roll_count);
    print_grid(grid);
    if new_roll_count != roll_count {
        recurse_to_get_rolls(grid, new_roll_count, height, width)
    } else {
        Some(new_roll_count)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let height = input.lines().count();
    let width  = input.lines().next()?.len();
    let mut grid   = make_grid(input);
    recurse_to_get_rolls(&mut grid, 0, height, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
