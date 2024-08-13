// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isArraySpecial(nums []int) bool {
	n := len(nums)
	for i := 1; i < n; i++ {
		if nums[i-1]%2 == nums[i]%2 {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		nums []int
		ans  bool
	}{
		{[]int{1}, true},
		{[]int{2, 1, 4}, true},
		{[]int{4, 3, 1, 6}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isArraySpecial(test.nums), index)
	}
}
