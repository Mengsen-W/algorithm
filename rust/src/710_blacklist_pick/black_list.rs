/*
 * @Date: 2022-06-26
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-26
 * @FilePath: /algorithm/710_ blacklist_pick/black_list.rs
 */
use rand;
use rand::Rng;

struct Solution {
    range: Vec<(i32, i32)>,
    prefix: Vec<i32>,
}

impl Solution {
    fn new(n: i32, mut blacklist: Vec<i32>) -> Self {
        blacklist.sort();
        let mut range = Vec::with_capacity(blacklist.len() + 1);
        let mut prefix = Vec::with_capacity(blacklist.len() + 1);
        let mut cur = 0;
        let mut last = 0;
        for num in blacklist.into_iter() {
            if cur < num {
                last += num - cur;
                range.push((cur, num));
                prefix.push(last);
            }
            cur = num + 1;
        }
        if cur < n {
            last += n - cur;
            range.push((cur, n));
            prefix.push(last);
        }
        Self {
            range: range,
            prefix: prefix,
        }
    }

    fn pick(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let (begin, end) = self.range[|rand: i32| -> usize {
            let mut lo = 0;
            let mut hi = self.prefix.len() - 1;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if rand < self.prefix[mid] {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            lo
        }(rng.gen_range(0, self.prefix.last().unwrap()))];
        rng.gen_range(begin, end)
    }
}

fn main() {}
