// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numOfWays(n int) int {
	mod := 1000000007
	fi0, fi1 := 6, 6
	for i := 2; i <= n; i++ {
		newFi0 := (2*fi0 + 2*fi1) % mod
		newFi1 := (2*fi0 + 3*fi1) % mod
		fi0, fi1 = newFi0, newFi1
	}
	return (fi0 + fi1) % mod
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{1, 12},
		{2, 54},
		{3, 246},
		{7, 106494},
		{5000, 30228214},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numOfWays(test.n), index)
	}
}
