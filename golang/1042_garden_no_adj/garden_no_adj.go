/*
 * @Date: 2023-04-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-15
 * @FilePath: /algorithm/golang/1042_garden_no_adj/garden_no_adj.go
 */

// Package main ...
package main

import "reflect"

func gardenNoAdj(n int, paths [][]int) []int {
	adj := make([][]int, n)
	for i := 0; i < n; i++ {
		adj[i] = []int{}
	}
	for _, path := range paths {
		adj[path[0]-1] = append(adj[path[0]-1], path[1]-1)
		adj[path[1]-1] = append(adj[path[1]-1], path[0]-1)
	}
	ans := make([]int, n)
	for i := 0; i < n; i++ {
		colored := make([]bool, 5)
		for _, vertex := range adj[i] {
			colored[ans[vertex]] = true
		}
		for j := 1; j <= 4; j++ {
			if !colored[j] {
				ans[i] = j
				break
			}
		}
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		n := 3
		paths := [][]int{{1, 2}, {2, 3}, {3, 1}}
		ans := []int{1, 2, 3}
		assert(gardenNoAdj(n, paths), ans)
	}

	{
		n := 4
		paths := [][]int{{1, 2}, {3, 4}}
		ans := []int{1, 2, 1, 2}
		assert(gardenNoAdj(n, paths), ans)
	}

	{
		n := 4
		paths := [][]int{{1, 2}, {2, 3}, {3, 4}, {4, 1}, {1, 3}, {2, 4}}
		ans := []int{1, 2, 3, 4}
		assert(gardenNoAdj(n, paths), ans)
	}
}
