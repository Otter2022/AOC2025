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
 
// pub fn fmt_nums(input: &str) -> Vec<Vec<u64>> {
//     let height = input.lines().count();
//     let width = input.lines().nth(0).unwrap().split_whitespace().count();
//     let mut grid: Vec<Vec<u64>> = Vec::new();
//     let mut str_grid: Vec<Vec<String>> = Vec::new();
//     let associated_operators: Vec<&str> = input.lines().nth(height - 1).unwrap().split_whitespace().collect();

//     for _ in 0..width { str_grid.push(Vec::new()) }

//     for i in 0..height-1 {
//         let cur: Vec<String> = input.lines().nth(i).unwrap().split_whitespace().map(String::from).collect();
//         let mut y = 0;
//         for x in cur {
//             let mut nums: Vec<char> = x.chars().collect();
//             if associated_operators[y] == "*" {
//                 nums.reverse();
//             } 
//             let mut x = 0;
//             for num in nums {
//                 match str_grid[y].get(x) {
//                     Some(_) => str_grid[y][x].push(num),
//                     None => {
//                         str_grid[y].push(String::new());
//                         str_grid[y][x].push(num);
//                     },
//                 }
//                 x += 1;
//             }
//             y += 1;
//         }
//     }

//     grid = str_grid.iter().map(|row| {
//         row.iter()
//         .map(|s| s.parse::<u64>().unwrap())
//         .collect()
//     }).collect();

//     grid
// }

pub fn part_two(input: &str) -> Option<u64> {
    let height = input.lines().count();
    let associated_operators: Vec<&str> = input.lines().nth(height - 1).unwrap().split_whitespace().collect();
    // associated_operators.reverse();
    let width = input.lines().nth(0).unwrap().split_whitespace().count();
    let nums: Vec<Vec<u64>> = Vec::new();
    let lines: Vec<String> = input.lines().collect();

    for i in 0..lines.len() {

    }

    let mut result: u64 = 0;

    for i in 0..width {
        let mut temp: u64 = nums[i][0];
        if associated_operators[i] == "*" {
            for x in 1..nums[i].len() {
                temp *= nums[i][x];
            }
        } else {
            for x in 1..nums[i].len() {
                temp += nums[i][x];
            }
        }
        result += temp;
        println!("{} {}", temp, result)
    }

    Some(result)
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
