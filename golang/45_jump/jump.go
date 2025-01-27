// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func jump(nums []int) int {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}

	length := len(nums)
	end := 0
	maxPosition := 0
	steps := 0
	for i := 0; i < length-1; i++ {
		maxPosition = max(maxPosition, i+nums[i])
		if i == end {
			end = maxPosition
			steps++
		}
	}
	return steps
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 3, 1, 1, 4}, 2},
		{[]int{2, 3, 0, 1, 4}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, jump(test.nums), index)
	}
}
