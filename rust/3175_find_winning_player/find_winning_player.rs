struct Solution;

impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let n = skills.len();
        let mut cnt = 0;
        let mut i = 0;
        let mut last_i = 0;

        while i < n {
            let mut j = i + 1;
            while j < n && skills[j] < skills[i] && cnt < k {
                j += 1;
                cnt += 1;
            }
            if cnt == k {
                return i as i32;
            }
            cnt = 1;
            last_i = i as i32;
            i = j;
        }
        last_i
    }
}

fn main() {
    let tests = vec![(vec![4, 2, 6, 3, 9], 2, 2), (vec![2, 5, 4], 3, 1)];

    for (skills, k, ans) in tests {
        assert_eq!(Solution::find_winning_player(skills, k), ans);
    }
}
