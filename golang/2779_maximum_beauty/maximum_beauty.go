// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumBeauty(nums []int, k int) int {
	m := 0
	for _, x := range nums {
		m = max(m, x)
	}
	diff := make([]int, m+2)
	for _, x := range nums {
		diff[max(x-k, 0)]++
		diff[min(x+k+1, m+1)]--
	}
	res, count := 0, 0
	for _, x := range diff {
		count += x
		res = max(res, count)
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{4, 6, 1, 2}, 2, 3},
		{[]int{1, 1, 1, 1}, 10, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumBeauty(test.nums, test.k), index)
	}
}
