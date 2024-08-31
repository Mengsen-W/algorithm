// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canMakeSquare(grid [][]byte) bool {
	for i := 0; i <= 1; i++ {
		for j := 0; j <= 1; j++ {
			if check(grid, i, j) {
				return true
			}
		}
	}
	return false
}

func check(grid [][]byte, x, y int) bool {
	count := 0
	for i := 0; i <= 1; i++ {
		for j := 0; j <= 1; j++ {
			if grid[x+i][y+j] == 'B' {
				count++
			}
		}
	}
	return count != 2
}

func main() {
	tests := []struct {
		grid [][]byte
		ans  bool
	}{
		{[][]byte{{'B', 'W', 'B'}, {'B', 'W', 'W'}, {'B', 'W', 'B'}}, true},
		{[][]byte{{'B', 'W', 'B'}, {'W', 'B', 'W'}, {'B', 'W', 'B'}}, false},
		{[][]byte{{'B', 'W', 'B'}, {'B', 'W', 'W'}, {'B', 'W', 'W'}}, true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canMakeSquare(test.grid), index)
	}
}
