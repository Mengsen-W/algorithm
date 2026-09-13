// Package main ...
package main

import "fmt"

func largestOverlap(img1 [][]int, img2 [][]int) int {
	maxOverlap := 0
	n := len(img1)
	maxMove := n - 1
	for i := -maxMove; i <= maxMove; i++ {
		for j := -maxMove; j <= maxMove; j++ {
			overlap := 0
			rowStart, rowEnd := max(0, i), min(n, n+i)
			columnStart, columnEnd := max(0, j), min(n, n+j)
			for row := rowStart; row < rowEnd; row++ {
				for column := columnStart; column < columnEnd; column++ {
					if img1[row-i][column-j] == 1 && img2[row][column] == 1 {
						overlap++
					}
				}
			}
			maxOverlap = max(maxOverlap, overlap)
		}
	}
	return maxOverlap
}

func main() {
	tests := []struct {
		img1 [][]int
		img2 [][]int
		ans  int
	}{
		{[][]int{{1, 1, 0}, {0, 1, 0}, {0, 1, 0}}, [][]int{{0, 0, 0}, {0, 1, 1}, {0, 0, 1}}, 3},
		{[][]int{{1}}, [][]int{{1}}, 1},
		{[][]int{{0}}, [][]int{{0}}, 0},
	}

	for index, test := range tests {
		result := largestOverlap(test.img1, test.img2)
		if result != test.ans {
			fmt.Printf("Test %d failed: expected %d, got %d\n", index, test.ans, result)
		} else {
			fmt.Printf("Test %d passed\n", index)
		}
	}
}
