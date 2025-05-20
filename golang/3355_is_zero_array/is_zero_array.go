// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isZeroArray(nums []int, queries [][]int) bool {
	deltaArray := make([]int, len(nums)+1)
	for _, query := range queries {
		left := query[0]
		right := query[1]
		deltaArray[left]++
		deltaArray[right+1]--
	}
	operationCounts := make([]int, len(deltaArray))
	currentOperations := 0
	for i, delta := range deltaArray {
		currentOperations += delta
		operationCounts[i] = currentOperations
	}
	for i := 0; i < len(nums); i++ {
		if operationCounts[i] < nums[i] {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		nums    []int
		queries [][]int
		ans     bool
	}{
		{[]int{1, 0, 1}, [][]int{{0, 2}}, true},
		{[]int{4, 3, 2, 1}, [][]int{{1, 3}, {0, 2}}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isZeroArray(test.nums, test.queries), index)
	}
}
