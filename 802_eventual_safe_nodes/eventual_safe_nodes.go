/*
 * @Date: 2021-08-05 14:47:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-05 14:57:39
 * @FilePath: /algorithm/802_eventual_safe_nodes/eventual_safe_nodes.go
 * @Description: file content
 */

package main

import "reflect"

func eventualSafeNodes(graph [][]int) []int {
	return eventualSafeNodes_dfs(graph)
	// return eventualSafeNodes_topology(graph)
}

func eventualSafeNodes_dfs(graph [][]int) (ans []int) {
	n := len(graph)
	color := make([]int, n)
	var safe func(int) bool
	safe = func(x int) bool {
		if color[x] > 0 {
			return color[x] == 2
		}
		color[x] = 1
		for _, y := range graph[x] {
			if !safe(y) {
				return false
			}
		}
		color[x] = 2
		return true
	}
	for i := 0; i < n; i++ {
		if safe(i) {
			ans = append(ans, i)
		}
	}
	return
}

func eventualSafeNodes_topology(graph [][]int) (ans []int) {
	n := len(graph)
	rg := make([][]int, n)
	inDeg := make([]int, n)
	for x, ys := range graph {
		for _, y := range ys {
			rg[y] = append(rg[y], x)
		}
		inDeg[x] = len(ys)
	}

	q := []int{}
	for i, d := range inDeg {
		if d == 0 {
			q = append(q, i)
		}
	}
	for len(q) > 0 {
		y := q[0]
		q = q[1:]
		for _, x := range rg[y] {
			inDeg[x]--
			if inDeg[x] == 0 {
				q = append(q, x)
			}
		}
	}

	for i, d := range inDeg {
		if d == 0 {
			ans = append(ans, i)
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		graph := [][]int{{1, 2}, {2, 3}, {5}, {0}, {5}, {}, {}}
		ans := []int{2, 4, 5, 6}
		assert(eventualSafeNodes(graph), ans)
	}
	{
		graph := [][]int{{1, 2, 3, 4}, {1, 2}, {3, 4}, {0, 4}, {}}
		ans := []int{4}
		assert(eventualSafeNodes(graph), ans)
	}
}
