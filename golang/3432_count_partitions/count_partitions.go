// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPartitions(nums []int) int {
	totalSum := 0
	for _, x := range nums {
		totalSum += x
	}
	if totalSum%2 == 0 {
		return len(nums) - 1
	}
	return 0
}

func main() {
	tests := []struct {
		nums []int
		want int
	}{
		{[]int{10, 10, 3, 7, 6}, 4},
		{[]int{1, 2, 2}, 0},
		{[]int{2, 4, 6, 8}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.want, countPartitions(test.nums), index)
	}
}
