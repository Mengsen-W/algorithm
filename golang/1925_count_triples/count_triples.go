// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countTriples(n int) int {
	res := 0
	// 枚举 a 与 b
	for a := 1; a <= n; a++ {
		for b := 1; b <= n; b++ {
			// 判断是否符合要求
			c := int(math.Sqrt(float64(a*a + b*b + 1)))
			if c <= n && c*c == a*a+b*b {
				res++
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{5, 2},
		{10, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countTriples(test.n), index)
	}
}
