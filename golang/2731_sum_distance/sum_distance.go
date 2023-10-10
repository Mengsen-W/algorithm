/*
 * @Date: 2023-10-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-10
 * @FilePath: /algorithm/golang/2731_sum_distance/sum_distance.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumDistance(nums []int, s string, d int) int {
	const mod int = 1e9 + 7
	n := len(nums)
	pos := make([]int, n)
	for i, ch := range s {
		if ch == 'L' {
			pos[i] = nums[i] - d
		} else {
			pos[i] = nums[i] + d
		}
	}
	sort.Ints(pos)
	res := 0
	for i := 1; i < n; i++ {
		res += (pos[i] - pos[i-1]) * i % mod * (n - i) % mod
		res %= mod
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		s    string
		d    int
		ans  int
	}{
		{[]int{-2, 0, 2}, "RLL", 3, 8},
		{[]int{1, 0}, "RL", 2, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, sumDistance(test.nums, test.s, test.d), test.ans, index)
	}
}
