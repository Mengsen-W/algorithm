// Package main ...
package main

import "strconv"

func maxSum(nums []int, k int) int {
	const MOD = 1000000007
	cnt := make([]int, 30)

	// 统计每个位上 1 的数量
	for _, num := range nums {
		for i := 0; i < 30; i++ {
			if num&(1<<i) != 0 {
				cnt[i]++
			}
		}
	}

	res := int64(0)
	for i := 0; i < k; i++ {
		cur := int64(0)
		// 从高位到低位构造当前数字
		for j := 29; j >= 0; j-- {
			if cnt[j] > 0 {
				cur |= int64(1) << j
				cnt[j]--
			}
		}
		res = (res + cur*cur) % MOD
	}

	return int(res)
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{2, 6, 5, 8}, 2, 261},
		{[]int{4, 5, 4, 7}, 3, 90},
	}

	for index, test := range tests {
		if got := maxSum(test.nums, test.k); got != test.ans {
			panic("index:" + strconv.Itoa(index) + " got:" + strconv.Itoa(got) + " want:" + strconv.Itoa(test.ans))
		}
	}
}
