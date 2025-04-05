// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func subsetXORSum(nums []int) int {
	res := 0
	n := len(nums)
	for _, num := range nums {
		res |= num
	}
	return res << (n - 1)
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 3}, 6},
		{[]int{5, 1, 6}, 28},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, subsetXORSum(test.nums), index)
	}
}
