// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int, k int) int {
	res := 0
	for _, num := range nums {
		if num < k {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{2, 11, 10, 1, 3}, 10, 3},
		{[]int{1, 1, 2, 4, 9}, 1, 0},
		{[]int{1, 1, 2, 4, 9}, 9, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.nums, test.k), index)
	}
}
