/*
 * @Date: 2023-12-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-28
 * @FilePath: /algorithm/golang/2735_min_cost/min_cost.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minCost(nums []int, x int) int64 {
	sum := func(arr []int) int64 {
		ans := int64(0)
		for _, a := range arr {
			ans += int64(a)
		}
		return ans
	}
	n := len(nums)
	f := make([]int, n)
	copy(f, nums)
	ans := sum(f)
	for k := 1; k < n; k++ {
		for i := 0; i < n; i++ {
			f[i] = min(f[i], nums[(i+k)%n])
		}
		ans = min(ans, int64(k)*int64(x)+sum(f))
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		x    int
		ans  int64
	}{
		{[]int{20, 1, 15}, 5, 13},
		{[]int{1, 2, 3}, 4, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minCost(test.nums, test.x), index)
	}
}
