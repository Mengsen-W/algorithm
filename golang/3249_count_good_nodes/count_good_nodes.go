// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countGoodNodes(edges [][]int) int {
	n := len(edges) + 1
	g := make([][]int, n)
	for _, edge := range edges {
		x, y := edge[0], edge[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}
	res := 0
	var dfs func(node, parent int) int
	dfs = func(node, parent int) int {
		valid := true
		treeSize := 0
		subTreeSize := 0

		for _, child := range g[node] {
			if child != parent {
				size := dfs(child, node)
				if subTreeSize == 0 {
					subTreeSize = size
				} else if size != subTreeSize {
					valid = false
				}
				treeSize += size
			}
		}
		if valid {
			res++
		}
		return treeSize + 1
	}

	dfs(0, -1)
	return res
}

func main() {
	tests := []struct {
		edges [][]int
		ans   int
	}{
		{[][]int{{0, 1}, {0, 2}, {1, 3}, {1, 4}, {2, 5}, {2, 6}}, 7},
		{[][]int{{0, 1}, {1, 2}, {2, 3}, {3, 4}, {0, 5}, {1, 6}, {2, 7}, {3, 8}}, 6},
		{[][]int{{0, 1}, {1, 2}, {1, 3}, {1, 4}, {0, 5}, {5, 6}, {6, 7}, {7, 8}, {0, 9}, {9, 10}, {9, 12}, {10, 11}}, 12},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countGoodNodes(test.edges), index)
	}
}
