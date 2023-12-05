/*
 * @Date: 2023-12-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-05
 * @FilePath: /algorithm/golang/2477_minimum_fuel_cost/minimum_fuel_cost.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumFuelCost(roads [][]int, seats int) int64 {
	g := make([][]int, len(roads)+1)
	for _, e := range roads {
		g[e[0]] = append(g[e[0]], e[1])
		g[e[1]] = append(g[e[1]], e[0])
	}
	var res int64
	var dfs func(int, int) int
	dfs = func(cur, fa int) int {
		peopleSum := 1
		for _, ne := range g[cur] {
			if ne != fa {
				peopleCnt := dfs(ne, cur)
				peopleSum, res = peopleSum+peopleCnt, res+int64((peopleCnt+seats-1)/seats)
			}
		}
		return peopleSum
	}
	dfs(0, -1)
	return res
}

func main() {
	tests := []struct {
		roads [][]int
		seats int
		ans   int64
	}{
		{[][]int{{0, 1}, {0, 2}, {0, 3}}, 5, 3},
		{[][]int{{3, 1}, {3, 2}, {1, 0}, {0, 4}, {0, 5}, {4, 6}}, 2, 7},
		{[][]int{}, 1, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumFuelCost(test.roads, test.seats), index)
	}
}
