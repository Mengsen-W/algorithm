// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func flowerGame(n int, m int) int64 {
	return int64(m) * int64(n) / 2
}

func main() {
	tests := []struct {
		n, m int
		ans  int64
	}{
		{3, 2, 3},
		{1, 1, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, flowerGame(test.n, test.m), index)
	}
}
