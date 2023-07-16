/*
 * @Date: 2023-07-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-16
 * @FilePath: /algorithm/golang/834_sum_of_distances_in_tree/sum_of_distances_in_tree.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumOfDistancesInTree(n int, edges [][]int) []int {
	graph := make([][]int, n)
	for _, e := range edges {
		u, v := e[0], e[1]
		graph[u] = append(graph[u], v)
		graph[v] = append(graph[v], u)
	}

	sz := make([]int, n)
	dp := make([]int, n)
	var dfs func(u, f int)
	dfs = func(u, f int) {
		sz[u] = 1
		for _, v := range graph[u] {
			if v == f {
				continue
			}
			dfs(v, u)
			dp[u] += dp[v] + sz[v]
			sz[u] += sz[v]
		}
	}
	dfs(0, -1)

	ans := make([]int, n)
	var dfs2 func(u, f int)
	dfs2 = func(u, f int) {
		ans[u] = dp[u]
		for _, v := range graph[u] {
			if v == f {
				continue
			}
			pu, pv := dp[u], dp[v]
			su, sv := sz[u], sz[v]

			dp[u] -= dp[v] + sz[v]
			sz[u] -= sz[v]
			dp[v] += dp[u] + sz[u]
			sz[v] += sz[u]

			dfs2(v, u)

			dp[u], dp[v] = pu, pv
			sz[u], sz[v] = su, sv
		}
	}
	dfs2(0, -1)
	return ans
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		ans   []int
	}{

		{6, [][]int{{0, 1}, {0, 2}, {2, 3}, {2, 4}, {2, 5}}, []int{8, 12, 6, 10, 10, 10}},
		{1, [][]int{}, []int{0}},
		{2, [][]int{{1, 0}}, []int{1, 1}},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, sumOfDistancesInTree(item.n, item.edges), item.ans)
	}
}
