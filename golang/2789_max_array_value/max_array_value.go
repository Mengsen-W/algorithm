/*
 * @Date: 2024-03-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-14
 * @FilePath: /algorithm/golang/2789_max_array_value/max_array_value.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxArrayValue(nums []int) int64 {
	sum := int64(nums[len(nums)-1])
	for i := len(nums) - 2; i >= 0; i-- {
		if int64(nums[i]) <= sum {
			sum = int64(nums[i]) + sum
		} else {
			sum = int64(nums[i])
		}
	}
	return sum
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{2, 3, 7, 9, 3}, 21},
		{[]int{5, 3, 3}, 11},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxArrayValue(test.nums), index)
	}
}
