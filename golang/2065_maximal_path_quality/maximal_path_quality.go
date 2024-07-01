// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximalPathQuality(values []int, edges [][]int, maxTime int) int {
	n := len(values)
	g := make([][][2]int, n)
	for _, edge := range edges {
		g[edge[0]] = append(g[edge[0]], [2]int{edge[1], edge[2]})
		g[edge[1]] = append(g[edge[1]], [2]int{edge[0], edge[2]})
	}

	visited := make([]bool, n)
	visited[0] = true
	ans := 0

	var dfs func(int, int, int)
	dfs = func(u, time, value int) {
		if u == 0 {
			ans = max(ans, value)
		}
		for _, e := range g[u] {
			v, dist := e[0], e[1]
			if time+dist <= maxTime {
				if !visited[v] {
					visited[v] = true
					dfs(v, time+dist, value+values[v])
					visited[v] = false
				} else {
					dfs(v, time+dist, value)
				}
			}
		}
	}

	dfs(0, 0, values[0])
	return ans
}

func main() {
	tests := []struct {
		values  []int
		edges   [][]int
		maxTime int
		ans     int
	}{
		{[]int{0, 32, 10, 43}, [][]int{{0, 1, 10}, {1, 2, 15}, {0, 3, 10}}, 49, 75},
		{[]int{5, 10, 15, 20}, [][]int{{0, 1, 10}, {1, 2, 10}, {0, 3, 10}}, 30, 25},
		{[]int{1, 2, 3, 4}, [][]int{{0, 1, 10}, {1, 2, 11}, {2, 3, 12}, {1, 3, 13}}, 50, 7},
		{[]int{0, 1, 2}, [][]int{{1, 2, 10}}, 10, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximalPathQuality(test.values, test.edges, test.maxTime), index)
	}
}
