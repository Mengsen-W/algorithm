// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxBottlesDrunk(numBottles int, numExchange int) int {
	ans := numBottles
	for empty := numBottles; empty >= numExchange; numExchange++ {
		ans++
		empty -= numExchange - 1
	}
	return ans
}

func main() {
	tests := []struct {
		numBottles  int
		numExchange int
		ans         int
	}{
		{13, 6, 15},
		{10, 3, 13},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxBottlesDrunk(test.numBottles, test.numExchange), index)
	}
}
