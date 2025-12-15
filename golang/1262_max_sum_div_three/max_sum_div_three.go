// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSumDivThree(nums []int) int {
	f := []int{0, -0x3f3f3f3f, -0x3f3f3f3f}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for _, num := range nums {
		g := make([]int, 3)
		for i := 0; i < 3; i++ {
			g[(i+num)%3] = max(f[(i+num)%3], f[i]+num)
		}
		f = g
	}
	return f[0]
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 6, 5, 1, 8}, 18},
		{[]int{4}, 0},
		{[]int{1, 2, 3, 4, 4}, 12},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSumDivThree(test.nums), index)
	}
}
