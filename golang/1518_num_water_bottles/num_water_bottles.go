// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numWaterBottles(numBottles int, numExchange int) int {
	if numBottles < numExchange {
		return numBottles
	}
	return (numBottles-numExchange)/(numExchange-1) + 1 + numBottles
}

func main() {
	tests := []struct {
		numBottles  int
		numExchange int
		expected    int
	}{
		{9, 3, 13},
		{15, 4, 19},
		{5, 5, 6},
		{2, 3, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, numWaterBottles(test.numBottles, test.numExchange), "case %d", index)
	}
}
