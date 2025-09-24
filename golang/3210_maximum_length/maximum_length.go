// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumLength(nums []int) int {
	res := 0
	patterns := [][]int{{0, 0}, {0, 1}, {1, 0}, {1, 1}}
	for _, pattern := range patterns {
		cnt := 0
		for _, num := range nums {
			if num%2 == pattern[cnt%2] {
				cnt++
			}
		}
		res = max(res, cnt)
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 4}, 4},
		{[]int{1, 2, 1, 1, 2, 1, 2}, 6},
		{[]int{1, 3}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumLength(test.nums), index)
	}
}
