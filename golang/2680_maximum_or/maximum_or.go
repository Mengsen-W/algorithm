// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumOr(nums []int, k int) int64 {
	var orSum, multiBits int64
	for _, x := range nums {
		multiBits |= int64(x) & orSum
		orSum |= int64(x)
	}
	var res int64
	for _, x := range nums {
		res = max(res, (orSum^int64(x))|(int64(x)<<k)|multiBits)
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int64
	}{
		{[]int{12, 9}, 1, 30},
		{[]int{8, 1, 2}, 2, 35},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumOr(test.nums, test.k), index)
	}
}
