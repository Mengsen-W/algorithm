// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countDays(days int, meetings [][]int) int {
	sort.Slice(meetings, func(i, j int) bool {
		return meetings[i][0] < meetings[j][0]
	})
	l, r := 1, 0
	for _, m := range meetings {
		if m[0] > r {
			days -= r - l + 1
			l = m[0]
		}
		if m[1] > r {
			r = m[1]
		}
	}
	days -= r - l + 1
	return days
}

func main() {
	tests := []struct {
		days     int
		meetings [][]int
		result   int
	}{
		{10, [][]int{{5, 7}, {1, 3}, {9, 10}}, 2},
		{5, [][]int{{2, 4}, {1, 3}}, 1},
		{6, [][]int{{1, 6}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.result, countDays(test.days, test.meetings), index)
	}
}
