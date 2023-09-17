/*
 * @Date: 2023-09-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-17
 * @FilePath: /algorithm/golang/213_rob/rob.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func rob(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	n := len(nums)
	if n == 1 {
		return nums[0]
	}
	if n == 2 {
		return max(nums[0], nums[1])
	}
	_rob := func(nums []int) int {
		first, second := nums[0], max(nums[0], nums[1])
		for _, v := range nums[2:] {
			first, second = second, max(first+v, second)
		}
		return second
	}
	return max(_rob(nums[:n-1]), _rob(nums[1:]))
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 3, 2}, 3},
		{[]int{1, 2, 3, 1}, 4},
		{[]int{1, 2, 3}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, rob(test.nums), index)
	}
}
