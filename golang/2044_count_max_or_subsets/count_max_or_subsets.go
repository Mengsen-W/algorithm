package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countMaxOrSubsets(nums []int) (ans int) {
	maxOr := 0
	var dfs func(int, int)
	dfs = func(pos, or int) {
		if pos == len(nums) {
			if or > maxOr {
				maxOr = or
				ans = 1
			} else if or == maxOr {
				ans++
			}
			return
		}
		dfs(pos+1, or|nums[pos])
		dfs(pos+1, or)
	}
	dfs(0, 0)
	return
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 1}, 2},
		{[]int{2, 2, 2}, 7},
		{[]int{3, 2, 1, 5}, 6},
	}
	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countMaxOrSubsets(test.nums), index)
	}
}
