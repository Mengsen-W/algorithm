// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfPermutations(n int, requirements [][]int) int {
	const MOD int = 1e9 + 7
	reqMap := map[int]int{0: 0}
	maxCnt := 0
	for _, req := range requirements {
		reqMap[req[0]] = req[1]
		maxCnt = max(maxCnt, req[1])
	}
	if reqMap[0] != 0 {
		return 0
	}
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, maxCnt+1)
		for j := range dp[i] {
			dp[i][j] = -1
		}
	}

	var dfs func(end, cnt int) int
	dfs = func(end, cnt int) int {
		if end == 0 {
			return 1
		}
		if dp[end][cnt] != -1 {
			return dp[end][cnt]
		}
		if r, exists := reqMap[end-1]; exists {
			if r <= cnt && cnt <= end+r {
				dp[end][cnt] = dfs(end-1, r)
				return dp[end][cnt]
			}
			return 0
		}

		totSum := 0
		for i := 0; i <= min(end, cnt); i++ {
			totSum = (totSum + dfs(end-1, cnt-i)) % MOD
		}
		dp[end][cnt] = totSum
		return dp[end][cnt]
	}

	return dfs(n-1, reqMap[n-1])
}

func main() {
	tests := []struct {
		n            int
		requirements [][]int
		ans          int
	}{
		{3, [][]int{{2, 2}, {0, 0}}, 2},
		{3, [][]int{{2, 2}, {1, 1}, {0, 0}}, 1},
		{2, [][]int{{0, 0}, {1, 0}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfPermutations(test.n, test.requirements), index)
	}
}