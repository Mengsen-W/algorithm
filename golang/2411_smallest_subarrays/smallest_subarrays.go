// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestSubarrays(nums []int) []int {
	n := len(nums)
	pos := make([]int, 31)
	for i := range pos {
		pos[i] = -1
	}
	ans := make([]int, n)
	for i := n - 1; i >= 0; i-- {
		j := i
		for bit := 0; bit < 31; bit++ {
			if (nums[i] & (1 << bit)) == 0 {
				if pos[bit] != -1 {
					j = max(j, pos[bit])
				}
			} else {
				pos[bit] = i
			}
		}
		ans[i] = j - i + 1
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{1, 0, 2, 1, 3}, []int{3, 3, 2, 2, 1}},
		{[]int{1, 2}, []int{2, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestSubarrays(test.nums), "testcase %d", index)
	}
}
