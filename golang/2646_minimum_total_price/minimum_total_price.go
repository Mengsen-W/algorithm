/*
 * @Date: 2023-12-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-06
 * @FilePath: /algorithm/golang/2646_minimum_total_price/minimum_total_price.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func find(uf []int, i int) int {
	if uf[i] == i {
		return i
	}
	uf[i] = find(uf, uf[i])
	return uf[i]
}

func minimumTotalPrice(n int, edges [][]int, price []int, trips [][]int) int {
	next := make([][]int, n)
	for _, edge := range edges {
		next[edge[0]] = append(next[edge[0]], edge[1])
		next[edge[1]] = append(next[edge[1]], edge[0])
	}

	query := make([][]int, n)
	for _, trip := range trips {
		query[trip[0]] = append(query[trip[0]], trip[1])
		if trip[0] != trip[1] {
			query[trip[1]] = append(query[trip[1]], trip[0])
		}
	}

	uf, visited, diff, parent := make([]int, n), make([]bool, n), make([]int, n), make([]int, n)
	var tarjan func(int, int)
	tarjan = func(node, p int) {
		parent[node], uf[node] = p, node
		for _, child := range next[node] {
			if child == p {
				continue
			}
			tarjan(child, node)
			uf[child] = node
		}
		for _, node1 := range query[node] {
			if node != node1 && !visited[node1] {
				continue
			}
			lca := find(uf, node1)
			diff[node]++
			diff[node1]++
			diff[lca]--
			if parent[lca] >= 0 {
				diff[parent[lca]]--
			}
		}
		visited[node] = true
	}
	tarjan(0, -1)

	count := make([]int, n)
	var dfs func(int, int) int
	dfs = func(node, p int) int {
		count[node] = diff[node]
		for _, child := range next[node] {
			if child == p {
				continue
			}
			count[node] += dfs(child, node)
		}
		return count[node]
	}
	dfs(0, -1)

	var dp func(int, int) []int
	dp = func(node, p int) []int {
		res := []int{
			price[node] * count[node], price[node] * count[node] / 2,
		}
		for _, child := range next[node] {
			if child == p {
				continue
			}
			v := dp(child, node)
			// node 没有减半，因此可以取子树的两种情况的最小值
			// node 减半，只能取子树没有减半的情况
			res[0], res[1] = res[0]+min(v[0], v[1]), res[1]+v[0]
		}
		return res
	}
	res := dp(0, -1)
	return min(res[0], res[1])
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		price []int
		trips [][]int
		ans   int
	}{
		{4, [][]int{{0, 1}, {1, 2}, {1, 3}}, []int{2, 2, 10, 6}, [][]int{{0, 3}, {2, 1}, {2, 3}}, 23},
		{2, [][]int{{0, 1}}, []int{2, 2}, [][]int{{0, 0}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumTotalPrice(test.n, test.edges, test.price, test.trips), index)
	}
}
