/*
 * @Date: 2024-03-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-19
 * @FilePath: /algorithm/golang/1793_maximum_score/maximum_score.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumScore(nums []int, k int) int {
	n := len(nums)
	left, right := k-1, k+1
	ans := nums[k]
	for i := nums[k]; ; {
		for left >= 0 && nums[left] >= i {
			left--
		}
		for right < n && nums[right] >= i {
			right++
		}
		ans = max(ans, (right-left-1)*i)
		if left == -1 && right == n {
			break
		}
		lval, rval := -1, -1
		if left != -1 {
			lval = nums[left]
		}
		if right != n {
			rval = nums[right]
		}
		i = max(lval, rval)
		if i == -1 {
			break
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 4, 3, 7, 4, 5}, 3, 15},
		{[]int{5, 5, 4, 5, 4, 1, 1, 1}, 0, 20},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumScore(test.nums, test.k), index)
	}
}
