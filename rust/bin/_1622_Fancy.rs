const MOD: i64 = 1_000_000_007;

struct Fancy {
    v: Vec<i32>,
    a: i64,
    b: i64,
}

impl Fancy {
    fn new() -> Self {
        Fancy {
            v: Vec::new(),
            a: 1,
            b: 0,
        }
    }

    // 快速幂
    fn quick_mul(&self, x: i64, y: i64) -> i64 {
        let mut ret = 1;
        let mut cur = x;
        let mut power = y;
        while power != 0 {
            if power & 1 != 0 {
                ret = ret * cur % MOD;
            }
            cur = cur * cur % MOD;
            power >>= 1;
        }
        ret
    }

    // 乘法逆元
    fn inv(&self, x: i64) -> i64 {
        self.quick_mul(x, MOD - 2)
    }

    fn append(&mut self, val: i32) {
        let adjusted_val = ((val as i64 - self.b + MOD) % MOD) * self.inv(self.a) % MOD;
        self.v.push(adjusted_val as i32);
    }

    fn add_all(&mut self, inc: i32) {
        self.b = (self.b + inc as i64) % MOD;
    }

    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        self.a = self.a * m % MOD;
        self.b = self.b * m % MOD;
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.v.len() {
            return -1;
        }
        let ans = (self.a * self.v[idx] as i64 % MOD + self.b) % MOD;
        ans as i32
    }
}

fn main() {
    let mut fancy = Fancy::new();
    fancy.append(2); // 奇妙序列：[2]
    fancy.add_all(3); // 奇妙序列：[2+3] -> [5]
    fancy.append(7); // 奇妙序列：[5, 7]
    fancy.mult_all(2); // 奇妙序列：[5*2, 7*2] -> [10, 14]
    assert_eq!(fancy.get_index(0), 10); // 返回 10
    fancy.add_all(3); // 奇妙序列：[10+3, 14+3] -> [13, 17]
    fancy.append(10); // 奇妙序列：[13, 17, 10]
    fancy.mult_all(2); // 奇妙序列：[13*2, 17*2, 10*2] -> [26, 34, 20]
    assert_eq!(fancy.get_index(0), 26); // 返回 26
    assert_eq!(fancy.get_index(1), 34); // 返回 34
    assert_eq!(fancy.get_index(2), 20); // 返回 20
}
