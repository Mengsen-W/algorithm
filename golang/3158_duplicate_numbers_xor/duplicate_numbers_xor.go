// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func duplicateNumbersXOR(nums []int) int {
	cnt := make(map[int]bool)
	res := 0
	for _, num := range nums {
		if _, found := cnt[num]; found {
			res ^= num
		} else {
			cnt[num] = true
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 1, 3}, 1},
		{[]int{1, 2, 3}, 0},
		{[]int{1, 2, 2, 1}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, duplicateNumbersXOR(test.nums), index)
	}
}
