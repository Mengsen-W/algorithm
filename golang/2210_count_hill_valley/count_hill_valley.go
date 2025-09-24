// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countHillValley(nums []int) int {
	res := 0 // 峰与谷的数量
	n := len(nums)
	for i := 1; i < n-1; i++ {
		if nums[i] == nums[i-1] {
			// 去重
			continue
		}
		left := 0 // 左边可能的不相等邻居对应状态
		for j := i - 1; j >= 0; j-- {
			if nums[j] > nums[i] {
				left = 1
				break
			} else if nums[j] < nums[i] {
				left = -1
				break
			}
		}
		right := 0 // 右边可能的不相等邻居对应状态
		for j := i + 1; j < n; j++ {
			if nums[j] > nums[i] {
				right = 1
				break
			} else if nums[j] < nums[i] {
				right = -1
				break
			}
		}
		if left == right && left != 0 {
			// 此时下标 i 为峰或谷的一部分
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 4, 1, 1, 6, 5}, 3},
		{[]int{6, 6, 5, 5, 4, 1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countHillValley(test.nums), index)
	}
}
