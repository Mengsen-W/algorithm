// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int) int {
	operation := 0
	for i := len(nums) - 2; i >= 0; i-- {
		if nums[i] != nums[i+1] {
			operation++
		}
	}
	if nums[0] == 1 {
		return operation
	}
	return operation + 1
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{0, 1, 1, 0, 1}, 4},
		{[]int{1, 0, 0, 0}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.nums), index)
	}
}
