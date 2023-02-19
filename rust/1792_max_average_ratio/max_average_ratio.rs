/*
 * @Date: 2023-02-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-19
 * @FilePath: /algorithm/rust/1792_max_average_ratio/max_average_ratio.rs
 */

struct Solution;

#[derive(Eq, PartialEq)]
struct Class {
    passing: i32,
    total: i32,
}

impl Class {
    pub fn delta(&self) -> f64 {
        let p = self.passing as f64;
        let t = self.total as f64;
        (p + 1.0) / (t + 1.0) - p / t
    }
}

impl std::cmp::Ord for Class {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl std::cmp::PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.delta().partial_cmp(&other.delta())
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        let mut heap = std::collections::BinaryHeap::new();
        for class in &classes {
            heap.push(Class {
                passing: class[0],
                total: class[1],
            })
        }
        while extra_students > 0 {
            let ord = heap.pop().unwrap();
            heap.push(Class {
                passing: ord.passing + 1,
                total: ord.total + 1,
            });
            extra_students -= 1;
        }

        let mut result = 0.0;
        while !heap.is_empty() {
            let class = heap.pop().unwrap();
            result += class.passing as f64 / class.total as f64;
        }
        result /= classes.len() as f64;
        result
    }
}

fn main() {
    {
        let classes = [[1, 2], [3, 5], [2, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let extra_students = 2;
        let ans = 0.7833333333333333;
        assert_eq!(Solution::max_average_ratio(classes, extra_students), ans);
    }

    {
        let classes = [[2, 4], [3, 9], [4, 5], [2, 10]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let extra_students = 4;
        let ans = 0.5348484848484849;
        assert_eq!(Solution::max_average_ratio(classes, extra_students), ans);
    }
}
