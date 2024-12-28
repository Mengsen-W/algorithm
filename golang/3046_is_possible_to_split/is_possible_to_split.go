// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isPossibleToSplit(nums []int) bool {
	count := make(map[int]int)
	for _, num := range nums {
		count[num]++
		if count[num] > 2 {
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
		{[]int{1, 1, 2, 2, 3, 4}, true},
		{[]int{1, 1, 1, 1}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isPossibleToSplit(test.nums), index)
	}
}
