/*
 * @Date: 2023-07-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-20
 * @FilePath: /algorithm/golang/918_max_subarray_sum_circular/max_subarray_sum_circular.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSubarraySumCircular(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	type pair struct{ idx, val int }
	n := len(nums)
	pre, res := nums[0], nums[0]
	q := []pair{{0, pre}}
	for i := 1; i < 2*n; i++ {
		for len(q) > 0 && q[0].idx < i-n {
			q = q[1:]
		}
		pre += nums[i%n]
		res = max(res, pre-q[0].val)
		for len(q) > 0 && q[len(q)-1].val >= pre {
			q = q[:len(q)-1]
		}
		q = append(q, pair{i, pre})
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, -2, 3, -2}, 3},
		{[]int{5, -3, 5}, 10},
		{[]int{3, -2, 2, -3}, 3},
	}
	for _, iter := range tests {
		assert.Equal(&testing.T{}, maxSubarraySumCircular(iter.nums), iter.ans)
	}
}
