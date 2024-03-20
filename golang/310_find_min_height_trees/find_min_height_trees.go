/*
 * @Date: 2022-04-06 11:08:38
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-20
 * @FilePath: /algorithm/golang/310_find_min_height_trees/find_min_height_trees.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findMinHeightTrees(n int, edges [][]int) []int {
	if n == 1 {
		return []int{0}
	}

	g := make([][]int, n)
	deg := make([]int, n)
	for _, e := range edges {
		x, y := e[0], e[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
		deg[x]++
		deg[y]++
	}

	q := []int{}
	for i, d := range deg {
		if d == 1 {
			q = append(q, i)
		}
	}

	remainNodes := n
	for remainNodes > 2 {
		remainNodes -= len(q)
		tmp := q
		q = nil
		for _, x := range tmp {
			for _, y := range g[x] {
				deg[y]--
				if deg[y] == 1 {
					q = append(q, y)
				}
			}
		}
	}
	return q
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		ans   []int
	}{
		{4, [][]int{{1, 0}, {1, 2}, {1, 3}}, []int{1}},
		{6, [][]int{{3, 0}, {3, 1}, {3, 2}, {3, 4}, {5, 4}}, []int{3, 4}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findMinHeightTrees(test.n, test.edges), index)
	}
}
