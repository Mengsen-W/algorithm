// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func mostPoints(questions [][]int) int64 {
	n := len(questions)
	dp := make([]int64, n+1) // 解决每道题及以后题目的最高分数
	for i := n - 1; i >= 0; i-- {
		dp[i] = max(dp[i+1], int64(questions[i][0])+dp[min(n, i+questions[i][1]+1)])
	}
	return dp[0]
}

func main() {
	tests := []struct {
		questions [][]int
		ans       int64
	}{
		{[][]int{{3, 2}, {4, 3}, {4, 4}, {2, 5}}, 5},
		{[][]int{{1, 1}, {2, 2}, {3, 3}, {4, 4}, {5, 5}}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, mostPoints(test.questions), index)
	}
}
