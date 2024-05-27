/*
 * @Date: 2022-03-27 02:45:06
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-27
 * @FilePath: /algorithm/golang/2028_missing_rolls/missing_rolls.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func missingRolls(rolls []int, mean, n int) []int {
	missingSum := mean * (n + len(rolls))
	for _, roll := range rolls {
		missingSum -= roll
	}
	if missingSum < n || missingSum > n*6 {
		return nil
	}

	quotient, remainder := missingSum/n, missingSum%n
	ans := make([]int, n)
	for i := range ans {
		ans[i] = quotient
		if i < remainder {
			ans[i]++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		rolls []int
		mean  int
		n     int
		ans   []int
	}{
		{[]int{3, 2, 4, 3}, 4, 2, []int{6, 6}},
		{[]int{1, 5, 6}, 3, 4, []int{3, 2, 2, 2}},
		{[]int{1, 2, 3, 4}, 5, 4, nil},
		{[]int{1}, 3, 1, []int{5}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, missingRolls(test.rolls, test.mean, test.n), index)
	}
}
