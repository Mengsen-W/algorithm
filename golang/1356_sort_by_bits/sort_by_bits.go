// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

var bit = [1e4 + 1]int{}

func init() {
	for i := 1; i <= 1e4; i++ {
		bit[i] = bit[i>>1] + i&1
	}
}

func sortByBits(a []int) []int {
	sort.Slice(a, func(i, j int) bool {
		x, y := a[i], a[j]
		cx, cy := bit[x], bit[y]
		return cx < cy || cx == cy && x < y
	})
	return a
}

func main() {
	tests := []struct {
		arr []int
		ans []int
	}{
		{
			[]int{0, 1, 2, 3, 4, 5, 6, 7, 8},
			[]int{0, 1, 2, 4, 8, 3, 5, 6, 7},
		},
		{
			[]int{1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1},
			[]int{1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024},
		},
		{
			[]int{10000, 10000},
			[]int{10000, 10000},
		},
		{
			[]int{2, 3, 5, 7, 11, 13, 17, 19},
			[]int{2, 3, 5, 17, 7, 11, 13, 19},
		},
		{
			[]int{10, 100, 1000, 10000},
			[]int{10, 100, 10000, 1000},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sortByBits(test.ans), index)
	}
}
