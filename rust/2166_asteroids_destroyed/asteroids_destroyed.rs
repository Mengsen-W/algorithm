struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut asteroids = asteroids;
        asteroids.sort(); // 按照质量升序排序
        let mut current_mass = mass as i64; // 防止整数溢出
        for asteroid in asteroids {
            // 按顺序遍历小行星，尝试摧毁并更新质量或者返回结果
            if current_mass < asteroid as i64 {
                return false;
            }
            current_mass += asteroid as i64;
        }
        true // 成功摧毁所有小行星
    }
}

fn main() {
    let tests = vec![
        (10, vec![3, 9, 19, 5, 21], true),
        (5, vec![4, 9, 23, 4], false),
    ];

    for (mass, asteroids, expected) in tests {
        assert_eq!(Solution::asteroids_destroyed(mass, asteroids), expected);
    }
}
