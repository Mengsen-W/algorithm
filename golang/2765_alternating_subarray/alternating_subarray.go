/*
 * @Date: 2024-01-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-23
 * @FilePath: /algorithm/golang/2765_alternating_subarray/alternating_subarray.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func alternatingSubarray(nums []int) int {
	res := -1
	n := len(nums)
	firstIndex := 0
	for i := 1; i < n; i++ {
		length := i - firstIndex + 1
		if nums[i]-nums[firstIndex] == (length-1)%2 {
			res = max(res, length)
		} else {
			if nums[i]-nums[i-1] == 1 {
				firstIndex = i - 1
				res = max(res, 2)
			} else {
				firstIndex = i
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 3, 4, 3, 4}, 4},
		{[]int{4, 5, 6}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, alternatingSubarray(test.nums), index)
	}
}
