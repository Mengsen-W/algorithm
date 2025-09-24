// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxTargetNodes(edges1 [][]int, edges2 [][]int) []int {
	var dfs func(node, parent, depth int, children [][]int, color []int) int
	dfs = func(node, parent, depth int, children [][]int, color []int) int {
		res := 1 - depth%2
		color[node] = depth % 2
		for _, child := range children[node] {
			if child == parent {
				continue
			}
			res += dfs(child, node, depth+1, children, color)
		}
		return res
	}

	build := func(edges [][]int, color []int) []int {
		n := len(edges) + 1
		children := make([][]int, n)
		for _, edge := range edges {
			u, v := edge[0], edge[1]
			children[u] = append(children[u], v)
			children[v] = append(children[v], u)
		}
		res := dfs(0, -1, 0, children, color)
		return []int{res, n - res}
	}

	n := len(edges1) + 1
	m := len(edges2) + 1
	color1 := make([]int, n)
	color2 := make([]int, m)
	count1 := build(edges1, color1)
	count2 := build(edges2, color2)
	res := make([]int, n)
	for i := 0; i < n; i++ {
		res[i] = count1[color1[i]] + max(count2[0], count2[1])
	}
	return res
}

func main() {
	tests := []struct {
		edges1 [][]int
		edges2 [][]int
		ans    []int
	}{
		{[][]int{{0, 1}, {0, 2}, {2, 3}, {2, 4}}, [][]int{{0, 1}, {0, 2}, {0, 3}, {2, 7}, {1, 4}, {4, 5}, {4, 6}}, []int{8, 7, 7, 8, 8}},
		{[][]int{{0, 1}, {0, 2}, {0, 3}, {0, 4}}, [][]int{{0, 1}, {1, 2}, {2, 3}}, []int{3, 6, 6, 6, 6}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxTargetNodes(test.edges1, test.edges2), index)
	}
}
