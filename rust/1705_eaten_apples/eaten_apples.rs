struct Solution;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        let mut i = 0;
        let mut max_day = days.len() as i32;
        let mut ret = 0;

        while i <= max_day {
            if (i as usize) < days.len() {
                heap.push((-i - days[i as usize], apples[i as usize]));
                max_day = max_day.max(i + days[i as usize]);
            }

            while let Some((day, apple)) = heap.pop() {
                if -day > i && apple > 0 {
                    heap.push((day, apple - 1));
                    ret += 1;
                    break;
                }
            }

            i += 1;
        }

        ret
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2], 7),
        (vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2], 5),
    ];

    for (apples, days, ans) in tests {
        assert_eq!(Solution::eaten_apples(apples, days), ans);
    }
}
