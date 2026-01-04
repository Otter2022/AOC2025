advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i64> {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut sum_of_invalids = 0;
    for range in ranges {
        let nums: Vec<&str> = range.split('-').collect();
        let start: i64 = nums[0].parse().unwrap();
        let end: i64 = nums[1].parse().unwrap();

        println!("running {start}-{end}", start=start, end=end);
        for i in start..=end {

            let cur = i.to_string();

            let seq_len = cur.len() / 2;

            if seq_len == 0 {
                continue
            }

            if cur.len() % seq_len != 0 {
                continue;
            }
            
            if &cur[seq_len..cur.len()] == &cur[0..seq_len]{
                println!("{i}", i=i);
                sum_of_invalids += i;
            } 

        }

    }
    println!("{sum_of_invalids}", sum_of_invalids=sum_of_invalids);
    Some(sum_of_invalids)
}

pub fn part_two(input: &str) -> Option<i64> {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut sum_of_invalids = 0;
    for range in ranges {
        let nums: Vec<&str> = range.split('-').collect();
        let start: i64 = nums[0].parse().unwrap();
        let end: i64 = nums[1].parse().unwrap();

        println!("running {start}-{end}", start=start, end=end);
        for i in start..=end {
            let mut flag = 0;
            let cur = i.to_string();
            // println!("{i}", i=i);
            for seq_len in (1..=cur.len() / 2).rev() {

                if cur.len() == 1 {
                    continue;
                }


                if cur.len() % seq_len != 0 {
                    continue;
                }
                
                let completed = 'loop_label: {
                    
                    for i in (seq_len..=cur.len()).step_by(seq_len) {
                        // println!("{p}, {o}", p=&cur[i-seq_len..i], o=&cur[0..seq_len]);
                        if &cur[i-seq_len..i] != &cur[0..seq_len]{
                            break 'loop_label false;
                        } 
                    }
                    true
                };

                if completed {
                    flag = 1;
                    break;
                }
            }
            
            if flag == 1 {
                println!("{i}", i=i);
                sum_of_invalids += i;
            }

        }

    }
    println!("{sum_of_invalids}", sum_of_invalids=sum_of_invalids);
    Some(sum_of_invalids)
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
