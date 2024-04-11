/*
 * @Date: 2024-04-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-11
 * @FilePath: /algorithm/golang/1766_get_coprimes/get_coprimes.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getCoprimes(nums []int, edges [][]int) []int {
	n := len(nums)
	gcds := make([][]int, 51)
	tmp := make([][]int, 51)
	ans := make([]int, n)
	dep := make([]int, n)
	g := make([][]int, n)
	// 初始化
	for i := 0; i < 51; i++ {
		gcds[i] = []int{}
		tmp[i] = []int{}
	}
	for i := 0; i < n; i++ {
		g[i] = []int{}
		ans[i], dep[i] = -1, -1
	}

	var dfs func(x, depth int)
	dfs = func(x, depth int) {
		dep[x] = depth
		for _, val := range gcds[nums[x]] {
			if len(tmp[val]) == 0 {
				continue
			}
			las := tmp[val][len(tmp[val])-1]
			if ans[x] == -1 || dep[las] > dep[ans[x]] {
				ans[x] = las
			}
		}
		tmp[nums[x]] = append(tmp[nums[x]], x)
		for _, val := range g[x] {
			if dep[val] == -1 { // 被访问过的点dep不为-1
				dfs(val, depth+1)
			}
		}
		tmp[nums[x]] = tmp[nums[x]][:len(tmp[nums[x]])-1]
	}

	for i := 1; i <= 50; i++ {
		for j := 1; j <= 50; j++ {
			if gcd(i, j) == 1 {
				gcds[i] = append(gcds[i], j)
			}
		}
	}
	for _, edge := range edges {
		x := edge[0]
		y := edge[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	dfs(0, 1)
	return ans
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func main() {
	tests := []struct {
		nums  []int
		edges [][]int
		ans   []int
	}{
		{[]int{2, 3, 3, 2}, [][]int{{0, 1}, {1, 2}, {1, 3}}, []int{-1, 0, 0, 1}},
		{[]int{5, 6, 10, 2, 3, 6, 15}, [][]int{{0, 1}, {0, 2}, {1, 3}, {1, 4}, {2, 5}, {2, 6}}, []int{-1, 0, -1, 0, 0, 0, -1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getCoprimes(test.nums, test.edges), index)
	}
}
