// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countCoveredBuildings(n int, buildings [][]int) int {
	maxRow := make([]int, n+1)
	minRow := make([]int, n+1)
	maxCol := make([]int, n+1)
	minCol := make([]int, n+1)

	for i := range minRow {
		minRow[i] = n + 1
		minCol[i] = n + 1
	}

	for _, p := range buildings {
		x, y := p[0], p[1]
		maxRow[y] = max(maxRow[y], x)
		minRow[y] = min(minRow[y], x)
		maxCol[x] = max(maxCol[x], y)
		minCol[x] = min(minCol[x], y)
	}

	res := 0
	for _, p := range buildings {
		x, y := p[0], p[1]
		if x > minRow[y] && x < maxRow[y] &&
			y > minCol[x] && y < maxCol[x] {
			res++
		}
	}

	return res
}

func main() {
	tests := []struct {
		n         int
		buildings [][]int
		ans       int
	}{
		{3, [][]int{{1, 2}, {2, 2}, {3, 2}, {2, 1}, {2, 3}}, 1},
		{3, [][]int{{1, 1}, {1, 2}, {2, 1}, {2, 2}}, 0},
		{5, [][]int{{1, 3}, {3, 2}, {3, 3}, {3, 5}, {5, 3}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countCoveredBuildings(test.n, test.buildings), index)
	}
}
