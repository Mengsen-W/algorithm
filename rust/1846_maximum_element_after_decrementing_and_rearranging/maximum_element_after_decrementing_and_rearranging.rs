struct Solution;

impl Solution {
    fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let n = arr.len() as i32;
        let mut cnt: Vec<i32> = vec![0; n as usize + 1];
        arr.iter().for_each(|&x| cnt[x.min(n) as usize] += 1);
        let mut miss = 0;
        for i in 1..=n as usize {
            match cnt[i] == 0 {
                true => miss += 1,
                false => miss -= miss.min(cnt[i] - 1),
            }
        }
        n - miss
    }
}

fn main() {
    let tests = vec![
        (vec![2, 2, 1, 2, 1], 2),
        (vec![100, 1, 1000], 3),
        (vec![1, 2, 3, 4, 5], 5),
    ];

    for (arr, expected) in tests {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(arr),
            expected
        );
    }
}
