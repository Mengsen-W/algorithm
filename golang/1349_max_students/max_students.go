/*
 * @Date: 2023-12-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-26
 * @FilePath: /algorithm/golang/1349_max_students/max_students.go
 */

// Package main ...
package main

import (
	"math"
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxStudents(seats [][]byte) int {
	m, n := len(seats), len(seats[0])
	memo := make(map[int]int)

	isSingleRowCompliant := func(status, row int) bool {
		n := len(seats[0])
		for j := 0; j < n; j++ {
			if (status>>j)&1 == 1 {
				if seats[row][j] == '#' {
					return false
				}
				if j > 0 && (status>>(j-1))&1 == 1 {
					return false
				}
			}
		}
		return true
	}

	isCrossRowsCompliant := func(status, upperRowStatus int) bool {
		n := len(seats[0])
		for j := 0; j < n; j++ {
			if (status>>j)&1 == 1 {
				if j > 0 && (upperRowStatus>>(j-1))&1 == 1 {
					return false
				}
				if j < n-1 && (upperRowStatus>>(j+1))&1 == 1 {
					return false
				}
			}
		}
		return true
	}

	var dp func(int, int) int
	dp = func(row, status int) int {
		n := len(seats[0])
		key := (row << n) + status
		if _, ok := memo[key]; !ok {
			if !isSingleRowCompliant(status, row) {
				memo[key] = math.MinInt32
				return math.MinInt32
			}
			students := bits.OnesCount(uint(status))
			if row == 0 {
				memo[key] = students
				return students
			}
			mx := 0
			for upperRowStatus := 0; upperRowStatus < 1<<n; upperRowStatus++ {
				if isCrossRowsCompliant(status, upperRowStatus) {
					mx = max(mx, dp(row-1, upperRowStatus))
				}
			}
			memo[key] = students + mx
		}
		return memo[key]
	}

	mx := 0
	for i := 0; i < (1 << n); i++ {
		mx = max(mx, dp(m-1, i))
	}
	return mx
}

func main() {
	tests := []struct {
		seats [][]byte
		ans   int
	}{
		{[][]byte{{'#', '.', '#', '#', '.', '#'}, {'.', '#', '#', '#', '#', '.'}, {'#', '.', '#', '#', '.', '#'}}, 4},
		{[][]byte{{'.', '#'}, {'#', '#'}, {'#', '.'}, {'#', '#'}, {'.', '#'}}, 3},
		{
			[][]byte{
				{'#', '.', '.', '.', '#'},
				{'.', '#', '.', '#', '.'},
				{'.', '.', '#', '.', '.'},
				{'.', '#', '.', '#', '.'},
				{'#', '.', '.', '.', '#'},
			},
			10,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxStudents(test.seats), index)
	}
}
