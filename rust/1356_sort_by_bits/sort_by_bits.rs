struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        let mut bit = vec![0; 10001];
        for i in 1..=10000 {
            bit[i] = bit[i >> 1] + (i & 1);
        }

        arr.sort_by(|&x, &y| {
            let count_x = bit[x as usize];
            let count_y = bit[y as usize];
            if count_x != count_y {
                count_x.cmp(&count_y)
            } else {
                x.cmp(&y)
            }
        });

        arr
    }
}

fn main() {
    let tests = vec![
        (
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7],
        ),
        (
            vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1],
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
        ),
        (vec![10000, 10000], vec![10000, 10000]),
        (
            vec![2, 3, 5, 7, 11, 13, 17, 19],
            vec![2, 3, 5, 17, 7, 11, 13, 19],
        ),
        (vec![10, 100, 1000, 10000], vec![10, 100, 10000, 1000]),
    ];

    for (arr, ans) in tests {
        assert_eq!(Solution::sort_by_bits(arr), ans);
    }
}
