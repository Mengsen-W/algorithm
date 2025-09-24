struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let dxz = (x - z).abs();
        let dyz = (y - z).abs();
        if dxz < dyz {
            1
        } else if dxz > dyz {
            2
        } else {
            0
        }
    }
}

fn main() {
    let tests = vec![(2, 7, 4, 1), (2, 5, 6, 2)];

    for (x, y, z, expected) in tests {
        assert_eq!(Solution::find_closest(x, y, z), expected);
    }
}
