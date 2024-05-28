struct Solution;

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 1..mountain.len() - 1 {
            if mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1] {
                res.push(i as i32);
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![2, 4, 4], vec![]), (vec![1, 4, 3, 8, 5], vec![1, 3])];

    for (mountain, ans) in tests {
        assert_eq!(Solution::find_peaks(mountain), ans);
    }
}
