package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countAlternatingSubarrays(nums []int) int64 {
	var res, cur int64
	pre := -1
	for _, a := range nums {
		if pre != a {
			cur++
		} else {
			cur = 1
		}
		pre = a
		res += cur
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{0, 1, 1, 1}, 5},
		{[]int{1, 0, 1, 0}, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countAlternatingSubarrays(test.nums), index)
	}
}
