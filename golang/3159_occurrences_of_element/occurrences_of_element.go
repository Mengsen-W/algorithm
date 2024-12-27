// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func occurrencesOfElement(nums []int, queries []int, x int) []int {
	indices := []int{}
	for i, num := range nums {
		if num == x {
			indices = append(indices, i)
		}
	}
	res := []int{}
	for _, q := range queries {
		if len(indices) < q {
			res = append(res, -1)
		} else {
			res = append(res, indices[q-1])
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums    []int
		queries []int
		x       int
		ans     []int
	}{
		{[]int{1, 3, 1, 7}, []int{1, 3, 2, 4}, 1, []int{0, -1, 2, -1}},
		{[]int{1, 2, 3}, []int{10}, 5, []int{-1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, occurrencesOfElement(test.nums, test.queries, test.x), index)
	}
}
