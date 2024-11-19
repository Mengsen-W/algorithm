// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func shortestDistanceAfterQueries(n int, queries [][]int) []int {
	prev := make([][]int, n)
	dp := make([]int, n)
	for i := 1; i < n; i++ {
		prev[i] = append(prev[i], i-1)
		dp[i] = i
	}
	var res []int
	for _, query := range queries {
		prev[query[1]] = append(prev[query[1]], query[0])
		for v := query[1]; v < n; v++ {
			for _, u := range prev[v] {
				dp[v] = min(dp[v], dp[u]+1)
			}
		}
		res = append(res, dp[n-1])
	}
	return res
}

func main() {
	tests := []struct {
		n       int
		queries [][]int
		ans     []int
	}{
		{5, [][]int{{2, 4}, {0, 2}, {0, 4}}, []int{3, 2, 1}},
		{4, [][]int{{0, 3}, {0, 2}}, []int{1, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, shortestDistanceAfterQueries(test.n, test.queries), index)
	}
}
