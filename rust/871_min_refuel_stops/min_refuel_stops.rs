struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let mut queue = BinaryHeap::new();
        let (mut pos, mut station, mut ret) = (0, 0, -1);
        queue.push(start_fuel);
        while pos < target && !queue.is_empty() {
            pos += queue.pop().unwrap();
            ret += 1;
            while station < stations.len() && pos >= stations[station][0] {
                queue.push(stations[station][1]);
                station += 1;
            }
        }
        return if pos >= target { ret } else { -1 };
    }
}

fn main() {
    let tests = vec![
        (1, 1, vec![], 0),
        (100, 1, vec![vec![10, 100]], -1),
        (
            100,
            10,
            vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]],
            2,
        ),
    ];

    for (target, start_fuel, stations, ans) in tests {
        assert_eq!(
            Solution::min_refuel_stops(target, start_fuel, stations),
            ans
        );
    }
}
