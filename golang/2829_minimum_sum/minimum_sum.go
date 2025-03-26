// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumSum(n int, k int) int {
	arithmeticSeriesSum := func(a1 int, d int, n int) int {
		return (a1 + a1 + (n-1)*d) * n / 2
	}
	if n <= k/2 {
		return arithmeticSeriesSum(1, 1, n)
	} else {
		return arithmeticSeriesSum(1, 1, k/2) + arithmeticSeriesSum(k, 1, n-k/2)
	}
}

func main() {
	tests := []struct {
		n   int
		k   int
		ans int
	}{
		{5, 4, 18},
		{2, 6, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumSum(test.n, test.k), index)
	}
}
