// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minPairSum(nums []int) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	sort.Ints(nums)
	n := len(nums)
	for i, val := range nums[:n/2] {
		ans = max(ans, val+nums[n-1-i])
	}
	return
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 5, 2, 3}, 7},
		{[]int{3, 5, 4, 2, 4, 6}, 8},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minPairSum(test.nums), "test %d", index)
	}
}
