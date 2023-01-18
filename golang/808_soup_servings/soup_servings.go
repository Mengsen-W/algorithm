/*
 * @Date: 2022-11-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-21
 * @FilePath: /algorithm/808_soup_servings/soup_servings.go
 */

package main

func soupServings(n int) float64 {
	n = (n + 24) / 25
	if n >= 179 {
		return 1
	}
	dp := make([][]float64, n+1)
	for i := range dp {
		dp[i] = make([]float64, n+1)
	}
	var dfs func(int, int) float64
	dfs = func(a, b int) float64 {
		if a <= 0 && b <= 0 {
			return 0.5
		}
		if a <= 0 {
			return 1
		}
		if b <= 0 {
			return 0
		}
		dv := &dp[a][b]
		if *dv > 0 {
			return *dv
		}
		res := (dfs(a-4, b) + dfs(a-3, b-1) +
			dfs(a-2, b-2) + dfs(a-1, b-3)) / 4
		*dv = res
		return res
	}
	return dfs(n, n)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(soupServings(50) == 0.62500)
	assert(soupServings(100) == 0.71875)
}
