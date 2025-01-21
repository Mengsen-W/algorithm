// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxValueOfCoins(piles [][]int, k int) int {
	f := make([]int, k+1)
	for i := range f {
		f[i] = -1
	}
	f[0] = 0
	for _, pile := range piles {
		for i := k; i > 0; i-- {
			value := 0
			for t := 1; t <= len(pile); t++ {
				value += pile[t-1]
				if i >= t && f[i-t] != -1 {
					f[i] = max(f[i], f[i-t]+value)
				}
			}
		}
	}
	return f[k]
}

func main() {
	tests := []struct {
		piles [][]int
		k     int
		ans   int
	}{
		{[][]int{{1, 100, 3}, {7, 8, 9}}, 2, 101},
		{[][]int{{100}, {100}, {100}, {100}, {100}, {100}, {1, 1, 1, 1, 1, 1, 700}}, 7, 706},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxValueOfCoins(test.piles, test.k), index)
	}
}
