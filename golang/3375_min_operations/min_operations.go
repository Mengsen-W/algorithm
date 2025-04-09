// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int, k int) int {
	st := make(map[int]bool)
	for _, x := range nums {
		if x < k {
			return -1
		} else if x > k {
			st[x] = true
		}
	}
	return len(st)
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{5, 2, 5, 4, 5}, 2, 2},
		{[]int{2, 1, 2}, 2, -1},
		{[]int{9, 7, 5, 3}, 1, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.nums, test.k), index)
	}
}
