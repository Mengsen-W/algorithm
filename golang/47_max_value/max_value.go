/*
 * @Date: 2023-03-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-08
 * @FilePath: /algorithm/golang/47_max_value/max_value.go
 */

package main

func maxValue(grid [][]int) int {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}

	m, n := len(grid), len(grid[0])
	f := make([][]int, 2)
	for i := range f {
		f[i] = make([]int, n)
	}
	for i, g := range grid {
		pos := i % 2
		for j, x := range g {
			f[pos][j] = 0
			if i > 0 {
				f[pos][j] = max(f[pos][j], f[1-pos][j])
			}
			if j > 0 {
				f[pos][j] = max(f[pos][j], f[pos][j-1])
			}
			f[pos][j] += x
		}
	}
	return f[(m-1)%2][n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	grid := [][]int{{1, 3, 1}, {1, 5, 1}, {4, 2, 1}}
	ans := 12
	assert(maxValue(grid) == ans)
}
