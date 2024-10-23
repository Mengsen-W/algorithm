// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countCompleteDayPairs(hours []int) int {
	ans := 0
	for i := 1; i < len(hours); i++ {
		for j := 0; j < i; j++ {
			if (hours[i]+hours[j])%24 == 0 {
				ans++
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		hours []int
		ans   int
	}{
		{[]int{12, 12, 30, 24, 24}, 2},
		{[]int{72, 48, 24, 3}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countCompleteDayPairs(test.hours), index)
	}
}
