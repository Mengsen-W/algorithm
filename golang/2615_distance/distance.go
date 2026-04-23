// package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func distance(nums []int) []int64 {
	n := len(nums)
	groups := make(map[int][]int)
	for i := 0; i < n; i++ {
		groups[nums[i]] = append(groups[nums[i]], i)
	}
	res := make([]int64, n)
	for _, group := range groups {
		var total int64
		for _, idx := range group {
			total += int64(idx)
		}
		var prefixTotal int64
		for i, idx := range group {
			res[idx] = total - prefixTotal*2 + int64(idx)*int64(2*i-len(group))
			prefixTotal += int64(idx)
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  []int64
	}{
		{[]int{1, 3, 1, 1, 2}, []int64{5, 0, 3, 4, 0}},
		{[]int{0, 5, 3}, []int64{0, 0, 0}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distance(test.nums), index)
	}
}
