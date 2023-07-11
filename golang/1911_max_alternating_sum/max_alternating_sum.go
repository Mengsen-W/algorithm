/*
 * @Date: 2023-07-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-11
 * @FilePath: /algorithm/golang/1911_max_alternating_sum/max_alternating_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxAlternatingSum(nums []int) int64 {
	even, odd := nums[0], 0
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for i := 1; i < len(nums); i++ {
		even = max(even, odd+nums[i])
		odd = max(odd, even-nums[i])
	}
	return int64(even)
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{4, 2, 5, 3, 7}, 7},
		{[]int{5, 6, 7, 8}, 8},
		{[]int{6, 2, 1, 2, 4, 5}, 10},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, maxAlternatingSum(item.nums), item.ans)
	}
}
