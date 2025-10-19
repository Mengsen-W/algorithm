// Package main ...
package main

import (
	"math"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDistinctElements(nums []int, k int) int {
	sort.Ints(nums)
	cnt := 0
	prev := math.MinInt32

	for _, num := range nums {
		curr := min(max(num-k, prev+1), num+k)
		if curr > prev {
			cnt++
			prev = curr
		}
	}
	return cnt
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 2, 2, 3, 3, 4}, 2, 6},
		{[]int{4, 4, 4, 4}, 1, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxDistinctElements(test.nums, test.k), index)
	}
}
