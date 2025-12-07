// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countOdds(low int, high int) int {
	pre := func(x int) int {
		return (x + 1) >> 1
	}
	return pre(high) - pre(low-1)
}

func main() {
	tests := []struct {
		low  int
		high int
		ans  int
	}{
		{3, 7, 3},
		{8, 10, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countOdds(test.low, test.high), index)
	}
}
