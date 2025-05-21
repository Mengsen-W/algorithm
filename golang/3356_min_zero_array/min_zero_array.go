// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minZeroArray(nums []int, queries [][]int) int {
	left, right := 0, len(queries)
	if !check(nums, queries, right) {
		return -1
	}
	for left < right {
		k := (left + right) / 2
		if check(nums, queries, k) {
			right = k
		} else {
			left = k + 1
		}
	}
	return left
}

func check(nums []int, queries [][]int, k int) bool {
	deltaArray := make([]int, len(nums)+1)
	for i := 0; i < k; i++ {
		left := queries[i][0]
		right := queries[i][1]
		value := queries[i][2]
		deltaArray[left] += value
		deltaArray[right+1] -= value
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
		result  int
	}{
		{[]int{2, 0, 2}, [][]int{{0, 2, 1}, {0, 2, 1}, {1, 1, 3}}, 2},
		{[]int{4, 3, 2, 1}, [][]int{{1, 3, 2}, {0, 2, 1}}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.result, minZeroArray(test.nums, test.queries), index)
	}
}
