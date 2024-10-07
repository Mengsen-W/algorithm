struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut i = 0;
        while i < n {
            let mut sum_of_gas = 0;
            let mut sum_of_cost = 0;
            let mut cnt = 0;
            while cnt < n {
                let j = (i + cnt) % n;
                sum_of_gas += gas[j];
                sum_of_cost += cost[j];
                if sum_of_cost > sum_of_gas {
                    break;
                }
                cnt += 1;
            }
            if cnt == n {
                return i as i32;
            } else {
                i = i + cnt + 1;
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2], 3),
        (vec![2, 3, 4], vec![3, 4, 3], -1),
    ];

    for (gas, cost, ans) in tests {
        assert_eq!(Solution::can_complete_circuit(gas, cost), ans);
    }
}
