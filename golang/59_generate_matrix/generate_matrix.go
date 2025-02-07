// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func generateMatrix(n int) [][]int {
	matrix := make([][]int, n)
	for i := range matrix {
		matrix[i] = make([]int, n)
	}
	num := 1
	left, right, top, bottom := 0, n-1, 0, n-1
	for left <= right && top <= bottom {
		for column := left; column <= right; column++ {
			matrix[top][column] = num
			num++
		}
		for row := top + 1; row <= bottom; row++ {
			matrix[row][right] = num
			num++
		}
		if left < right && top < bottom {
			for column := right - 1; column > left; column-- {
				matrix[bottom][column] = num
				num++
			}
			for row := bottom; row > top; row-- {
				matrix[row][left] = num
				num++
			}
		}
		left++
		right--
		top++
		bottom--
	}
	return matrix
}

func main() {
	tests := []struct {
		n   int
		ans [][]int
	}{
		{3, [][]int{{1, 2, 3}, {8, 9, 4}, {7, 6, 5}}},
		{1, [][]int{{1}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, generateMatrix(test.n), index)
	}
}
