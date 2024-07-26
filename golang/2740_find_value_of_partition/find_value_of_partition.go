// Package main ...
package main

import (
	"math"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findValueOfPartition(nums []int) int {
	sort.Ints(nums)
	res := math.MaxInt
	for i := 1; i < len(nums); i++ {
		res = min(res, nums[i]-nums[i-1])
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 3, 2, 4}, 1},
		{[]int{100, 1, 10}, 9},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findValueOfPartition(test.nums), index)
	}
}
