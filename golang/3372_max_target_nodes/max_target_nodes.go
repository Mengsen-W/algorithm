// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxTargetNodes(edges1 [][]int, edges2 [][]int, k int) []int {
	var dfs func(node, parent int, children [][]int, k int) int
	dfs = func(node, parent int, children [][]int, k int) int {
		if k < 0 {
			return 0
		}
		res := 1
		for _, child := range children[node] {
			if child == parent {
				continue
			}
			res += dfs(child, node, children, k-1)
		}
		return res
	}

	build := func(edges [][]int, k int) []int {
		n := len(edges) + 1
		children := make([][]int, n)
		for _, edge := range edges {
			u, v := edge[0], edge[1]
			children[u] = append(children[u], v)
			children[v] = append(children[v], u)
		}
		res := make([]int, n)
		for i := 0; i < n; i++ {
			res[i] = dfs(i, -1, children, k)
		}
		return res
	}

	n := len(edges1) + 1
	count1 := build(edges1, k)
	count2 := build(edges2, k-1)
	maxCount2 := 0
	for _, c := range count2 {
		if c > maxCount2 {
			maxCount2 = c
		}
	}
	res := make([]int, n)
	for i := 0; i < n; i++ {
		res[i] = count1[i] + maxCount2
	}
	return res
}

func main() {
	tests := []struct {
		edges1 [][]int
		edges2 [][]int
		k      int
		res    []int
	}{
		{[][]int{{0, 1}, {0, 2}, {2, 3}, {2, 4}}, [][]int{{0, 1}, {0, 2}, {0, 3}, {2, 7}, {1, 4}, {4, 5}, {4, 6}}, 2, []int{9, 7, 9, 8, 8}},
		{[][]int{{0, 1}, {0, 2}, {0, 3}, {0, 4}}, [][]int{{0, 1}, {1, 2}, {2, 3}}, 1, []int{6, 3, 3, 3, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.res, maxTargetNodes(test.edges1, test.edges2, test.k), index)
	}
}
