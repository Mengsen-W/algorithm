/*
 * @Date: 2021-12-19 01:00:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-19 01:18:44
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findJudge(n int, trust [][]int) int {
	inDegrees := make([]int, n+1)
	outDegrees := make([]int, n+1)
	for _, t := range trust {
		inDegrees[t[1]]++
		outDegrees[t[0]]++
	}
	for i := 1; i <= n; i++ {
		if inDegrees[i] == n-1 && outDegrees[i] == 0 {
			return i
		}
	}
	return -1
}

func main() {
	tests := []struct {
		n     int
		trust [][]int
		ans   int
	}{
		{2, [][]int{{1, 2}}, 2},
		{3, [][]int{{1, 3}, {2, 3}}, 3},
		{3, [][]int{{1, 3}, {2, 3}, {3, 1}}, -1},
		{3, [][]int{{1, 2}, {2, 3}}, -1},
		{4, [][]int{{1, 3}, {1, 4}, {2, 3}, {2, 4}, {4, 3}}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findJudge(test.n, test.trust), index)
	}
}
