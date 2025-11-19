// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findFinalValue(nums []int, original int) int {
	set := make(map[int]bool)
	for _, num := range nums {
		set[num] = true
	}
	for set[original] {
		original *= 2
	}
	return original
}

func main() {
	tests := []struct {
		nums     []int
		original int
		ans      int
	}{
		{[]int{5, 3, 6, 1, 12}, 3, 24},
		{[]int{2, 7, 9}, 4, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findFinalValue(test.nums, test.original), index)
	}
}
