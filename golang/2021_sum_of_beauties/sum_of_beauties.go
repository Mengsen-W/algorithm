// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumOfBeauties(nums []int) int {
	n := len(nums)
	state := make([]int, n)
	pre_max := nums[0]
	for i := 1; i < n-1; i++ {
		if nums[i] > pre_max {
			state[i] = 1
			pre_max = nums[i]
		}
	}
	suf_min := nums[n-1]
	res := 0
	for i := n - 2; i > 0; i-- {
		if state[i] == 1 && nums[i] < suf_min {
			res += 2
		} else if nums[i-1] < nums[i] && nums[i] < nums[i+1] {
			res += 1
		}
		suf_min = min(suf_min, nums[i])
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3}, 2},
		{[]int{2, 4, 6, 4}, 1},
		{[]int{3, 2, 1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sumOfBeauties(test.nums), index)
	}
}
