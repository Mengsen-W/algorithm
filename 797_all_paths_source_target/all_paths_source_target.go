/*
 * @Date: 2021-08-25 10:42:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-25 10:54:24
 */

package main

import "reflect"

func allPathsSourceTarget(graph [][]int) (ans [][]int) {
	stk := []int{0}
	var dfs func(int)
	dfs = func(x int) {
		if x == len(graph)-1 {
			ans = append(ans, append([]int(nil), stk...))
			return
		}
		for _, y := range graph[x] {
			stk = append(stk, y)
			dfs(y)
			stk = stk[:len(stk)-1]
		}
	}
	dfs(0)
	return
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		graph := [][]int{{1, 2}, {3}, {3}, {}}
		ans := [][]int{{0, 1, 3}, {0, 2, 3}}
		assert(allPathsSourceTarget(graph), ans)
	}
	{
		graph := [][]int{{4, 3, 1}, {3, 2, 4}, {3}, {4}, {}}
		ans := [][]int{
			{0, 4}, {0, 3, 4}, {0, 1, 3, 4}, {0, 1, 2, 3, 4}, {0, 1, 4}}
		assert(allPathsSourceTarget(graph), ans)
	}
	{
		graph := [][]int{{1}, {}}
		ans := [][]int{{0, 1}}
		assert(allPathsSourceTarget(graph), ans)
	}
	{
		graph := [][]int{{1, 2, 3}, {2}, {3}, {}}
		ans := [][]int{{0, 1, 2, 3}, {0, 2, 3}, {0, 3}}
		assert(allPathsSourceTarget(graph), ans)
	}
	{
		graph := [][]int{{1, 3}, {2}, {3}, {}}
		ans := [][]int{{0, 1, 2, 3}, {0, 3}}
		assert(allPathsSourceTarget(graph), ans)
	}
}
