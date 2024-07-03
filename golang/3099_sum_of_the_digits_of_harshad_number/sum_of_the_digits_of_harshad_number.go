// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumOfTheDigitsOfHarshadNumber(x int) int {
	s := 0
	for y := x; y != 0; y /= 10 {
		s += y % 10
	}

	if x%s != 0 {
		return -1
	}
	return s
}

func main() {
	tests := []struct {
		x   int
		ans int
	}{
		{18, 9},
		{23, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sumOfTheDigitsOfHarshadNumber(test.x), index)
	}
}
