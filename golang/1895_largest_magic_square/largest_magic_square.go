// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func largestMagicSquare(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	// 每一行的前缀和
	rowsum := make([][]int, m)
	for i := 0; i < m; i++ {
		rowsum[i] = make([]int, n)
		rowsum[i][0] = grid[i][0]
		for j := 1; j < n; j++ {
			rowsum[i][j] = rowsum[i][j-1] + grid[i][j]
		}
	}
	// 每一列的前缀和
	colsum := make([][]int, m)
	for i := 0; i < m; i++ {
		colsum[i] = make([]int, n)
	}
	for j := 0; j < n; j++ {
		colsum[0][j] = grid[0][j]
		for i := 1; i < m; i++ {
			colsum[i][j] = colsum[i-1][j] + grid[i][j]
		}
	}

	for edge := min(m, n); edge >= 2; edge-- {
		// 枚举正方形的左上角位置 (i,j)
		for i := 0; i+edge <= m; i++ {
			for j := 0; j+edge <= n; j++ {
				// 计算标准值
				stdsum := rowsum[i][j+edge-1]
				if j > 0 {
					stdsum -= rowsum[i][j-1]
				}
				check := true
				// 检查每一行
				for ii := i + 1; ii < i+edge; ii++ {
					sum := rowsum[ii][j+edge-1]
					if j > 0 {
						sum -= rowsum[ii][j-1]
					}
					if sum != stdsum {
						check = false
						break
					}
				}
				if !check {
					continue
				}
				// 检查每一列
				for jj := j; jj < j+edge; jj++ {
					sum := colsum[i+edge-1][jj]
					if i > 0 {
						sum -= colsum[i-1][jj]
					}
					if sum != stdsum {
						check = false
						break
					}
				}
				if !check {
					continue
				}
				// 检查对角线
				d1, d2 := 0, 0
				for k := 0; k < edge; k++ {
					d1 += grid[i+k][j+k]
					d2 += grid[i+k][j+edge-1-k]
				}
				if d1 == stdsum && d2 == stdsum {
					return edge
				}
			}
		}
	}
	return 1
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{7, 1, 4, 5, 6}, {2, 5, 1, 6, 4}, {1, 5, 4, 3, 2}, {1, 2, 7, 3, 4}}, 3},
		{[][]int{{5, 1, 3, 1}, {9, 3, 3, 1}, {1, 3, 3, 8}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, largestMagicSquare(test.grid), index)
	}
}
