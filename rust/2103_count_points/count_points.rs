/*
 * @Date: 2023-11-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-02
 * @FilePath: /algorithm/rust/2103_count_points/count_points.rs
 */

struct Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut d: [i32; 90] = [0; 90];
        d['R' as usize] = 1;
        d['G' as usize] = 2;
        d['B' as usize] = 4;

        let mut mask: [i32; 10] = [0; 10];

        let cs: Vec<char> = rings.chars().collect();

        for i in (0..cs.len()).step_by(2) {
            let c = cs[i] as usize;
            let j = cs[i + 1] as usize - '0' as usize;
            mask[j] |= d[c];
        }

        mask.iter().filter(|&&x| x == 7).count() as i32
    }
}

fn main() {
    let tests = vec![("B0B6G0R6R0R6G9", 1), ("B0R0G0R9R0B0G0", 1), ("G4", 0)];

    for (rings, ans) in tests {
        assert_eq!(Solution::count_points(rings.to_string()), ans);
    }
}
