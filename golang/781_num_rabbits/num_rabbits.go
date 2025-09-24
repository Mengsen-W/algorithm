// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numRabbits(answers []int) (ans int) {
	count := map[int]int{}
	for _, y := range answers {
		count[y]++
	}
	for y, x := range count {
		ans += (x + y) / (y + 1) * (y + 1)
	}
	return
}

func main() {
	tests := []struct {
		answers []int
		ans     int
	}{
		{[]int{1, 1, 2}, 5},
		{[]int{10, 10, 10}, 11},
		{[]int{}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numRabbits(test.answers), index)
	}
}
