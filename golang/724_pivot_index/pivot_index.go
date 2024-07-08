// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func pivotIndex(nums []int) int {
	total := 0
	for _, v := range nums {
		total += v
	}
	sum := 0
	for i, v := range nums {
		if 2*sum+v == total {
			return i
		}
		sum += v
	}
	return -1
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 7, 3, 6, 5, 6}, 3},
		{[]int{1, 2, 3}, -1},
		{[]int{2, 1, -1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, pivotIndex(test.nums), index)
	}
}
