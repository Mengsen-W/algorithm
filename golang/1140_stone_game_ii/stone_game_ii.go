/*
 * @Date: 2023-02-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-22
 * @FilePath: /algorithm/golang/1140_stone_game_ii/stone_game_ii.go
 */

package main

func stoneGameII(piles []int) int {
	n := len(piles)
	s := make([]int, n+1)
	f := make([][]int, n+1)
	for i, x := range piles {
		s[i+1] = s[i] + x
		f[i] = make([]int, n+1)
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	var dfs func(i, m int) int
	dfs = func(i, m int) int {
		if m*2 >= n-i {
			return s[n] - s[i]
		}
		if f[i][m] > 0 {
			return f[i][m]
		}
		f[i][m] = 0
		for x := 1; x <= m<<1; x++ {
			f[i][m] = max(f[i][m], s[n]-s[i]-dfs(i+x, max(m, x)))
		}
		return f[i][m]
	}
	return dfs(0, 1)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		piles := []int{2, 7, 9, 4, 4}
		ans := 10
		assert(stoneGameII(piles) == ans)
	}

	{
		piles := []int{1, 2, 3, 4, 5, 100}
		ans := 104
		assert(stoneGameII(piles) == ans)
	}
}
