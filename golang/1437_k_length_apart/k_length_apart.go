// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func kLengthApart(nums []int, k int) bool {
	n := len(nums)
	prev := -1
	for i := 0; i < n; i++ {
		if nums[i] == 1 {
			if prev != -1 && i-prev-1 < k {
				return false
			}
			prev = i
		}
	}
	return true
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  bool
	}{
		{[]int{1, 0, 0, 0, 1, 0, 0, 1}, 2, true},
		{[]int{1, 0, 0, 1, 0, 1}, 2, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, kLengthApart(test.nums, test.k), index)
	}
}
