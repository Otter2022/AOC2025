advent_of_code::solution!(4);

pub fn make_grid(input: &str) -> Vec<Vec<char>> {
    let height: usize = input.lines().count();
    let width: usize = input.lines().next().unwrap().len();
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];

    for line in input.lines() {
        for c in line.chars() {

        }
    }

    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    None
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
