/*
 * @Date: 2023-09-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-21
 * @FilePath: /algorithm/golang/2603_collect_the_coins/collect_the_coins.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func collectTheCoins(coins []int, edges [][]int) int {
	n := len(coins)
	g := make([][]int, n)
	degree := make([]int, n)
	for _, edge := range edges {
		x, y := edge[0], edge[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
		degree[x]++
		degree[y]++
	}

	rest := n
	// 删除树中所有无金币的叶子节点，直到树中所有的叶子节点都是含有金币的
	q := []int{}
	for i := 0; i < n; i++ {
		if degree[i] == 1 && coins[i] == 0 {
			q = append(q, i)
		}
	}
	for len(q) > 0 {
		u := q[0]
		q = q[1:]
		degree[u]--
		rest--
		for _, v := range g[u] {
			degree[v]--
			if degree[v] == 1 && coins[v] == 0 {
				q = append(q, v)
			}
		}
	}

	// 删除树中所有的叶子节点, 连续删除2次
	for j := 0; j < 2; j++ {
		q := []int{}
		for i := 0; i < n; i++ {
			if degree[i] == 1 {
				q = append(q, i)
			}
		}
		for len(q) > 0 {
			u := q[0]
			q = q[1:]
			degree[u]--
			rest--
			for _, v := range g[u] {
				degree[v]--
			}
		}
	}

	if rest == 0 {
		return 0
	}
	return (rest - 1) * 2
}

func main() {
	tests := []struct {
		coins []int
		edges [][]int
		ans   int
	}{
		{[]int{1, 0, 0, 0, 0, 1}, [][]int{{0, 1}, {1, 2}, {2, 3}, {3, 4}, {4, 5}}, 2},
		{[]int{0, 0, 0, 1, 1, 0, 0, 1}, [][]int{{0, 1}, {0, 2}, {1, 3}, {1, 4}, {2, 5}, {5, 6}, {5, 7}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, collectTheCoins(test.coins, test.edges), index)
	}
}
