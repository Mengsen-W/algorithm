// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func totalNQueens(n int) (ans int) {
	var solve func(row, columns, diagonals1, diagonals2 int)
	solve = func(row, columns, diagonals1, diagonals2 int) {
		if row == n {
			ans++
			return
		}
		availablePositions := (1<<n - 1) &^ (columns | diagonals1 | diagonals2)
		for availablePositions > 0 {
			position := availablePositions & -availablePositions
			solve(row+1, columns|position, (diagonals1|position)<<1, (diagonals2|position)>>1)
			availablePositions &^= position // 移除该比特位
		}
	}
	solve(0, 0, 0, 0)
	return
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{4, 2},
		{1, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, totalNQueens(test.n), index)
	}
}
