// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getGoodIndices(variables [][]int, target int) []int {
	powMod := func(x, y, mod int) int {
		res := 1
		for y > 0 {
			if (y & 1) == 1 {
				res = res * x % mod
			}
			x = x * x % mod
			y >>= 1
		}
		return res
	}

	ans := []int{}
	for i, v := range variables {
		if powMod(powMod(v[0], v[1], 10), v[2], v[3]) == target {
			ans = append(ans, i)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		variables [][]int
		target    int
		ans       []int
	}{
		{[][]int{{2, 3, 3, 10}, {3, 3, 3, 1}, {6, 1, 1, 4}}, 2, []int{0, 2}},
		{[][]int{{39, 3, 1000, 1000}}, 17, []int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getGoodIndices(test.variables, test.target), index)
	}
}
