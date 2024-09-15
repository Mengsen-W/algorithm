// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfPoints(nums [][]int) int {
	C := 0
	for _, interval := range nums {
		if interval[1] > C {
			C = interval[1]
		}
	}

	count := make([]int, C+1)
	for _, interval := range nums {
		for i := interval[0]; i <= interval[1]; i++ {
			count[i]++
		}
	}

	ans := 0
	for i := 1; i <= C; i++ {
		if count[i] > 0 {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums [][]int
		ans  int
	}{
		{[][]int{{3, 6}, {1, 5}, {4, 7}}, 7},
		{[][]int{{1, 3}, {5, 8}}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfPoints(test.nums), index)
	}
}
