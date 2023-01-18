/*
 * @Date: 2021-08-15 14:20:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-15 14:50:30
 */

package main

const mod int = 1e9 + 7

var dirs = []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}} // 上下左右

func findPaths(m, n, maxMove, startRow, startColumn int) (ans int) {
	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, n)
	}
	dp[startRow][startColumn] = 1
	for i := 0; i < maxMove; i++ {
		dpNew := make([][]int, m)
		for j := range dpNew {
			dpNew[j] = make([]int, n)
		}
		for j := 0; j < m; j++ {
			for k := 0; k < n; k++ {
				count := dp[j][k]
				if count > 0 {
					for _, dir := range dirs {
						j1, k1 := j+dir.x, k+dir.y
						if j1 >= 0 && j1 < m && k1 >= 0 && k1 < n {
							dpNew[j1][k1] = (dpNew[j1][k1] + count) % mod
						} else {
							ans = (ans + count) % mod
						}
					}
				}
			}
		}
		dp = dpNew
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		m := 2
		n := 2
		maxMove := 2
		startRow := 0
		startColumn := 0
		ans := 6
		assert(findPaths(m, n, maxMove, startRow, startColumn) == ans)
	}
	{
		m := 1
		n := 3
		maxMove := 3
		startRow := 0
		startColumn := 1
		ans := 12
		assert(findPaths(m, n, maxMove, startRow, startColumn) == ans)
	}
}
