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

pub fn print_string_grid(grid: &Vec<Vec<String>>) {
    for row in grid {
        for cell in row {
            print!("\"{}\" ", cell);
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let height = input.lines().count();
    let associated_operators: Vec<&str> = input.lines().nth(height - 1).unwrap().split_whitespace().collect();
    let mut nums_as_str: Vec<Vec<String>> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    
    nums_as_str.push(Vec::new());
    nums_as_str[0].push(String::new());
    let mut cur_strs = 0;
    let mut cur_str = 0;
    for i1 in 0..lines[0].len() {
        
        let mut space_count = 0;
        
        for i2 in 0..lines.len() - 1 {
            let cur_char = lines[i2].chars().nth(i1).unwrap();
            println!("{}", cur_char);
            if cur_char != ' ' {
                nums_as_str[cur_strs][cur_str].push(cur_char);
            } else {
                space_count += 1
            }
        }

        if space_count == height - 1 {
            nums_as_str[cur_strs].remove(cur_str);
            cur_strs += 1;
            cur_str = 0;
            nums_as_str.push(Vec::new());
            nums_as_str[cur_strs].push(String::new());
        } else {
            cur_str += 1;
            nums_as_str[cur_strs].push(String::new());
        }
    }
    nums_as_str[cur_strs].remove(cur_str);

    print_string_grid(&nums_as_str);
    let mut nums: Vec<Vec<u64>> = Vec::new();
    nums = nums_as_str.iter().map(|row| {
        row.iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
    }).collect();

    let mut result: u64 = 0;

    for (i, num_arr) in nums.iter().enumerate() {
        let mut cur_num: u64 = 0;
        for (i2, num) in num_arr.iter().enumerate() {
            if i2 == 0 {
                cur_num = *num;
            } else {
                if associated_operators[i] == "*" {
                    cur_num *= *num;
                } else {
                    cur_num += *num;
                }
            }
        }
        result += cur_num;
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
