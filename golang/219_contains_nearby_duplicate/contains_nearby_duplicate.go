// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func containsNearbyDuplicate(nums []int, k int) bool {
	set := map[int]struct{}{}
	for i, num := range nums {
		if i > k {
			delete(set, nums[i-k-1])
		}
		if _, ok := set[num]; ok {
			return true
		}
		set[num] = struct{}{}
	}
	return false
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  bool
	}{
		{[]int{1, 2, 3, 1}, 3, true},
		{[]int{1, 0, 1, 1}, 1, true},
		{[]int{1, 2, 3, 1, 2, 3}, 2, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, containsNearbyDuplicate(test.nums, test.k), index)
	}
}
