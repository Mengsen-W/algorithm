// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func differenceOfSums(n int, m int) int {
	k := n / m
	return n*(n+1)/2 - k*(k+1)*m
}

func main() {
	tests := []struct {
		n   int
		m   int
		ans int
	}{
		{10, 3, 19},
		{5, 6, 15},
		{5, 1, -15},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, differenceOfSums(test.n, test.m), "Test Case #%d", index+1)
	}
}
