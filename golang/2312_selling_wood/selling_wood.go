/*
 * @Date: 2024-03-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-15
 * @FilePath: /algorithm/golang/2312_selling_wood/selling_wood.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sellingWood(m int, n int, prices [][]int) int64 {
	value := make(map[[2]int]int, 0)
	memo := make([][]int64, m+1)
	for i := range memo {
		memo[i] = make([]int64, n+1)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int64
	dfs = func(x, y int) int64 {
		if memo[x][y] != -1 {
			return memo[x][y]
		}

		var ret int64
		if val, ok := value[[2]int{x, y}]; ok {
			ret = int64(val)
		}
		if x > 1 {
			for i := 1; i < x; i++ {
				ret = max(ret, dfs(i, y)+dfs(x-i, y))
			}
		}
		if y > 1 {
			for j := 1; j < y; j++ {
				ret = max(ret, dfs(x, j)+dfs(x, y-j))
			}
		}
		memo[x][y] = ret
		return ret
	}

	for _, price := range prices {
		value[[2]int{price[0], price[1]}] = price[2]
	}
	return dfs(m, n)
}

func main() {
	tests := []struct {
		m      int
		n      int
		prices [][]int
		ans    int64
	}{
		{3, 5, [][]int{{1, 4, 2}, {2, 2, 7}, {2, 1, 3}}, 19},
		{4, 6, [][]int{{3, 2, 10}, {1, 4, 2}, {4, 1, 3}}, 32},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sellingWood(test.m, test.n, test.prices), index)
	}
}
