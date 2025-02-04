// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sortArrayByParityII(nums []int) []int {
	for i, j := 0, 1; i < len(nums); i += 2 {
		if nums[i]%2 == 1 {
			for nums[j]%2 == 1 {
				j += 2
			}
			nums[i], nums[j] = nums[j], nums[i]
		}
	}
	return nums
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{4, 2, 5, 7}, []int{4, 5, 2, 7}},
		{[]int{2, 3}, []int{2, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sortArrayByParityII(test.nums), index)
	}
}
