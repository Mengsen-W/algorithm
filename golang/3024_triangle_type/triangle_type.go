// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func triangleType(nums []int) string {
	sort.Ints(nums)

	switch {
	case nums[0]+nums[1] <= nums[2]:
		return "none"
	case nums[0] == nums[2]:
		return "equilateral"
	case nums[0] == nums[1] || nums[1] == nums[2]:
		return "isosceles"
	default:
		return "scalene"
	}
}

func main() {
	tests := []struct {
		nums []int
		ans  string
	}{
		{[]int{3, 3, 3}, "equilateral"},
		{[]int{3, 4, 5}, "scalene"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, triangleType(test.nums), index)
	}
}
