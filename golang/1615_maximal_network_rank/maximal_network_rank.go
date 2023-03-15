/*
 * @Date: 2023-03-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-15
 * @FilePath: /algorithm/golang/1615_maximal_network_rank/maximal_network_rank.go
 */

// Package main ...
package main

func maximalNetworkRank(n int, roads [][]int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	connect := make([][]int, n)
	for i := range connect {
		connect[i] = make([]int, n)
	}
	degree := make([]int, n)
	for _, v := range roads {
		connect[v[0]][v[1]] = 1
		connect[v[1]][v[0]] = 1
		degree[v[0]]++
		degree[v[1]]++
	}

	maxRank := 0
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			rank := degree[i] + degree[j] - connect[i][j]
			maxRank = max(maxRank, rank)
		}
	}
	return maxRank
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		n := 4
		roads := [][]int{{0, 1}, {0, 3}, {1, 2}, {1, 3}}
		ans := 4
		assert(maximalNetworkRank(n, roads) == ans)
	}
	{
		n := 5
		roads := [][]int{{0, 1}, {0, 3}, {1, 2}, {1, 3}, {2, 3}, {2, 4}}
		ans := 5
		assert(maximalNetworkRank(n, roads) == ans)
	}
	{
		n := 8
		roads := [][]int{{0, 1}, {1, 2}, {2, 3}, {2, 4}, {5, 6}, {5, 7}}
		ans := 5
		assert(maximalNetworkRank(n, roads) == ans)
	}
}
