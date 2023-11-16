/*
 * @Date: 2023-11-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-16
 * @FilePath: /algorithm/golang/2760_longest_alternating_subarray/longest_alternating_subarray.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestAlternatingSubarray(nums []int, threshold int) int {
	res, dp := 0, 0
	for l := len(nums) - 1; l >= 0; l-- {
		if nums[l] > threshold {
			dp = 0
		} else if l == len(nums)-1 || nums[l]%2 != nums[l+1]%2 {
			dp++
		} else {
			dp = 1
		}
		if nums[l]%2 == 0 && dp > res {
			res = dp
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums      []int
		threshold int
		ans       int
	}{
		{[]int{3, 2, 5, 4}, 5, 3},
		{[]int{1, 2}, 2, 1},
		{[]int{2, 3, 4, 5}, 4, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestAlternatingSubarray(test.nums, test.threshold), index)
	}
}
