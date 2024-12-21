// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func sortTheStudents(score [][]int, k int) [][]int {
	sort.Slice(score, func(i, j int) bool {
		return score[i][k] > score[j][k]
	})
	return score
}

func main() {
	tests := []struct {
		source [][]int
		k      int
		ans    [][]int
	}{
		{[][]int{{10, 6, 9, 1}, {7, 5, 11, 2}, {4, 8, 3, 15}}, 2, [][]int{{7, 5, 11, 2}, {10, 6, 9, 1}, {4, 8, 3, 15}}},
		{[][]int{{3, 4}, {5, 6}}, 0, [][]int{{5, 6}, {3, 4}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sortTheStudents(test.source, test.k), index)
	}
}
