/*
 * @Date: 2024-01-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-29
 * @FilePath: /algorithm/golang/514_find_rotate_steps/find_rotate_steps.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findRotateSteps(ring string, key string) int {
	min := func(a ...int) int {
		res := a[0]
		for _, v := range a[1:] {
			if v < res {
				res = v
			}
		}
		return res
	}

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	const inf = math.MaxInt64 / 2
	n, m := len(ring), len(key)
	pos := [26][]int{}
	for i, c := range ring {
		pos[c-'a'] = append(pos[c-'a'], i)
	}
	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, n)
		for j := range dp[i] {
			dp[i][j] = inf
		}
	}
	for _, p := range pos[key[0]-'a'] {
		dp[0][p] = min(p, n-p) + 1
	}
	for i := 1; i < m; i++ {
		for _, j := range pos[key[i]-'a'] {
			for _, k := range pos[key[i-1]-'a'] {
				dp[i][j] = min(dp[i][j], dp[i-1][k]+min(abs(j-k), n-abs(j-k))+1)
			}
		}
	}
	return min(dp[m-1]...)
}

func main() {
	tests := []struct {
		ring string
		key  string
		ans  int
	}{
		{"godding", "gd", 4},
		{"godding", "godding", 13},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findRotateSteps(test.ring, test.key), index)
	}
}
