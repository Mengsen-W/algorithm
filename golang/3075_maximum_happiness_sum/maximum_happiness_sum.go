// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumHappinessSum(happiness []int, k int) int64 {
	sort.Slice(happiness, func(i, j int) bool {
		return happiness[i] > happiness[j]
	})

	ans := int64(0)
	for i := 0; i < k; i++ {
		val := happiness[i] - i
		if val > 0 {
			ans += int64(val)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		happiness []int
		k         int
		ans       int64
	}{
		{[]int{1, 2, 3}, 2, 4},
		{[]int{1, 1, 1, 1}, 2, 1},
		{[]int{2, 3, 4, 5}, 1, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumHappinessSum(test.happiness, test.k), index)
	}
}
