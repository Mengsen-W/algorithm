/*
 * @Date: 2023-02-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-17
 * @FilePath: /algorithm/golang/1139_largest1_bordered_square/largest1_bordered_square.go
 */

package main

func largest1BorderedSquare(grid [][]int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	m, n := len(grid), len(grid[0])
	left := make([][]int, m+1)
	up := make([][]int, m+1)
	for i := range left {
		left[i] = make([]int, n+1)
		up[i] = make([]int, n+1)
	}
	maxBorder := 0
	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if grid[i-1][j-1] == 1 {
				left[i][j] = left[i][j-1] + 1
				up[i][j] = up[i-1][j] + 1
				border := min(left[i][j], up[i][j])
				for left[i-border+1][j] < border || up[i][j-border+1] < border {
					border--
				}
				maxBorder = max(maxBorder, border)
			}
		}
	}
	return maxBorder * maxBorder
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		grid := [][]int{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}
		ans := 2
		assert(largest1BorderedSquare(grid) == ans)
	}

	{
		grid := [][]int{{1, 1, 0, 0}}
		ans := 1
		assert(largest1BorderedSquare(grid) == ans)
	}
}
