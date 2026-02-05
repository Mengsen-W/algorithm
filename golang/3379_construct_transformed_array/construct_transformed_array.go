// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func constructTransformedArray(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	for i := 0; i < n; i++ {
		res[i] = nums[((i+nums[i])%n+n)%n]
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{3, -2, 1, 1}, []int{1, 1, 1, 3}},
		{[]int{-1, 4, -1}, []int{-1, -1, 4}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, constructTransformedArray(test.nums), index)
	}
}
