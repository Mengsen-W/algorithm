// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxAdjacentDistance(nums []int) int {
	n := len(nums)
	res := int(math.Abs(float64(nums[0] - nums[n-1])))
	for i := 0; i < n-1; i++ {
		res = int(math.Max(float64(res), math.Abs(float64(nums[i]-nums[i+1]))))
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 4}, 3},
		{[]int{-5, -10, -5}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxAdjacentDistance(test.nums), index)
	}
}
