/*
 * @Date: 2023-11-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-12
 * @FilePath: /algorithm/rust/715_RangeModule/RangeModule.rs
 */

struct RangeModule {
    range_tab: Vec<Range>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Range(i32, i32);
/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl RangeModule {
    fn new() -> Self {
        Self { range_tab: vec![] }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        if self.range_tab.is_empty() {
            self.range_tab.push(Range(left, right));
        } else {
            //二分类似lower_bound快速定位
            let idx_start = self.range_tab.partition_point(|Range(_, r)| *r < left);
            let i = idx_start;

            let mut new_left = left;
            let mut new_right = right;
            while i < self.range_tab.len() && right >= self.range_tab[i].0 {
                let l = self.range_tab[i].0;
                let r = self.range_tab[i].1;
                //如[1,3),[4,5),[6,10)加入[2,8)
                //最后会合成[1,10)
                if l < new_left {
                    new_left = l;
                }
                if r > new_right {
                    new_right = r;
                }
                self.range_tab.remove(i);
            }

            self.range_tab.insert(i, Range(new_left, new_right));
        }
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        if self.range_tab.is_empty() {
            return false;
        }

        let idx_start = self.range_tab.partition_point(|Range(_, r)| *r < left);
        if idx_start == self.range_tab.len() || right <= self.range_tab[idx_start].0 {
            return false;
        }

        let mut i = idx_start;
        while i < self.range_tab.len() && right >= self.range_tab[i].0 {
            let l = self.range_tab[i].0;
            let r = self.range_tab[i].1;
            if l <= left && r >= right {
                return true;
            }
            i += 1;
        }

        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        if self.range_tab.is_empty() {
            return;
        }
        let idx_start = self.range_tab.partition_point(|Range(_, r)| *r < left);
        let mut i = idx_start;

        while i < self.range_tab.len() && right >= self.range_tab[i].0 {
            let l = self.range_tab[i].0;
            let r = self.range_tab[i].1;
            //如[1,6)中移除[2,3)
            if l < left && r > right {
                self.range_tab[i].1 = left;
                self.range_tab.insert(i + 1, Range(right, r));
                return;
            }
            //如[1,4), [6, 10)中移除[2,8)
            //移除后要变成[1,2),[8,10)
            if l < left {
                self.range_tab[i].1 = left;
            } else if r > right {
                self.range_tab[i].0 = right;
            } else {
                self.range_tab.remove(i);
                //移除后不需要i+1，i会指向下一个元素
                continue;
            }
            i += 1;
        }
    }
}

fn main() {
    let mut range_module = RangeModule::new();
    range_module.add_range(10, 20);
    range_module.remove_range(14, 16);
    assert_eq!(range_module.query_range(10, 14), true);
    assert_eq!(range_module.query_range(13, 15), false);
    assert_eq!(range_module.query_range(16, 17), true);
}
