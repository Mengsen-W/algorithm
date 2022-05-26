/*
 * @Date: 2022-05-26 10:19:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-26 10:30:12
 * @FilePath: /algorithm/699_falling_squares/falling_squares.rs
 */

pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::BTreeMap;
    use std::ops::Bound::Included;
    let n = positions.len();
    let mut st = BTreeMap::new();
    let (mut del_rng, mut add_rng) = (Vec::with_capacity(n), Vec::with_capacity(n));
    let mut ans = Vec::with_capacity(n);
    let mut h = 0i32;
    for position in positions {
        let (x, y, mut z) = (position[0], position[0] + position[1] - 1, position[1]);
        del_rng.clear();
        add_rng.clear();
        for (&(r, l), &h) in st.range((Included((x, i32::MIN)), Included((i32::MAX, i32::MAX)))) {
            if l > y {
                break;
            }
            del_rng.push((r, l));
            z = z.max(h + position[1]);
            if r - l + 1 > 1 {
                if l < x && x <= r {
                    add_rng.push((l, x - 1, h));
                }
                if y <= r {
                    add_rng.push((y + 1, r, h));
                }
            }
        }
        for rng in &del_rng {
            st.remove(rng);
        }
        for &(l, r, h) in &add_rng {
            st.insert((r, l), h);
        }
        st.insert((y, x), z);
        h = h.max(z);
        ans.push(h);
    }
    ans
}

fn main() {
    assert_eq!(
        falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]]),
        vec![2, 5, 5]
    );
    assert_eq!(
        falling_squares(vec![vec![100, 100], vec![200, 100]]),
        vec![100, 100]
    );
}
