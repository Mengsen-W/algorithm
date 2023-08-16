/*
 * @Date: 2023-08-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-16
 * @FilePath: /algorithm/golang/2682_circular_game_losers/circular_game_losers.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func circularGameLosers(n, k int) []int {
	visit := make([]bool, n)
	j := 0
	for i := k; !visit[j]; i += k {
		visit[j] = true
		j = (j + i) % n
	}
	ans := make([]int, 0, n)
	for i := 0; i < n; i++ {
		if !visit[i] {
			ans = append(ans, i+1)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		n   int
		k   int
		ans []int
	}{
		{5, 2, []int{4, 5}},
		{4, 4, []int{2, 3, 4}},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, circularGameLosers(item.n, item.k))
	}
}
