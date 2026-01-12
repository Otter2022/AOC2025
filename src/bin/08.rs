advent_of_code::solution!(8);
use std::collections::BinaryHeap;

pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn squared_distance(pnt1: &Point, pnt2: &Point) -> i32 {
    return (pnt1.x - pnt2.x).pow(2) + (pnt1.y - pnt2.y).pow(2) + (pnt1.z - pnt2.z).pow(2);
}

pub fn read_points(input: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    for i in input.lines() {
        let nums: Vec<i32> = i.split(',').map(|x| {x.parse::<i32>().unwrap()}).collect();

        if nums.len() != 3 {
            panic!("line length less than 3");
        }

        let new_point = Point {x: nums[0], y: nums[1], z: nums[2] };

        points.push(new_point)
    }

    points
}

pub struct DSU {
    parent: Vec<i32>,
    sz: Vec<i32>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut sz = vec![1; n];

        for i in 0..n {
            parent.push()
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    let mut points = read_points(input);
    let k = 1000;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let dist2 = squared_distance(&points[i], &points[j]);

            if heap.len() < k {
                heap.push((dist2, i as i32, j as i32))
            } else if dist2 < heap.peek().unwrap().0 {
                heap.pop();
                heap.push( (dist2, i as i32, j as i32));
            }
        }
    }

    let edges = heap.into_sorted_vec();



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
