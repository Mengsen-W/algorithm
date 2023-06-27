/*
 * @Date: 2023-06-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-22
 * @FilePath: /algorithm/golang/16.19_pond_sizes/pond_sizes.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func pondSizes(land [][]int) []int {
	m, n := len(land), len(land[0])
	bfs := func(x, y int) int {
		q, res := [][]int{}, 0
		q, land[x][y] = append(q, []int{x, y}), -1
		for len(q) > 0 {
			x, y, q = q[0][0], q[0][1], q[1:]
			res++
			for dx := -1; dx <= 1; dx++ {
				for dy := -1; dy <= 1; dy++ {
					if dx == 0 && dy == 0 {
						continue
					}
					if x+dx < 0 || x+dx >= m || y+dy < 0 || y+dy >= n || land[x+dx][y+dy] != 0 {
						continue
					}
					land[x+dx][y+dy] = -1
					q = append(q, []int{x + dx, y + dy})
				}
			}
		}
		return res
	}
	res := []int{}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if land[i][j] == 0 {
				res = append(res, bfs(i, j))
			}
		}
	}
	sort.Ints(res)
	return res
}

func main() {
	land := [][]int{{0, 2, 1, 0}, {0, 1, 0, 1}, {1, 1, 0, 1}, {0, 1, 0, 1}}
	ans := []int{1, 2, 4}
	assert.Equal(&testing.B{}, pondSizes(land), ans)
}
