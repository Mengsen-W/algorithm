// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumPoints(edges [][]int, coins []int, k int) int {
	n := len(coins)
	children := make([][]int, n)
	for _, edge := range edges {
		u, v := edge[0], edge[1]
		children[u] = append(children[u], v)
		children[v] = append(children[v], u)
	}

	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, 14)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(node, parent, f int) int
	dfs = func(node, parent, f int) int {
		if memo[node][f] >= 0 {
			return memo[node][f]
		}
		res0, res1 := (coins[node]>>f)-k, coins[node]>>(f+1)
		for _, child := range children[node] {
			if child == parent {
				continue
			}
			res0 += dfs(child, node, f)
			if f+1 < 14 {
				res1 += dfs(child, node, f+1)
			}
		}
		memo[node][f] = max(res0, res1)
		return memo[node][f]
	}

	return dfs(0, -1, 0)
}

func main() {
	tests := []struct {
		edges [][]int
		coins []int
		k     int
		ans   int
	}{
		{[][]int{{0, 1}, {1, 2}, {2, 3}}, []int{10, 10, 3, 3}, 5, 11},
		{[][]int{{0, 1}, {0, 2}}, []int{8, 4, 4}, 0, 16},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumPoints(test.edges, test.coins, test.k), index)
	}
}
