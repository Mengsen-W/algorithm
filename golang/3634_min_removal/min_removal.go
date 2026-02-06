// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minRemoval(nums []int, k int) int {
	n := len(nums)
	sort.Ints(nums)

	ans := n
	right := 0

	for left := 0; left < n; left++ {
		for right < n && int64(nums[right]) <= int64(nums[left])*int64(k) {
			right++
		}
		current := n - (right - left)
		if current < ans {
			ans = current
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
		{[]int{2, 1, 5}, 2, 1},
		{[]int{1, 6, 2, 9}, 3, 2},
		{[]int{4, 6}, 2, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minRemoval(test.nums, test.k), index)
	}
}
