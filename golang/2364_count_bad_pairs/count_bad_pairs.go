// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countBadPairs(nums []int) int64 {
	mp := make(map[int]int)
	var res int64
	for i := 0; i < len(nums); i++ {
		key := nums[i] - i
		res += int64(i - mp[key])
		mp[key]++
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{4, 1, 3, 3}, 5},
		{[]int{1, 2, 3, 4, 5}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countBadPairs(test.nums), index)
	}
}
