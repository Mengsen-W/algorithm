/*
 * @Date: 2023-06-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-08
 * @FilePath: /algorithm/golang/1240_tiling_rectangle/tiling_rectangle.go
 */

// Package main ...
package main

func tilingRectangle(n int, m int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	ans := n * m
	filled := make([]int, n)
	var dfs func(i, j, t int)
	dfs = func(i, j, t int) {
		if j == m {
			i++
			j = 0
		}
		if i == n {
			ans = t
			return
		}
		if filled[i]>>j&1 == 1 {
			dfs(i, j+1, t)
		} else if t+1 < ans {
			var r, c int
			for k := i; k < n; k++ {
				if filled[k]>>j&1 == 1 {
					break
				}
				r++
			}
			for k := j; k < m; k++ {
				if filled[i]>>k&1 == 1 {
					break
				}
				c++
			}
			mx := min(r, c)
			for w := 1; w <= mx; w++ {
				for k := 0; k < w; k++ {
					filled[i+w-1] |= 1 << (j + k)
					filled[i+k] |= 1 << (j + w - 1)
				}
				dfs(i, j+w, t+1)
			}
			for x := i; x < i+mx; x++ {
				for y := j; y < j+mx; y++ {
					filled[x] ^= 1 << y
				}
			}
		}
	}
	dfs(0, 0, 0)
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 2
		m := 3
		ans := 3
		assert(tilingRectangle(n, m) == ans)
	}

	{
		n := 5
		m := 8
		ans := 5
		assert(tilingRectangle(n, m) == ans)
	}

	{
		n := 11
		m := 13
		ans := 6
		assert(tilingRectangle(n, m) == ans)
	}
}
