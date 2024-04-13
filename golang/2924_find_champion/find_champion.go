/*
 * @Date: 2024-04-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-13
 * @FilePath: /algorithm/golang/2924_find_champion/find_champion.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findChampion(n int, edges [][]int) int {
	degree := make([]int, n)
	for _, e := range edges {
		degree[e[1]]++
	}
	champion := -1
	for i, d := range degree {
		if d == 0 {
			if champion == -1 {
				champion = i
			} else {
				return -1
			}
		}
	}
	return champion
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		ans   int
	}{
		{3, [][]int{{0, 1}, {1, 2}}, 0},
		{4, [][]int{{0, 2}, {1, 3}, {1, 2}}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findChampion(test.n, test.edges), index)
	}
}
