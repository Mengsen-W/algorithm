struct BookMyShow {
    n: i32,
    m: i32,
    min_tree: Vec<i32>,
    sum_tree: Vec<i64>,
}

impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        BookMyShow {
            n,
            m,
            min_tree: vec![0; 4 * n as usize],
            sum_tree: vec![0; 4 * n as usize],
        }
    }

    fn modify(&mut self, i: usize, l: i32, r: i32, index: i32, val: i32) {
        if l == r {
            self.min_tree[i] = val;
            self.sum_tree[i] = val as i64;
            return;
        }
        let mid = (l + r) / 2;
        if index <= mid {
            self.modify(i * 2, l, mid, index, val);
        } else {
            self.modify(i * 2 + 1, mid + 1, r, index, val);
        }
        self.min_tree[i] = self.min_tree[i * 2].min(self.min_tree[i * 2 + 1]);
        self.sum_tree[i] = self.sum_tree[i * 2] + self.sum_tree[i * 2 + 1];
    }

    fn query_min_row(&self, i: usize, l: i32, r: i32, val: i32) -> i32 {
        if l == r {
            if self.min_tree[i] > val {
                return self.n;
            }
            return l;
        }
        let mid = (l + r) / 2;
        if self.min_tree[i * 2] <= val {
            self.query_min_row(i * 2, l, mid, val)
        } else {
            self.query_min_row(i * 2 + 1, mid + 1, r, val)
        }
    }

    fn query_sum(&self, i: usize, l: i32, r: i32, l2: i32, r2: i32) -> i64 {
        if l2 <= l && r <= r2 {
            return self.sum_tree[i];
        }
        let mid = (l + r) / 2;
        let mut sum = 0;
        if mid >= l2 {
            sum += self.query_sum(i * 2, l, mid, l2, r2);
        }
        if mid + 1 <= r2 {
            sum += self.query_sum(i * 2 + 1, mid + 1, r, l2, r2);
        }
        sum
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let i = self.query_min_row(1, 0, self.n - 1, self.m - k);
        if i > max_row {
            return Vec::new();
        }
        let used = self.query_sum(1, 0, self.n - 1, i as i32, i as i32);
        self.modify(1, 0, self.n - 1, i as i32, used as i32 + k);
        vec![i as i32, used as i32]
    }

    fn scatter(&mut self, k2: i32, max_row: i32) -> bool {
        let mut k: i32 = k2;
        let used_total = self.query_sum(1, 0, self.n - 1, 0, max_row);
        if ((max_row as i64) + 1) * (self.m as i64) - used_total < (k as i64) {
            return false;
        }
        let mut i = self.query_min_row(1, 0, self.n - 1, self.m - 1) as i32;
        loop {
            let used = self.query_sum(1, 0, self.n - 1, i, i) as i32;
            if self.m - (used as i32) >= k {
                self.modify(1, 0, self.n - 1, i, used + k);
                break;
            }
            k -= self.m - used;
            self.modify(1, 0, self.n - 1, i, self.m);
            i += 1;
        }
        true
    }
}

fn main() {
    let mut bms = BookMyShow::new(2, 5);
    bms.gather(4, 0);
    bms.gather(2, 0);
    bms.scatter(5, 1);
    bms.scatter(5, 1);
}
