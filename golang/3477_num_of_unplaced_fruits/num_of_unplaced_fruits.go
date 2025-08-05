// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numOfUnplacedFruits(fruits []int, baskets []int) int {
	count := 0
	n := len(baskets)
	for _, fruit := range fruits {
		unset := 1
		for i := 0; i < n; i++ {
			if fruit <= baskets[i] {
				baskets[i] = 0
				unset = 0
				break
			}
		}
		count += unset
	}
	return count
}

func main() {
	tests := []struct {
		fruits  []int
		baskets []int
		ans     int
	}{
		{[]int{4, 2, 5}, []int{3, 5, 4}, 1},
		{[]int{3, 6, 1}, []int{6, 4, 7}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numOfUnplacedFruits(test.fruits, test.baskets), index)
	}
}
