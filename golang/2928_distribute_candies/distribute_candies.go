// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func distributeCandies(n int, limit int) int {
	cal := func(x int) int {
		if x < 0 {
			return 0
		}
		return x * (x - 1) / 2
	}
	return cal(n+2) - 3*cal(n-limit+1) + 3*cal(n-(limit+1)*2+2) - cal(n-3*(limit+1)+2)
}

func main() {
	tests := []struct {
		n     int
		limit int
		ans   int
	}{
		{5, 2, 3},
		{3, 3, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distributeCandies(test.n, test.limit), index)
	}
}
