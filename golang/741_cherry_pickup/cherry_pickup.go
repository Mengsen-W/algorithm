/*
 * @Date: 2022-07-10
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-10
 * @FilePath: /algorithm/741_cherry_pickup/cherry_pickup.go
 */

package main

import "math"

func cherryPickup(grid [][]int) int {
	n := len(grid)
	f := make([][]int, n)
	for i := range f {
		f[i] = make([]int, n)
		for j := range f[i] {
			f[i][j] = math.MinInt32
		}
	}
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	f[0][0] = grid[0][0]
	for k := 1; k < n*2-1; k++ {
		for x1 := min(k, n-1); x1 >= max(k-n+1, 0); x1-- {
			for x2 := min(k, n-1); x2 >= x1; x2-- {
				y1, y2 := k-x1, k-x2
				if grid[x1][y1] == -1 || grid[x2][y2] == -1 {
					f[x1][x2] = math.MinInt32
					continue
				}
				res := f[x1][x2] // 都往右
				if x1 > 0 {
					res = max(res, f[x1-1][x2]) // 往下，往右
				}
				if x2 > 0 {
					res = max(res, f[x1][x2-1]) // 往右，往下
				}
				if x1 > 0 && x2 > 0 {
					res = max(res, f[x1-1][x2-1]) // 都往下
				}
				res += grid[x1][y1]
				if x2 != x1 { // 避免重复摘同一个樱桃
					res += grid[x2][y2]
				}
				f[x1][x2] = res
			}
		}
	}
	return max(f[n-1][n-1], 0)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(cherryPickup([][]int{{0, 1, -1}, {1, 0, -1}, {1, 1, 1}}) == 5)
}
