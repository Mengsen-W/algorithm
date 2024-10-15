struct Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        fn _max_height(x: i32, y: i32) -> i32 {
            let odd = 2 * ((x as f64).sqrt() as i32) - 1;
            let even = 2 * (((-1.0 + (1.0 + 4.0 * (y as f64)).sqrt()) / 2.0) as i32);
            odd.min(even) + 1
        }
        std::cmp::max(_max_height(red, blue), _max_height(blue, red))
    }
}

fn main() {
    let tests = vec![(2, 4, 3), (2, 1, 2), (1, 1, 1), (10, 1, 2)];

    for (red, blue, ans) in tests {
        assert_eq!(Solution::max_height_of_triangle(red, blue), ans);
    }
}
