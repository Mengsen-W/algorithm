// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSum(nums []int) int {
	positiveNumsSet := make(map[int]bool)
	maxNum := nums[0]
	for _, num := range nums {
		if num > 0 {
			positiveNumsSet[num] = true
		}
		maxNum = max(maxNum, num)
	}

	if len(positiveNumsSet) == 0 {
		return maxNum
	}
	sum := 0
	for num := range positiveNumsSet {
		sum += num
	}
	return sum
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 4, 5}, 15},
		{[]int{1, 1, 0, 1, 1}, 1},
		{[]int{1, 2, -1, -2, 1, 0, -1}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSum(test.nums), index)
	}
}
