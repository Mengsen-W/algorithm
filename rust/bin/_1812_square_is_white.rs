struct Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let v = coordinates.as_bytes();
        v[0] % 2 != v[1] % 2
    }
}

fn main() {
    let tests = vec![("a1", false), ("h3", true), ("c7", false)];

    for (coordinates, ans) in tests {
        assert_eq!(Solution::square_is_white(coordinates.to_string()), ans);
    }
}
