advent_of_code::solution!(4);

const DIRS_8: [(isize, isize); 8] = [
    (-1, -1), (-1,  0), (-1,  1),
    ( 0, -1),           ( 0,  1),
    ( 1, -1), ( 1,  0), ( 1,  1),
];

pub fn make_grid(input: &str) -> Vec<Vec<char>> {
    let height: usize = input.lines().count();
    let width: usize = input.lines().next().unwrap().len();
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];
    
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        for c in line.chars() {
            grid[y][x] = c;
            x = x + 1;
        }
        y = y + 1;
    }

    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut roll_count: u64 = 0;
    let height: usize = input.lines().count();
    let width: usize = input.lines().next().unwrap().len();
    let grid= make_grid(input);

    for x in 0..(height) {
        for y in 0..(width) {
            if grid[x][y] == '@' {

                let mut num_rolls = 0;
                for (dx, dy) in DIRS_8 {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                
                    if nx < 0 || ny < 0 { continue; }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx >= height || ny >= width { continue; }
                
                    let val = grid[nx][ny];

                    if val == '@' {
                        num_rolls += 1;
                    }
                }
                if num_rolls < 4 {
                    roll_count += 1;
                }
            }
        }
    }
    println!("########### {} ###########", roll_count);
    Some(roll_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
