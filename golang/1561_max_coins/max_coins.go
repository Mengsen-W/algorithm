// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxCoins(piles []int) int {
	sort.Ints(piles)
	length := len(piles)
	rounds := length / 3
	coins := 0
	index := length - 2
	for i := 0; i < rounds; i++ {
		coins += piles[index]
		index -= 2
	}
	return coins
}

func main() {
	tests := []struct {
		piles []int
		ans   int
	}{
		{[]int{2, 4, 1, 2, 7, 8}, 9},
		{[]int{2, 4, 5}, 4},
		{[]int{9, 8, 7, 6, 5, 1, 2, 3, 4}, 18},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxCoins(test.piles), index)
	}
}
