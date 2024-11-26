// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfAlternatingGroups(colors []int) int {
	n := len(colors)
	res := 0
	for i := 0; i < n; i++ {
		if colors[i] != colors[(i-1+n)%n] && colors[i] != colors[(i+1)%n] {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		colors []int
		ans    int
	}{
		{[]int{1, 1, 1}, 0},
		{[]int{0, 1, 0, 0, 1}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfAlternatingGroups(test.colors), index)
	}
}
