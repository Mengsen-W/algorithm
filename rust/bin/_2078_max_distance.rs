struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut res = 0;   // 两栋颜色不同房子的最远距离
        // 遍历两栋房子下标并维护最远距离
        for i in 0..n {
            for j in i + 1..n {
                if colors[i] != colors[j] {
                    res = res.max((j as i32) - (i as i32));
                }
            }
        }
        res
    }
}


fn main() {
    let tests = vec![
        (vec![1,1,1,6,1,1,1], 3),
        (vec![1,8,3,8,3], 4),
        (vec![0,1], 1),
    ];

    for (colors, ans) in tests {
        assert_eq!(Solution::max_distance(colors), ans);
    }
}
