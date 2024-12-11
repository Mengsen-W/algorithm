// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func semiOrderedPermutation(nums []int) int {
	n := len(nums)
	first, last := 0, 0
	for i := 0; i < n; i++ {
		if nums[i] == 1 {
			first = i
		}
		if nums[i] == n {
			last = i
		}
	}
	if last < first {
		return first + n - 1 - last - 1
	}
	return first + n - 1 - last
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 1, 4, 3}, 2},
		{[]int{2, 4, 1, 3}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, semiOrderedPermutation(test.nums), index)
	}
}
