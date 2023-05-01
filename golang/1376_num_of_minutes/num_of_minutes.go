/*
 * @Date: 2023-05-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-01
 * @FilePath: /algorithm/golang/1376_num_of_minutes/num_of_minutes.go
 */

// Package main ...
package main

func numOfMinutes(n int, headID int, manager []int, informTime []int) int {
	memo := map[int]int{}
	var dfs func(int) int
	dfs = func(cur int) int {
		if cur == headID {
			return 0
		}
		if value, ok := memo[cur]; ok {
			return value
		}
		res := dfs(manager[cur]) + informTime[manager[cur]]
		memo[cur] = res
		return res
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	res := 0
	for i := 0; i < n; i++ {
		res = max(res, dfs(i))
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 1
		headID := 0
		manager := []int{-1}
		informTime := []int{0}
		ans := 0
		assert(numOfMinutes(n, headID, manager, informTime) == ans)
	}

	{
		n := 5
		headID := 2
		manager := []int{2, 2, -1, 2, 2, 2}
		informTime := []int{0, 0, 1, 0, 0, 0}
		ans := 1
		assert(numOfMinutes(n, headID, manager, informTime) == ans)
	}
}
