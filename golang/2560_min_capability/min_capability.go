/*
 * @Date: 2023-09-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-19
 * @FilePath: /algorithm/golang/2560_min_capability/min_capability.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minCapability(nums []int, k int) int {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	lower, upper := nums[0], nums[0]
	for _, x := range nums {
		lower = min(lower, x)
		upper = max(upper, x)
	}

	for lower <= upper {
		middle := (lower + upper) / 2
		count, visited := 0, false
		for _, x := range nums {
			if x <= middle && !visited {
				count, visited = count+1, true
			} else {
				visited = false
			}
		}
		if count >= k {
			upper = middle - 1
		} else {
			lower = middle + 1
		}
	}
	return lower
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{2, 3, 5, 9}, 2, 5},
		{[]int{2, 7, 9, 3, 1}, 2, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minCapability(test.nums, test.k), index)
	}
}
