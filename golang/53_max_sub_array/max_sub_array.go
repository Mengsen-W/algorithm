/*
 * @Date: 2023-11-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-20
 * @FilePath: /algorithm/golang/53_max_sub_array/max_sub_array.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSubArray(nums []int) int {
	max := nums[0]
	for i := 1; i < len(nums); i++ {
		if nums[i]+nums[i-1] > nums[i] {
			nums[i] += nums[i-1]
		}
		if nums[i] > max {
			max = nums[i]
		}
	}
	return max
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{-2, 1, -3, 4, -1, 2, 1, -5, 4}, 6},
		{[]int{1}, 1},
		{[]int{5, 4, -1, 7, 8}, 23},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSubArray(test.nums), index)
	}
}
