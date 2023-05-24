/*
 * @Date: 2023-05-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-24
 * @FilePath: /algorithm/golang/1377_frog_position/frog_position.go
 */

// Package main ...
package main

func frogPosition(n int, edges [][]int, t int, target int) float64 {
	G := make([][]int, n+1)
	for i := 0; i <= n; i++ {
		G[i] = make([]int, 0)
	}
	for _, e := range edges {
		G[e[0]] = append(G[e[0]], e[1])
		G[e[1]] = append(G[e[1]], e[0])
	}
	seen := make([]bool, n+1)
	var dfs func(G [][]int, seen []bool, i int, t int, target int) float64
	dfs = func(G [][]int, seen []bool, i int, t int, target int) float64 {
		nxt := len(G[i])
		if i > 1 {
			nxt -= 1
		}
		if t == 0 || nxt == 0 {
			if i == target {
				return 1.0
			} else {
				return 0.0
			}
		}
		seen[i] = true
		ans := 0.0
		for _, j := range G[i] {
			if !seen[j] {
				ans += dfs(G, seen, j, t-1, target)
			}
		}
		return ans / float64(nxt)
	}

	return dfs(G, seen, 1, t, target)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 7
		edges := [][]int{{1, 2}, {1, 3}, {1, 7}, {2, 4}, {2, 6}, {3, 5}}
		t := 2
		target := 4
		ans := 0.16666666666666666
		assert(frogPosition(n, edges, t, target) == ans)
	}

	{
		n := 7
		edges := [][]int{{1, 2}, {1, 3}, {1, 7}, {2, 4}, {2, 6}, {3, 5}}
		t := 1
		target := 7
		ans := 0.3333333333333333
		assert(frogPosition(n, edges, t, target) == ans)
	}
}
