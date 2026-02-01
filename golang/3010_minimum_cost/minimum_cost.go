// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumCost(nums []int) int {
	first := int(^uint(0) >> 1)
	second := int(^uint(0) >> 1)

	for i := 1; i < len(nums); i++ {
		x := nums[i]
		if x < first {
			second = first
			first = x
		} else if x < second {
			second = x
		}
	}
	return nums[0] + first + second
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 12}, 6},
		{[]int{5, 4, 3}, 12},
		{[]int{10, 3, 1, 1}, 12},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumCost(test.nums), index)
	}
}
