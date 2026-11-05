struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut arr2 = arr2;
        arr2.sort_unstable();

        arr1.iter()
            .filter(|&&x| {
                let p = arr2.binary_search(&x).unwrap_or_else(|i| i);
                (p >= arr2.len() || arr2[p] - x > d) && (p == 0 || x - arr2[p - 1] > d)
            })
            .count() as i32
    }
}

fn main() {
    let tests = vec![
        (vec![4, 5, 8], vec![10, 9, 1, 8], 2, 2),
        (vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3, 2),
        (vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6, 1),
    ];

    for (i, (arr1, arr2, d, ans)) in tests.iter().enumerate() {
        assert_eq!(
            Solution::find_the_distance_value(arr1.clone(), arr2.clone(), *d),
            *ans,
            "{}",
            i
        );
    }
}
