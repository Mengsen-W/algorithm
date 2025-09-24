// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestSubarray(nums []int) int {
	ans := 0
	p0, p1 := 0, 0
	for _, num := range nums {
		if num == 0 {
			p1 = p0
			p0 = 0
		} else {
			p0++
			p1++
		}
		if p1 > ans {
			ans = p1
		}
	}
	if ans == len(nums) {
		ans--
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 1, 0, 1}, 3},
		{[]int{0, 1, 1, 1, 0, 1, 1, 0, 1}, 5},
		{[]int{1, 1, 1}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestSubarray(test.nums), index)
	}
}
