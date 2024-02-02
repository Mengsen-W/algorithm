/*
 * @Date: 2024-02-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-02
 * @FilePath: /algorithm/golang/1686_stone_game_vi/stone_game_vi.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func stoneGameVI(aliceValues []int, bobValues []int) int {
	n := len(aliceValues)
	values := make([][]int, n)
	for i := 0; i < n; i++ {
		values[i] = []int{aliceValues[i] + bobValues[i], aliceValues[i], bobValues[i]}
	}
	sort.Slice(values, func(i, j int) bool {
		return values[i][0] > values[j][0]
	})
	aliceSum, bobSum := 0, 0
	for i := 0; i < n; i++ {
		if i%2 == 0 {
			aliceSum += values[i][1]
		} else {
			bobSum += values[i][2]
		}
	}

	switch true {
	case aliceSum > bobSum:
		return 1
	case aliceSum == bobSum:
		return 0
	default:
		return -1
	}
}

func main() {
	tests := []struct {
		aliceValues []int
		bobValues   []int
		ans         int
	}{
		{[]int{1, 3}, []int{2, 1}, 1},
		{[]int{1, 2}, []int{3, 1}, 0},
		{[]int{2, 4, 3}, []int{1, 6, 7}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, stoneGameVI(test.aliceValues, test.bobValues), index)
	}
}
