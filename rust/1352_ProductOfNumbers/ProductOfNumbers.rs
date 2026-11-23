struct ProductOfNumbers {
    nums: Vec<i32>,
    zero_count: Vec<i32>,
    prev_non_one: Vec<i32>,
    n: usize,
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            nums: Vec::new(),
            zero_count: Vec::new(),
            prev_non_one: Vec::new(),
            n: 0,
        }
    }

    fn add(&mut self, num: i32) {
        self.n += 1;
        self.nums.push(num);

        self.prev_non_one.push(-1);
        if self.n > 1 {
            if self.nums[self.n - 2] != 1 && self.nums[self.n - 2] != 0 {
                self.prev_non_one[self.n - 1] = (self.n - 2) as i32;
            } else {
                self.prev_non_one[self.n - 1] = self.prev_non_one[self.n - 2];
            }
        }

        let zero_val = if num == 0 { 1 } else { 0 };
        self.zero_count.push(zero_val);
        if self.n > 1 {
            self.zero_count[self.n - 1] += self.zero_count[self.n - 2];
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;

        let mut total_zeros = self.zero_count[self.n - 1];
        if self.n >= k + 1 {
            total_zeros -= self.zero_count[self.n - 1 - k];
        }
        if total_zeros > 0 {
            return 0;
        }

        let mut ans = 1;
        let mut i = self.n - 1;
        let end = self.n - k;

        while i >= end {
            ans *= self.nums[i];
            let next_i = self.prev_non_one[i];
            if next_i == -1 {
                break;
            }
            i = next_i as usize;
        }

        ans
    }
}

fn main() {
    let mut product_of_numbers = ProductOfNumbers::new();
    product_of_numbers.add(3); // [3]
    product_of_numbers.add(0); // [3,0]
    product_of_numbers.add(2); // [3,0,2]
    product_of_numbers.add(5); // [3,0,2,5]
    product_of_numbers.add(4); // [3,0,2,5,4]
    product_of_numbers.get_product(2); // 返回 20 。最后 2 个数字的乘积是 5 * 4 = 20
    product_of_numbers.get_product(3); // 返回 40 。最后 3 个数字的乘积是 2 * 5 * 4 = 40
    product_of_numbers.get_product(4); // 返回  0 。最后 4 个数字的乘积是 0 * 2 * 5 * 4 = 0
    product_of_numbers.add(8); // [3,0,2,5,4,8]
    product_of_numbers.get_product(2); // 返回 32 。最后 2 个数字的乘积是 4 * 8 = 32
}
