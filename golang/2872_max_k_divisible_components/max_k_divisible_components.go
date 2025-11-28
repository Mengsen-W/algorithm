// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxKDivisibleComponents(n int, edges [][]int, values []int, k int) int {
	graph := make([][]int, n)
	for _, edge := range edges {
		u, v := edge[0], edge[1]
		graph[u] = append(graph[u], v)
		graph[v] = append(graph[v], u)
	}
	result := 0
	var dfs func(int, int) int64
	dfs = func(node, parent int) int64 {
		sum := int64(values[node])
		for _, neighbor := range graph[node] {
			if neighbor != parent {
				sum += dfs(neighbor, node)
			}
		}
		if sum%int64(k) == 0 {
			result++
		}
		return sum
	}
	dfs(0, -1)
	return result
}

func main() {
	tests := []struct {
		n      int
		edges  [][]int
		values []int
		k      int
		ans    int
	}{
		{5, [][]int{{0, 2}, {1, 2}, {1, 3}, {2, 4}}, []int{1, 8, 1, 4, 4}, 6, 2},
		{7, [][]int{{0, 1}, {0, 2}, {1, 3}, {1, 4}, {2, 5}, {2, 6}}, []int{3, 0, 6, 1, 5, 2, 1}, 3, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxKDivisibleComponents(test.n, test.edges, test.values, test.k), index)
	}
}
