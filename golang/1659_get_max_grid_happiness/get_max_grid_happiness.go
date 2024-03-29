/*
 * @Date: 2023-06-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-24
 * @FilePath: /algorithm/golang/1659_get_max_grid_happiness/get_max_grid_happiness.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func getMaxGridHappiness(m int, n int, introvertsCount int, extrovertsCount int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	p := int(math.Pow(3, float64(n-1)))
	h := [3][3]int{{0, 0, 0}, {0, -60, -10}, {0, -10, 40}}
	memo := make([][][][]int, m*n)
	for i := range memo {
		memo[i] = make([][][]int, p*3)
		for j := range memo[i] {
			memo[i][j] = make([][]int, introvertsCount+1)
			for k := range memo[i][j] {
				memo[i][j][k] = make([]int, extrovertsCount+1)
				for l := range memo[i][j][k] {
					memo[i][j][k][l] = -1
				}
			}
		}
	}
	var dfs func(int, int, int, int) int
	dfs = func(pos, pre, ic, ec int) int {
		if pos == m*n || (ic == 0 && ec == 0) {
			return 0
		}
		if memo[pos][pre][ic][ec] != -1 {
			return memo[pos][pre][ic][ec]
		}
		ans := 0
		up := pre / p
		left := pre % 3
		if pos%n == 0 {
			left = 0
		}
		for i := 0; i < 3; i++ {
			if (i == 1 && ic == 0) || (i == 2 && ec == 0) {
				continue
			}
			cur := pre%p*3 + i
			nic, nec := ic, ec
			c := 0
			if i == 1 {
				nic--
				c = 120
			} else if i == 2 {
				nec--
				c = 40
			}
			a := h[up][i] + h[left][i]
			b := dfs(pos+1, cur, nic, nec)
			ans = max(ans, a+b+c)
		}
		memo[pos][pre][ic][ec] = ans
		return ans
	}
	return dfs(0, 0, introvertsCount, extrovertsCount)
}

func main() {
	{
		m := 2
		n := 3
		introvertsCount := 1
		extrovertsCount := 2
		ans := 240
		assert.Equal(&testing.B{}, getMaxGridHappiness(m, n, introvertsCount, extrovertsCount), ans)
	}

	{
		m := 3
		n := 1
		introvertsCount := 2
		extrovertsCount := 1
		ans := 260
		assert.Equal(&testing.B{}, getMaxGridHappiness(m, n, introvertsCount, extrovertsCount), ans)
	}

	{
		m := 2
		n := 2
		introvertsCount := 4
		extrovertsCount := 0
		ans := 240
		assert.Equal(&testing.B{}, getMaxGridHappiness(m, n, introvertsCount, extrovertsCount), ans)
	}
}
