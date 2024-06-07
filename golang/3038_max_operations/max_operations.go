// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxOperations(nums []int) int {
	n, t := len(nums), 0
	for i := 1; i < n; i += 2 {
		if nums[i]+nums[i-1] != nums[1]+nums[0] {
			break
		}
		t++
	}
	return t
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 2, 1, 4, 5}, 2},
		{[]int{3, 2, 6, 1, 4}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxOperations(test.nums), index)
	}
}
