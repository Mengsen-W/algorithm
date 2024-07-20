// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumMoves(grid [][]int) int {
	more, less := [][]int{}, [][]int{}
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if grid[i][j] > 1 {
				for k := 0; k < grid[i][j]-1; k++ {
					more = append(more, []int{i, j})
				}
			} else if grid[i][j] == 0 {
				less = append(less, []int{i, j})
			}
		}
	}

	ans := math.MaxInt32
	for {
		steps := 0
		for i := 0; i < len(more); i++ {
			steps += abs(less[i][0]-more[i][0]) + abs(less[i][1]-more[i][1])
		}
		if steps < ans {
			ans = steps
		}
		if !nextPermutation(more) {
			break
		}
	}
	return ans
}

func isLess(a, b []int) bool {
	return a[0] < b[0] || (a[0] == b[0] && a[1] < b[1])
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func nextPermutation(arr [][]int) bool {
	p := -1
	for i := 0; i < len(arr)-1; i++ {
		if isLess(arr[i], arr[i+1]) {
			p = i
		}
	}
	if p == -1 {
		return false
	}
	q := -1
	for j := p + 1; j < len(arr); j++ {
		if isLess(arr[p], arr[j]) {
			q = j
		}
	}
	arr[p], arr[q] = arr[q], arr[p]
	i, j := p+1, len(arr)-1
	for i < j {
		arr[i], arr[j] = arr[j], arr[i]
		i++
		j--
	}
	return true
}

func main() {
	tests := []struct {
		arr [][]int
		ans int
	}{
		{[][]int{{1, 1, 0}, {1, 1, 1}, {1, 2, 1}}, 3},
		{[][]int{{1, 3, 0}, {1, 0, 0}, {1, 0, 3}}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumMoves(test.arr), index)
	}
}
