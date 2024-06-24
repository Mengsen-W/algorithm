// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func nextGreaterElements(nums []int) []int {
	n := len(nums)
	ans := make([]int, n)
	for i := range ans {
		ans[i] = -1
	}
	stack := []int{}
	for i := 0; i < n*2-1; i++ {
		for len(stack) > 0 && nums[stack[len(stack)-1]] < nums[i%n] {
			ans[stack[len(stack)-1]] = nums[i%n]
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, i%n)
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{1, 2, 1}, []int{2, -1, 2}},
		{[]int{1, 2, 3, 4, 3}, []int{2, 3, 4, -1, 4}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, nextGreaterElements(test.nums), index)
	}
}
