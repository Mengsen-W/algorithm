// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findKDistantIndices(nums []int, key int, k int) []int {
	var res []int
	r := 0 // 未被判断过的最小下标
	n := len(nums)
	for j := 0; j < n; j++ {
		if nums[j] == key {
			l := max(r, j-k)
			r = min(n-1, j+k) + 1
			for i := l; i < r; i++ {
				res = append(res, i)
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		key  int
		k    int
		ans  []int
	}{
		{[]int{3, 4, 9, 1, 3, 9, 5}, 9, 1, []int{1, 2, 3, 4, 5, 6}},
		{[]int{2, 2, 2, 2, 2}, 2, 2, []int{0, 1, 2, 3, 4}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findKDistantIndices(test.nums, test.key, test.k), index)
	}
}
