// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func resultArray(nums []int, k int) []int64 {
	n := len(nums)
	result := make([]int64, k)
	dp := make([]int64, k) // 初始状态，表示尚未处理任何元素，因此不存在非空子数组

	for i := 0; i < n; i++ {
		ndp := make([]int64, k) // 当前层状态（滚动数组）
		ndp[nums[i]%k]++
		for r := 0; r < k; r++ {
			ndp[(int64(r)*int64(nums[i]))%int64(k)] += dp[r]
		}

		dp = ndp // 更新状态

		// 累加答案
		for r := 0; r < k; r++ {
			result[r] += dp[r]
		}
	}

	return result
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  []int64
	}{
		{[]int{1, 2, 3, 4, 5}, 3, []int64{9, 2, 4}},
		{[]int{1, 2, 4, 8, 16, 32}, 4, []int64{18, 1, 2, 0}},
		{[]int{1, 1, 2, 1, 1}, 2, []int64{9, 6}},
	}

	for index, test := range tests {
		if !reflect.DeepEqual(test.ans, resultArray(test.nums, test.k)) {
			fmt.Println("Test", index+1, "failed")
		}
	}
}
