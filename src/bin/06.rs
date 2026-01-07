advent_of_code::solution!(6);


pub fn part_one(input: &str) -> Option<u64> {
    let height = input.lines().count();
    let associated_operators: Vec<&str> = input.lines().nth(height - 1).unwrap().split_whitespace().collect();
    let mut results: Vec<u64> = input.lines().nth(0).unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
    // println!("{:?}", associated_operators);

    for i in 1..height-1 {
        let cur: Vec<u64> = input.lines().nth(i).unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
        for x in 0..cur.len() {
            if associated_operators[x] == "*" {
                results[x] = results[x] * cur[x];
            } else {
                results[x] = results[x] + cur[x];
            }
        }
    }

    // println!("{:?}", results);
    let ans: u64 = results.iter().sum();

    Some(ans)
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
