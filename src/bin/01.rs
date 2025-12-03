advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut dial: i64 = 50;
    let mut pswrd: i64 = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let mut clicks: i64 = chars.as_str().parse().unwrap();
        let full_rotations = clicks.div_euclid(100);
        pswrd = pswrd + full_rotations;
        clicks = clicks - (100 * full_rotations);
        
        if direction == 'L' {
            let temp_dial = dial - clicks;
            if temp_dial < 0 && dial != 0 {pswrd += 1}
            dial = (((dial - clicks) % 100) + 100) % 100;
            // println!("{temp_dial}", temp_dial=temp_dial)
        } else {
            let temp_dial = dial + clicks;
            if temp_dial > 100 && dial != 0 {pswrd += 1}
            dial = (((dial + clicks) % 100) + 100) % 100;
            // println!("{temp_dial}", temp_dial=temp_dial)
        }

        if dial == 0 {
            pswrd = pswrd + 1;
        }

        println!("{dial} | {pswrd}", dial=dial, pswrd=pswrd)
    }
    println!("password: {pswrd}", pswrd=pswrd);
    Some(pswrd)
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(44)
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
