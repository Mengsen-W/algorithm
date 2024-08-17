// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumValueSum(nums []int, andValues []int) int {
	const INF = (1 << 20) - 1
	n := len(nums)
	m := len(andValues)
	memo := make([]map[int]int, n*m)
	for i := range memo {
		memo[i] = make(map[int]int)
	}

	var dfs func(i, j, cur int) int
	dfs = func(i, j, cur int) int {
		key := i*m + j
		if i == n && j == m {
			return 0
		}
		if i == n || j == m {
			return INF
		}
		if val, ok := memo[key][cur]; ok {
			return val
		}

		cur &= nums[i]
		if cur&andValues[j] < andValues[j] {
			return INF
		}

		res := dfs(i+1, j, cur)
		if cur == andValues[j] {
			res = min(res, dfs(i+1, j+1, INF)+nums[i])
		}
		memo[key][cur] = res
		return res
	}

	res := dfs(0, 0, INF)
	if res < INF {
		return res
	}
	return -1
}

func main() {
	tests := []struct {
		nums      []int
		andValues []int
		ans       int
	}{
		{[]int{1, 4, 3, 3, 2}, []int{0, 3, 3, 2}, 12},
		{[]int{2, 3, 5, 7, 7, 7, 5}, []int{0, 7, 5}, 17},
		{[]int{1, 2, 3, 4}, []int{2}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumValueSum(test.nums, test.andValues), index)
	}
}
