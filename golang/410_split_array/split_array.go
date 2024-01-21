/*
 * @Date: 2024-01-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-21
 * @FilePath: /algorithm/golang/410_split_array/split_array.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func splitArray(nums []int, m int) int {
	left, right := 0, 0
	for i := 0; i < len(nums); i++ {
		right += nums[i]
		if left < nums[i] {
			left = nums[i]
		}
	}
	for left < right {
		mid := (right-left)/2 + left
		if check(nums, mid, m) {
			right = mid
		} else {
			left = mid + 1
		}
	}
	return left
}

func check(nums []int, x, m int) bool {
	sum, cnt := 0, 1
	for i := 0; i < len(nums); i++ {
		if sum+nums[i] > x {
			cnt++
			sum = nums[i]
		} else {
			sum += nums[i]
		}
	}
	return cnt <= m
}

func main() {
	tests := []struct {
		nums []int
		m    int
		ans  int
	}{
		{[]int{7, 2, 5, 10, 8}, 2, 18},
		{[]int{1, 2, 3, 4, 5}, 2, 9},
		{[]int{1, 4, 4}, 3, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, splitArray(test.nums, test.m), index)
	}
}
