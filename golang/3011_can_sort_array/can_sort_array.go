// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func canSortArray(nums []int) bool {
	lastCnt := 0
	lastGroupMax := 0
	curGroupMax := 0

	for _, num := range nums {
		curCnt := bits.OnesCount(uint(num))
		if curCnt == lastCnt {
			curGroupMax = max(curGroupMax, num)
		} else {
			lastCnt = curCnt
			lastGroupMax = curGroupMax
			curGroupMax = num
		}
		if num < lastGroupMax {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		nums []int
		ans  bool
	}{
		{[]int{8, 4, 2, 30, 15}, true},
		{[]int{1, 2, 3, 4, 5}, true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canSortArray(test.nums), index)
	}
}
