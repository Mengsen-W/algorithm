/*
 * @Date: 2023-12-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-27
 * @FilePath: /algorithm/golang/2660_is_winner/is_winner.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isWinner(player1 []int, player2 []int) int {
	score := func(player []int) int {
		res := 0
		for i, x := range player {
			if i > 0 && player[i-1] == 10 || i > 1 && player[i-2] == 10 {
				res += 2 * x
			} else {
				res += x
			}
		}
		return res
	}
	s1, s2 := score(player1), score(player2)
	if s1 == s2 {
		return 0
	} else if s1 > s2 {
		return 1
	}
	return 2
}

func main() {
	tests := []struct {
		player1 []int
		player2 []int
		ans     int
	}{
		{[]int{4, 10, 7, 9}, []int{6, 5, 2, 3}, 1},
		{[]int{3, 5, 7, 6}, []int{8, 10, 10, 2}, 2},
		{[]int{2, 3}, []int{4, 1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isWinner(test.player1, test.player2), index)
	}
}
