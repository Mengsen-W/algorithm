// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumSize(nums []int, maxOperations int) int {
	max := 0
	for _, x := range nums {
		if x > max {
			max = x
		}
	}
	return sort.Search(max, func(y int) bool {
		if y == 0 {
			return false
		}
		ops := 0
		for _, x := range nums {
			ops += (x - 1) / y
		}
		return ops <= maxOperations
	})
}

func main() {
	tests := []struct {
		nums          []int
		maxOperations int
		ans           int
	}{
		{[]int{9}, 2, 3},
		{[]int{2, 4, 8, 2}, 4, 2},
		{[]int{7, 17}, 2, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumSize(test.nums, test.maxOperations), index)
	}
}
