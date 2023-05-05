/*
 * @Date: 2023-05-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-05
 * @FilePath: /algorithm/golang/2432_hardest_worker/hardest_worker.go
 */

// Package main ...
package main

func hardestWorker(n int, logs [][]int) int {
	ans, maxCost := logs[0][0], logs[0][1]
	for i := 1; i < len(logs); i++ {
		idx := logs[i][0]
		cost := logs[i][1] - logs[i-1][1]
		if cost > maxCost || (cost == maxCost && idx < ans) {
			ans = idx
			maxCost = cost
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 10
		logs := [][]int{{0, 3}, {2, 5}, {0, 9}, {1, 15}}
		ans := 1
		assert(hardestWorker(n, logs) == ans)
	}

	{
		n := 26
		logs := [][]int{{1, 1}, {3, 7}, {2, 12}, {7, 17}}
		ans := 3
		assert(hardestWorker(n, logs) == ans)
	}

	{
		n := 2
		logs := [][]int{{0, 10}, {1, 20}}
		ans := 0
		assert(hardestWorker(n, logs) == ans)
	}
}
