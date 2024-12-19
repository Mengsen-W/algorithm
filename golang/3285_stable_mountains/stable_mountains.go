// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func stableMountains(height []int, threshold int) []int {
	var result []int
	for i := 1; i < len(height); i++ {
		if height[i-1] > threshold {
			result = append(result, i)
		}
	}
	return result
}

func main() {
	tests := []struct {
		height    []int
		threshold int
		ans       []int
	}{
		{[]int{1, 2, 3, 4, 5}, 2, []int{3, 4}},
		{[]int{10, 1, 10, 1, 10}, 3, []int{1, 3}},
		{[]int{10, 1, 10, 1, 10}, 10, nil},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, stableMountains(test.height, test.threshold), index)
	}
}
