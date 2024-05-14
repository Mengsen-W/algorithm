/*
 * @Date: 2024-05-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-14
 * @FilePath: /algorithm/golang/2244_minimum_rounds/minimum_rounds.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumRounds(tasks []int) int {
	cnt := map[int]int{}
	for _, t := range tasks {
		cnt[t]++
	}
	res := 0
	for _, v := range cnt {
		if v == 1 {
			return -1
		} else if v%3 == 0 {
			res += v / 3
		} else {
			res += v/3 + 1
		}
	}
	return res
}

func main() {
	tests := []struct {
		tasks []int
		ans   int
	}{
		{[]int{2, 2, 3, 3, 2, 4, 4, 4, 4, 4}, 4},
		{[]int{2, 3, 3}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumRounds(test.tasks), index)
	}
}
