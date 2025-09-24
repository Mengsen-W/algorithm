// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumTripletValue(nums []int) int64 {
	n := len(nums)
	var res, imax, dmax int64 = 0, 0, 0
	for k := 0; k < n; k++ {
		res = max(res, dmax*int64(nums[k]))
		dmax = max(dmax, imax-int64(nums[k]))
		imax = max(imax, int64(nums[k]))
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{12, 6, 1, 2, 7}, 77},
		{[]int{1, 10, 3, 4, 19}, 133},
		{[]int{1, 2, 3}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumTripletValue(test.nums), index)
	}
}
