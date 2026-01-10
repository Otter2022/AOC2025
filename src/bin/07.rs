advent_of_code::solution!(7);
use std::collections::HashSet;
use std::collections::VecDeque;


pub fn part_one(input: &str) -> Option<u64> {
    let mut split_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut start: (usize, usize) = (0, 0);

    for (i, s) in input.lines().enumerate() {
        for (i2, c) in s.chars().enumerate() {
            if c == 'S' {
                start = (i, i2);
            } else if c == '^' {
                split_coords.insert((i, i2));
            }
        }
    }
    // println!("{:?}", split_coords);

    let end = input.lines().count();
    let mut cur_rays: Vec<(usize, usize)> = Vec::new();
    cur_rays.push(start);
    let mut num_splits: u64 = 0;

    while !cur_rays.is_empty() {
        // println!("{}", num_splits);
        if cur_rays[0].0 + 1 == end {
            // println!("beam ended")
        } else if split_coords.contains(&(cur_rays[0].0 + 1,cur_rays[0].1)){
            let new_ray_1 = (cur_rays[0].0 + 1, cur_rays[0].1 + 1);
            let new_ray_2 = (cur_rays[0].0 + 1, cur_rays[0].1 - 1);
            if !cur_rays.contains(&new_ray_1) {
                cur_rays.push(new_ray_1);
            }
            if !cur_rays.contains(&new_ray_2) {
                cur_rays.push(new_ray_2);
            }
            num_splits += 1;
        } else {
            let new_ray = (cur_rays[0].0 + 1, cur_rays[0].1);
            if !cur_rays.contains(&new_ray) {
                cur_rays.push(new_ray);
            }
        }
        cur_rays.remove(0);
    }

    Some(num_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    if lines.is_empty() {
        return Some(0);
    }

    let h = lines.len();
    let w = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut split = vec![vec![false; w]; h];
    let mut start: Option<(usize, usize)> = None;

    for (r, row) in lines.iter().enumerate() {
        for (c, &b) in row.iter().enumerate() {
            match b {
                b'S' => start = Some((r, c)),
                b'^' => split[r][c] = true,
                _ => {}
            }
        }
    }

    let (start_r, start_c) = start?;

    let mut counts = vec![0u64; w];
    if start_c >= w {
        return Some(0);
    }
    counts[start_c] = 1;

    for r in start_r..(h - 1) {
        let nr = r + 1;
        let mut next = vec![0u64; w];

        for c in 0..w {
            let k = counts[c];
            if k == 0 {
                continue;
            }

            if split[nr][c] {
                if c > 0 {
                    next[c - 1] = next[c - 1].wrapping_add(k);
                }
                if c + 1 < w {
                    next[c + 1] = next[c + 1].wrapping_add(k);
                }
            } else {
                next[c] = next[c].wrapping_add(k);
            }
        }

        counts = next;
    }

    let result: u64 = counts.into_iter().sum();
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
