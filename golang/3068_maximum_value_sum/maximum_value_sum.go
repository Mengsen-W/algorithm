// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumValueSum(nums []int, k int, edges [][]int) int64 {
	var f0, f1 int64
	f0, f1 = int64(0), math.MinInt64
	for _, x := range nums {
		f0, f1 = max(f0+int64(x), f1+int64(x^k)), max(f1+int64(x), f0+int64(x^k))
	}
	return f0
}

func main() {
	tests := []struct {
		nums  []int
		k     int
		edges [][]int
		ans   int64
	}{
		{[]int{1, 2, 1}, 3, [][]int{{0, 1}, {0, 2}}, 6},
		{[]int{2, 3}, 7, [][]int{{0, 1}}, 9},
		{[]int{7, 7, 7, 7, 7, 7}, 3, [][]int{{0, 1}, {0, 2}, {0, 3}, {0, 4}, {0, 5}}, 42},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumValueSum(test.nums, test.k, test.edges), "case %d", index)
	}
}
