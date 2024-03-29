/*
 * @Date: 2021-09-15 08:45:54
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-18
 */

package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findPeakElement(nums []int) int {
	n := len(nums)

	// 辅助函数，输入下标 i，返回 nums[i] 的值
	// 方便处理 nums[-1] 以及 nums[n] 的边界情况
	get := func(i int) int {
		if i == -1 || i == n {
			return math.MinInt64
		}
		return nums[i]
	}

	left, right := 0, n-1
	for {
		mid := (left + right) / 2
		if get(mid-1) < get(mid) && get(mid) > get(mid+1) {
			return mid
		}
		if get(mid) < get(mid+1) {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 1}, 2},
		{[]int{1, 2, 1, 3, 5, 6, 4}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findPeakElement(test.nums), index)
	}
}
