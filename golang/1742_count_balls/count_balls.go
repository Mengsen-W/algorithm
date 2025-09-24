// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countBalls(lowLimit, highLimit int) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	count := map[int]int{}
	for i := lowLimit; i <= highLimit; i++ {
		sum := 0
		for x := i; x > 0; x /= 10 {
			sum += x % 10
		}
		count[sum]++
		ans = max(ans, count[sum])
	}
	return
}

func main() {
	tests := []struct {
		lowLimit  int
		highLimit int
		ans       int
	}{
		{1, 10, 2},
		{5, 15, 2},
		{19, 28, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countBalls(test.lowLimit, test.highLimit), index)
	}
}
