// Package main ...
package main

import (
	"math"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumAverage(nums []int) float64 {
	sort.Ints(nums)
	res, n := math.MaxFloat64, len(nums)
	for i := 0; i < n/2; i++ {
		res = min(res, float64(nums[i]+nums[n-1-i])/2)
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  float64
	}{
		{[]int{7, 8, 3, 4, 15, 13, 4, 1}, 5.5},
		{[]int{1, 9, 8, 3, 10, 5}, 5.5},
		{[]int{1, 2, 3, 7, 8, 9}, 5.0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumAverage(test.nums), index)
	}
}
