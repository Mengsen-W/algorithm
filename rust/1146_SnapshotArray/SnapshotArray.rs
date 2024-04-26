/*
 * @Date: 2024-04-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-26
 * @FilePath: /algorithm/rust/1146_SnapshotArray/SnapshotArray.rs
 */

struct SnapshotArray {
    snap_cnt: i32,
    data: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            snap_cnt: 0,
            data: vec![vec![]; length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.data[index as usize].push((self.snap_cnt, val));
    }

    fn snap(&mut self) -> i32 {
        let ans = self.snap_cnt;
        self.snap_cnt += 1;
        ans
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let idx = self.binary_search(index, snap_id);
        if idx == 0 {
            return 0;
        }
        self.data[index as usize][idx - 1].1
    }

    fn binary_search(&self, index: i32, snap_id: i32) -> usize {
        let mut low = 0;
        let mut high = self.data[index as usize].len();
        while low < high {
            let mid = low + ((high - low) / 2);
            let (x, y) = self.data[index as usize][mid];
            if x > snap_id + 1 || (x == snap_id + 1 && y >= 0) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

fn main() {
    let mut snapshot_array = SnapshotArray::new(3);
    snapshot_array.set(0, 5);
    assert_eq!(snapshot_array.snap(), 0);
    snapshot_array.set(0, 6);
    assert_eq!(snapshot_array.get(0, 0), 5);
}
