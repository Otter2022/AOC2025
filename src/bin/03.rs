advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i64> {
    
    let mut res: i64 = 0;

    for num in input.lines() {
        let mut cur_largest_first: i64 = 0;
        let mut cur_largest_second: i64 = 0;
        let mut cur_ind = 0;

        while cur_ind < num.len() {
            let temp = num.chars().nth(cur_ind).unwrap_or('0');
            let cur_as_num: i64 = temp.to_digit(10).unwrap() as i64;
            
            if cur_as_num > cur_largest_first && cur_ind != num.len() - 1 {
                cur_largest_first = cur_as_num;
                cur_largest_second = 0;
                println!("new first num is {i1}{i2}",i1=cur_largest_first,i2=cur_largest_second);
            } else if cur_as_num > cur_largest_second {
                cur_largest_second = cur_as_num;
                println!("new sec num is {i1}{i2}",i1=cur_largest_first,i2=cur_largest_second);
            }
            cur_ind += 1;
        }
        println!("{i}", i = cur_largest_first * 10 + cur_largest_second);
        res += cur_largest_first * 10 + cur_largest_second;
    }

    println!("The answer is {t}!", t=res);
    return Some(res);
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut res: i64 = 0;

    for num in input.lines() {
        let mut v = vec![0i64; 12]; 
        let mut cur_ind = 0;

        while cur_ind < num.len() {
            let temp = num.chars().nth(cur_ind).unwrap_or('0');
            let cur_as_num: i64 = temp.to_digit(10).unwrap() as i64;
            
            for i in 0..v.len(){
                if cur_as_num > v[i] && cur_ind + (11 - i) < num.len() {
                    v[i] = cur_as_num;
                    for j in i+1..v.len(){
                        v[j] = 0;
                    }
                    break
                }
            }

            for el in &v {
                print!("{}", el);
            }
            println!("");

            cur_ind += 1;
        }

        for (index, value) in v.iter().enumerate() {
            let base: i64 = 10;
            res += base.pow(11-(index as u32)) * value;
            
            println!("{s} {sy}",s=value, sy=base.pow(11-(index as u32)));
        }
    }

    println!("The answer is {t}!", t=res);
    return Some(res);
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
