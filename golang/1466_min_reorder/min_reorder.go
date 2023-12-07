/*
 * @Date: 2023-12-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-07
 * @FilePath: /algorithm/golang/1466_min_reorder/min_reorder.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func dfs(x, parent int, e [][][]int) int {
	res := 0
	for _, edge := range e[x] {
		if edge[0] == parent {
			continue
		}
		res += edge[1] + dfs(edge[0], x, e)
	}
	return res
}

func minReorder(n int, connections [][]int) int {
	e := make([][][]int, n)
	for _, edge := range connections {
		e[edge[0]] = append(e[edge[0]], []int{edge[1], 1})
		e[edge[1]] = append(e[edge[1]], []int{edge[0], 0})
	}
	return dfs(0, -1, e)
}

func main() {
	tests := []struct {
		n           int
		connections [][]int
		ans         int
	}{
		{6, [][]int{{0, 1}, {1, 3}, {2, 3}, {4, 0}, {4, 5}}, 3},
		{5, [][]int{{1, 0}, {1, 2}, {3, 2}, {3, 4}}, 2},
		{3, [][]int{{1, 0}, {2, 0}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minReorder(test.n, test.connections), index)
	}
}
