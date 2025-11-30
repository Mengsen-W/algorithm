// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func prefixesDivBy5(nums []int) []bool {
	ans := make([]bool, len(nums))
	x := 0
	for i, v := range nums {
		x = (x<<1 | v) % 5
		ans[i] = x == 0
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  []bool
	}{
		{[]int{0, 1, 1}, []bool{true, false, false}},
		{[]int{1, 1, 1}, []bool{false, false, false}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, prefixesDivBy5(test.nums), index)
	}
}
