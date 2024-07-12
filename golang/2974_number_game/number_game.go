// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberGame(nums []int) []int {
	sort.Ints(nums)
	for i := 0; i < len(nums)-1; i += 2 {
		nums[i], nums[i+1] = nums[i+1], nums[i]
	}
	return nums
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{5, 4, 2, 3}, []int{3, 2, 5, 4}},
		{[]int{2, 5}, []int{5, 2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberGame(test.nums), index)
	}
}
