// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int, k int) int {
	sum := 0
	for _, num := range nums {
		sum += num
	}
	return sum % k
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{3, 9, 7}, 5, 4},
		{[]int{4, 1, 3}, 4, 0},
		{[]int{3, 2}, 6, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.nums, test.k), index)
	}
}
