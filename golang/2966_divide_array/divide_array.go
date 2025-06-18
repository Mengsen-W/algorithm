// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func divideArray(nums []int, k int) [][]int {
	sort.Ints(nums)
	res := [][]int{}
	for i := 0; i < len(nums); i += 3 {
		if nums[i+2]-nums[i] > k {
			return nil
		}
		res = append(res, nums[i:i+3])
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  [][]int
	}{
		{[]int{1, 3, 4, 8, 7, 9, 3, 5, 1}, 2, [][]int{{1, 1, 3}, {3, 4, 5}, {7, 8, 9}}},
		{[]int{2, 4, 2, 2, 5, 2}, 2, [][]int{}},
		{
			[]int{4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11},
			14,
			[][]int{{2, 2, 2}, {4, 5, 5}, {5, 5, 7}, {7, 8, 8}, {9, 9, 10}, {11, 12, 12}},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, divideArray(test.nums, test.k), index)
	}
}
