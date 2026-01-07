advent_of_code::solution!(5);

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::new();

    for (start, end) in ranges {
        if let Some((_, last_end)) = merged.last_mut() {
            if start <= *last_end {
                *last_end = (*last_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    merged
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();         
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    while let Some(cur) = lines.next() {
        if cur.is_empty() {
            break;
        }

        let mut parts = cur.split('-');
        let start: i64 = parts.next()?.parse().ok()?;
        let end: i64 = parts.next()?.parse().ok()?;

        ranges.push((start, end));
    }

    ranges = merge_ranges(ranges);

    let mut fresh_nums: u64 = 0;

    while let Some(cur) = lines.next() {
        let num: i64 = cur.parse().ok()?;
        for i in &ranges {
            if num <= i.1 && num >= i.0 {
                fresh_nums += 1;
                break;
            }
        }
    }

    Some(fresh_nums)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut lines = input.lines();         
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    while let Some(cur) = lines.next() {
        if cur.is_empty() {
            break;
        }

        let mut parts = cur.split('-');
        let start: i64 = parts.next()?.parse().ok()?;
        let end: i64 = parts.next()?.parse().ok()?;

        ranges.push((start, end));
    }

    ranges = merge_ranges(ranges);
    let mut total_ids: i64 = 0;

    for i in ranges {
        total_ids += i.1 - i.0 + 1;
    }

    Some(total_ids)
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
