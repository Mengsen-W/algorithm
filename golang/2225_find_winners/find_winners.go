/*
 * @Date: 2024-05-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-22
 * @FilePath: /algorithm/golang/2225_find_winners/find_winners.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findWinners(matches [][]int) [][]int {
	freq := map[int]int{}
	for _, match := range matches {
		winner, loser := match[0], match[1]
		if freq[winner] == 0 {
			freq[winner] = 0
		}
		freq[loser]++
	}
	ans := make([][]int, 2)
	for key, value := range freq {
		if value < 2 {
			ans[value] = append(ans[value], key)
		}
	}
	sort.Ints(ans[0])
	sort.Ints(ans[1])
	return ans
}

func main() {
	tests := []struct {
		matches [][]int
		ans     [][]int
	}{
		{[][]int{{1, 3}, {2, 3}, {3, 6}, {5, 6}, {5, 7}, {4, 5}, {4, 8}, {4, 9}, {10, 4}, {10, 9}}, [][]int{{1, 2, 10}, {4, 5, 7, 8}}},
		{[][]int{{2, 3}, {1, 3}, {5, 4}, {6, 4}}, [][]int{{1, 2, 5, 6}, nil}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findWinners(test.matches), index)
	}
}
