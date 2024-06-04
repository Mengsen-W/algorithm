// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPairsOfConnectableServers(edges [][]int, signalSpeed int) []int {
	n := len(edges) + 1
	graph := make([][][]int, n)
	for _, e := range edges {
		u, v, w := e[0], e[1], e[2]
		graph[u] = append(graph[u], []int{v, w})
		graph[v] = append(graph[v], []int{u, w})
	}

	var dfs func(int, int, int) int
	dfs = func(p, root, curr int) int {
		res := 0
		if curr == 0 {
			res++
		}
		for _, e := range graph[p] {
			v, cost := e[0], e[1]
			if v != root {
				res += dfs(v, p, (curr+cost)%signalSpeed)
			}
		}
		return res
	}

	res := make([]int, n)
	for i := 0; i < n; i++ {
		pre := 0
		for _, e := range graph[i] {
			v, cost := e[0], e[1]
			cnt := dfs(v, i, cost%signalSpeed)
			res[i] += pre * cnt
			pre += cnt
		}
	}
	return res
}

func main() {
	tests := []struct {
		edges       [][]int
		signalSpeed int
		ans         []int
	}{
		{[][]int{{0, 1, 1}, {1, 2, 5}, {2, 3, 13}, {3, 4, 9}, {4, 5, 2}}, 1, []int{0, 4, 6, 6, 4, 0}},
		{[][]int{{0, 6, 3}, {6, 5, 3}, {0, 3, 1}, {3, 2, 7}, {3, 1, 6}, {3, 4, 2}}, 3, []int{2, 0, 0, 0, 0, 0, 2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPairsOfConnectableServers(test.edges, test.signalSpeed), index)
	}
}
