/*
 * @Date: 2023-02-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-02
 * @FilePath: /algorithm/golang/1129_shortest_alternating_paths/shortest_alternating_paths.go
 */

package main

import "reflect"

func shortestAlternatingPaths(n int, redEdges, blueEdges [][]int) []int {
	type pair struct{ x, color int }
	g := make([][]pair, n)
	for _, e := range redEdges {
		g[e[0]] = append(g[e[0]], pair{e[1], 0})
	}
	for _, e := range blueEdges {
		g[e[0]] = append(g[e[0]], pair{e[1], 1})
	}

	dis := make([]int, n)
	for i := range dis {
		dis[i] = -1
	}
	vis := make([][2]bool, n)
	vis[0] = [2]bool{true, true}
	q := []pair{{0, 0}, {0, 1}}
	for level := 0; len(q) > 0; level++ {
		tmp := q
		q = nil
		for _, p := range tmp {
			x := p.x
			if dis[x] < 0 {
				dis[x] = level
			}
			for _, to := range g[x] {
				if to.color != p.color && !vis[to.x][to.color] {
					vis[to.x][to.color] = true
					q = append(q, to)
				}
			}
		}
	}
	return dis
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		n := 3
		redEdges := [][]int{{0, 1}, {1, 2}}
		blueEdges := [][]int{}
		ans := []int{0, 1, -1}
		assert(shortestAlternatingPaths(n, redEdges, blueEdges), ans)
	}

	{
		n := 3
		redEdges := [][]int{{0, 1}}
		blueEdges := [][]int{{2, 1}}
		ans := []int{0, 1, -1}
		assert(shortestAlternatingPaths(n, redEdges, blueEdges), ans)
	}

	{
		n := 3
		redEdges := [][]int{{1, 0}}
		blueEdges := [][]int{{2, 1}}
		ans := []int{0, -1, -1}
		assert(shortestAlternatingPaths(n, redEdges, blueEdges), ans)
	}

	{
		n := 3
		redEdges := [][]int{{0, 1}}
		blueEdges := [][]int{{1, 2}}
		ans := []int{0, 1, 2}
		assert(shortestAlternatingPaths(n, redEdges, blueEdges), ans)
	}

	{
		n := 3
		redEdges := [][]int{{0, 1}, {0, 2}}
		blueEdges := [][]int{{1, 0}}
		ans := []int{0, 1, 1}
		assert(shortestAlternatingPaths(n, redEdges, blueEdges), ans)
	}
}
