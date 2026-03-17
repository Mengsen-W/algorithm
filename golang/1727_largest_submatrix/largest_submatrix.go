// Package main ...
package main

import "sort"

func largestSubmatrix(matrix [][]int) int {
	m, n := len(matrix), len(matrix[0])
	maxArea := 0

	for i := 1; i < m; i++ {
		for j := 0; j < n; j++ {
			if matrix[i][j] == 1 {
				matrix[i][j] += matrix[i-1][j]
			}
		}
	}

	for i := 0; i < m; i++ {
		sort.Slice(matrix[i], func(a, b int) bool {
			return matrix[i][a] > matrix[i][b]
		})
		for j := 0; j < n; j++ {
			maxArea = max(maxArea, (j+1)*matrix[i][j])
		}
	}

	return maxArea
}

func main() {
	tests := []struct {
		matrix [][]int
		ans    int
	}{
		{[][]int{{0, 0, 1}, {1, 1, 1}, {1, 0, 1}}, 4},
		{[][]int{{1, 0, 1, 0, 1}}, 3},
		{[][]int{{1, 1, 0}, {1, 0, 1}}, 2},
		{[][]int{{0, 0}, {0, 0}}, 0},
	}

	for _, test := range tests {
		ans := largestSubmatrix(test.matrix)
		if ans != test.ans {
			panic("error")
		}
	}
}
