/*
 * @Date: 2024-05-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-26
 * @FilePath: /algorithm/golang/find_indices.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findIndices(nums []int, indexDifference int, valueDifference int) []int {
	minIndex, maxIndex := 0, 0
	for j := indexDifference; j < len(nums); j++ {
		i := j - indexDifference
		if nums[i] < nums[minIndex] {
			minIndex = i
		}
		if nums[j]-nums[minIndex] >= valueDifference {
			return []int{minIndex, j}
		}
		if nums[i] > nums[maxIndex] {
			maxIndex = i
		}
		if nums[maxIndex]-nums[j] >= valueDifference {
			return []int{maxIndex, j}
		}
	}
	return []int{-1, -1}
}

func main() {
	tests := []struct {
		nums            []int
		indexDifference int
		valueDifference int
		ans             []int
	}{
		{[]int{5, 1, 4, 1}, 2, 4, []int{0, 3}},
		{[]int{2, 1}, 0, 0, []int{0, 0}},
		{[]int{1, 2, 3}, 2, 4, []int{-1, -1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findIndices(test.nums, test.indexDifference, test.valueDifference), index)
	}
}
