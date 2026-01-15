// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximizeSquareHoleArea(n int, m int, hBars []int, vBars []int) int {
	sort.Ints(hBars)
	sort.Ints(vBars)
	hmax, vmax := 1, 1
	hcur, vcur := 1, 1
	for i := 1; i < len(hBars); i++ {
		if hBars[i] == hBars[i-1]+1 {
			hcur++
		} else {
			hcur = 1
		}
		hmax = max(hmax, hcur)
	}
	for i := 1; i < len(vBars); i++ {
		if vBars[i] == vBars[i-1]+1 {
			vcur++
		} else {
			vcur = 1
		}
		vmax = max(vmax, vcur)
	}
	side := min(hmax, vmax) + 1
	return side * side
}

func main() {
	tests := []struct {
		n     int
		m     int
		hBars []int
		vBars []int
		ans   int
	}{
		{2, 1, []int{2, 3}, []int{2}, 4},
		{1, 1, []int{2}, []int{2}, 4},
		{2, 3, []int{2, 3}, []int{2, 3, 4}, 9},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximizeSquareHoleArea(test.n, test.m, test.hBars, test.vBars), index)
	}
}
