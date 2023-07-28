/*
 * @Date: 2023-07-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-28
 * @FilePath: /algorithm/golang/2050_minimum_time/minimum_time.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumTime(n int, relations [][]int, time []int) int {
	res := 0
	prev := make([][]int, n+1)
	for i := 0; i <= n; i++ {
		prev[i] = make([]int, 0)
	}
	for _, relation := range relations {
		x := relation[0]
		y := relation[1]
		prev[y] = append(prev[y], x)
	}
	memo := make(map[int]int)
	for i := 1; i <= n; i++ {
		res = max(res, dp(i, time, prev, memo))
	}
	return res
}

func dp(i int, time []int, prev [][]int, memo map[int]int) int {
	if _, ok := memo[i]; !ok {
		cur := 0
		for _, p := range prev[i] {
			cur = max(cur, dp(p, time, prev, memo))
		}
		cur += time[i-1]
		memo[i] = cur
	}
	return memo[i]
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	tests := []struct {
		n         int
		relations [][]int
		time      []int
		ans       int
	}{
		{3, [][]int{{1, 3}, {2, 3}}, []int{2, 3, 5}, 8},
		{5, [][]int{{1, 5}, {2, 5}, {3, 5}, {3, 4}, {4, 5}}, []int{1, 2, 3, 4, 5}, 12},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, minimumTime(item.n, item.relations, item.time))
	}
}
