// Package main ...
package main

import "fmt"

func countMajoritySubarrays(nums []int, target int) int64 {
	n := len(nums)
	// 表示前缀和为 -n, -(n-1), ..., 0, 1, ..., n 的出现次数，下标偏移 n
	pre := make([]int, n*2+1)
	pre[n] = 1
	cnt := n
	var ans, presum int64 = 0, 0
	for i := 0; i < n; i++ {
		if nums[i] == target {
			presum += int64(pre[cnt])
			cnt++
			pre[cnt]++
		} else {
			cnt--
			presum -= int64(pre[cnt])
			pre[cnt]++
		}
		ans += presum
	}
	return ans
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    int64
	}{
		{[]int{1, 2, 2, 3}, 2, 5},
		{[]int{1, 1, 1, 1}, 1, 10},
		{[]int{1, 2, 3}, 4, 0},
	}

	for index, test := range tests {
		if countMajoritySubarrays(test.nums, test.target) != test.ans {
			fmt.Printf("test %d failed\n", index)
		}
	}
}
